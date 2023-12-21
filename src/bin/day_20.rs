use std::{
    collections::{HashMap, VecDeque},
    io,
};

use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let mut circuit = read_input()?;
    for _ in 0..1000 {
        circuit.push_button();
    }
    let ans = circuit.pending.low_times_high();
    dbg!(ans);
    Ok(())
}

impl Circuit {
    fn push_button(&mut self) {
        self.pending
            .enqueue(Pulse::new(Bit::Low, "button", "broadcaster"));

        let _ = self.pending.dequeue().unwrap();
        for rx in &self.broadcaster.outputs {
            self.pending
                .enqueue(Pulse::new(Bit::Low, "broadcaster", rx));
        }

        while let Some(pulse) = self.pending.dequeue() {
            let label = &pulse.receiver;
            if label == "rx" {
                // Ignore the non-existent receiver, "rx".
                continue;
            }
            let gate = self.gates.get_mut(label).unwrap();

            let bit = match &mut gate.logic {
                Logic::FlipFlop(f) => match pulse.bit {
                    Bit::Low => {
                        let out = f.prev_output.toggle();
                        f.prev_output = out;
                        out
                    }
                    // Skip output.
                    Bit::High => continue,
                },
                Logic::Nand(n) => {
                    n.prev_inputs.insert(pulse.sender.clone(), pulse.bit);
                    if n.prev_inputs.values().all(|b| matches!(b, Bit::High)) {
                        Bit::Low
                    } else {
                        Bit::High
                    }
                }
            };

            for rx in &gate.outputs {
                self.pending.enqueue(Pulse::new(bit, label, rx));
            }
        }
    }
}

impl PendingMessages {
    fn enqueue(&mut self, msg: Pulse) {
        match msg.bit {
            Bit::Low => self.low_pulses += 1,
            Bit::High => self.high_pulses += 1,
        }
        self.buf.push_back(msg);
    }

    fn dequeue(&mut self) -> Option<Pulse> {
        self.buf.pop_front()
    }

    fn low_times_high(&self) -> u64 {
        assert!(self.buf.is_empty());
        self.low_pulses as u64 * self.high_pulses as u64
    }
}

impl Pulse {
    fn new(bit: Bit, sender: impl Into<String>, receiver: impl Into<String>) -> Self {
        Self {
            bit,
            sender: sender.into(),
            receiver: receiver.into(),
        }
    }
}

impl Bit {
    #[must_use]
    fn toggle(self) -> Self {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

struct Circuit {
    broadcaster: Broadcaster,
    gates: HashMap<String, Gate>,
    pending: PendingMessages,
}

struct Broadcaster {
    outputs: Vec<String>,
}

struct Gate {
    outputs: Vec<String>,
    logic: Logic,
}

enum Logic {
    FlipFlop(FlipFlop),
    Nand(Nand),
}

struct FlipFlop {
    prev_output: Bit,
}

struct Nand {
    prev_inputs: HashMap<String, Bit>,
}

#[derive(Default)]
struct PendingMessages {
    buf: VecDeque<Pulse>,
    low_pulses: usize,
    high_pulses: usize,
}

struct Pulse {
    bit: Bit,
    sender: String,
    receiver: String,
}

#[derive(Clone, Copy)]
enum Bit {
    Low,
    High,
}

fn read_input() -> Result<Circuit> {
    let lines: Vec<_> = io::stdin().lines().try_collect()?;

    let mut broadcaster = Broadcaster { outputs: vec![] };
    let mut gates = HashMap::new();

    for l in lines {
        let (gate, outputs) = l.split_once(" -> ").context("arrow")?;
        let outputs = outputs.split(", ").map(str::to_owned).collect();

        if gate == "broadcaster" {
            ensure!(broadcaster.outputs.is_empty(), "more than one broadcaster");
            broadcaster.outputs = outputs;
            continue;
        }

        let logic = match gate.chars().next().context("logic symbol")? {
            '%' => Logic::FlipFlop(FlipFlop {
                prev_output: Bit::Low,
            }),
            '&' => Logic::Nand(Nand {
                // Initialized below.
                prev_inputs: HashMap::new(),
            }),
            c => bail!("invalid logic symbol {c:?}"),
        };
        let label = gate[1..].to_owned();

        gates.insert(label, Gate { outputs, logic });
    }

    // Initialize Nand.prev_inputs.
    let labels: Vec<_> = gates.keys().cloned().collect();
    for input in labels {
        for output in gates[&input].outputs.clone() {
            // Ignore the non-existent receiver, "rx".
            if output == "rx" {
                continue;
            }

            match &mut gates.get_mut(&output).unwrap().logic {
                Logic::Nand(nand) => {
                    nand.prev_inputs.insert(input.clone(), Bit::Low);
                }
                Logic::FlipFlop(_) => (),
            }
        }
    }

    Ok(Circuit {
        broadcaster,
        gates,
        pending: PendingMessages::default(),
    })
}

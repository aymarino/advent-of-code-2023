use std::collections::{hash_map::Entry, HashMap, VecDeque};

use itertools::Itertools;
use num::Integer;

struct ConjunctionState<'a> {
    memory: HashMap<&'a str, Pulse>,
}

enum ModuleType<'a> {
    Broadcast,
    FlipFlop(bool),
    Conjunction(ConjunctionState<'a>),
}

struct Module<'a> {
    mod_type: ModuleType<'a>,
    outputs: Vec<&'a str>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/20.txt");

    let mut mod_inputs = HashMap::<&str, Vec<&str>>::new();
    let mut modules = input
        .lines()
        .map(|line| {
            let (name, outputs) = line.split_once(" -> ").unwrap();
            let (mod_type, name) = name.split_at(1);
            let (mod_type, name) = {
                if mod_type == "b" {
                    (ModuleType::Broadcast, "broadcast")
                } else {
                    let mod_type = match mod_type {
                        "%" => ModuleType::FlipFlop(false),
                        "&" => ModuleType::Conjunction(ConjunctionState {
                            memory: HashMap::new(),
                        }),
                        _ => panic!("Invalid mod type: {}", mod_type),
                    };
                    (mod_type, name)
                }
            };
            let outputs = outputs.split(", ").collect_vec();
            outputs
                .iter()
                .for_each(|&output| match mod_inputs.entry(output) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(name);
                    }
                    Entry::Vacant(e) => {
                        e.insert(Vec::from([name]));
                    }
                });
            (name, Module { mod_type, outputs })
        })
        .collect::<HashMap<&str, Module>>();

    for (module_name, inputs) in &mod_inputs {
        if let Some(module) = modules.get_mut(module_name) {
            if let ModuleType::Conjunction(state) = &mut module.mod_type {
                for i in inputs {
                    state.memory.insert(i, Pulse::Low);
                }
            }
        }
    }

    let rx_inputs = mod_inputs.get("rx").unwrap();
    assert!(rx_inputs.len() == 1);
    let rx_rx_inputs = mod_inputs.get(rx_inputs.last().unwrap()).unwrap();
    // Maps the input to `rx`'s inputs to the number of button presses it takes to
    // first receive a High pulse.
    let mut rx_rx_input_button_presses =
        HashMap::<&str, Option<u64>>::from_iter(rx_rx_inputs.iter().map(|&input| (input, None)));

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut p1 = 0;
    let mut num_button_presses = 0;
    'outer: loop {
        if num_button_presses == 1000 {
            p1 = low_pulses * high_pulses;
        }

        num_button_presses += 1;

        struct PulseEvent<'a> {
            pulse: Pulse,
            source: &'a str,
            destination: &'a str,
        }
        let mut pulses = VecDeque::from([PulseEvent {
            pulse: Pulse::Low,
            source: "button",
            destination: "broadcast",
        }]);
        while let Some(event) = pulses.pop_front() {
            match event.pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }

            if !modules.contains_key(event.destination) {
                continue;
            }

            let target = modules.get_mut(event.destination).unwrap();
            match &mut target.mod_type {
                ModuleType::Broadcast => {
                    // Sends the same pulse to all outputs
                    target.outputs.iter().for_each(|output| {
                        pulses.push_back(PulseEvent {
                            pulse: event.pulse,
                            source: event.destination,
                            destination: output,
                        });
                    });
                }
                ModuleType::FlipFlop(on) => {
                    let pulse = match event.pulse {
                        Pulse::High => None,
                        Pulse::Low => {
                            let pulse = if *on { Pulse::Low } else { Pulse::High };
                            *on = !*on;
                            Some(pulse)
                        }
                    };
                    if let Some(pulse) = pulse {
                        target.outputs.iter().for_each(|output| {
                            pulses.push_back(PulseEvent {
                                pulse,
                                source: event.destination,
                                destination: output,
                            });
                        });
                    }
                }
                ModuleType::Conjunction(state) => {
                    if event.pulse == Pulse::High && rx_rx_inputs.contains(&event.source) {
                        let p = rx_rx_input_button_presses.get_mut(&event.source).unwrap();
                        *p = Some(num_button_presses);
                        if rx_rx_input_button_presses.values().all(|&v| v.is_some()) {
                            break 'outer;
                        }
                    }
                    *state.memory.get_mut(event.source).unwrap() = event.pulse;
                    let pulse = if state.memory.values().all(|&p| p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    target.outputs.iter().for_each(|output| {
                        pulses.push_back(PulseEvent {
                            pulse,
                            source: event.destination,
                            destination: output,
                        });
                    });
                }
            }
        }
    }

    // Assumes that High pulses to the input's to `rx`'s inputs occur
    // on even cycles starting at button press == 0.
    let p2 = rx_rx_input_button_presses
        .values()
        .fold(1, |acc, v| acc.lcm(&v.unwrap()));

    (p1, p2)
}

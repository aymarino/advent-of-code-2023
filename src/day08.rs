use std::collections::HashMap;

use num::Integer;
use regex::Regex;

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/8.txt");
    let (steps, map) = input.split_once("\n\n").unwrap();
    let steps = steps.as_bytes();

    let network: HashMap<&[u8], Vec<&[u8]>> = map
        .lines()
        .map(|s| {
            let (from_node, to_nodes) = s.split_once(" = ").unwrap();
            let groups = Regex::new(r"\(([a-zA-Z0-9]+), ([a-zA-Z0-9]+)\)")
                .unwrap()
                .captures(to_nodes)
                .unwrap();
            let (left, right) = (
                groups.get(1).unwrap().as_str(),
                groups.get(2).unwrap().as_str(),
            );
            (
                from_node.as_bytes(),
                vec![left.as_bytes(), right.as_bytes()],
            )
        })
        .collect();

    let mut current_node = b"AAA".as_slice();
    let mut p1_steps = 0u64;
    loop {
        let left = steps[p1_steps as usize % steps.len()] == b'L';
        current_node = network.get(current_node).unwrap()[if left { 0 } else { 1 }];
        p1_steps += 1;
        if current_node == b"ZZZ" {
            break;
        }
    }

    // Since we know that each path forms a cycle containing an "end" node (i.e. which
    // ends in 'Z'), if we make the further assumption that those cycles don't start on
    // non-divisible offset from the start nodes, we can find the first step where all
    // the paths land on a end node at the same time by taking the LCM of each path's
    // cycle length.
    // This additionally makes the assumption that there is only one "end" node in each path.
    let p2_steps = network
        .keys()
        .filter(|n| *n.last().unwrap() == b'A')
        .copied()
        .map(|start_node| {
            // Find the cycle length
            let mut current_node = start_node;
            let mut steps_to_z = None;
            let mut n_steps = 0u64;
            loop {
                let left = steps[n_steps as usize % steps.len()] == b'L';
                current_node = network.get(current_node).unwrap()[if left { 0 } else { 1 }];
                n_steps += 1;
                if *current_node.last().unwrap() == b'Z' {
                    if let Some(s) = steps_to_z {
                        let cycle_length = n_steps - s;
                        // The "LCM of cycle length" strategy only works if cycle length is
                        // a multiple of the length-to-first-Z-node -- i.e., that the initial
                        // offset before the cycle start is 0, or a mulitple of the cycle length.
                        assert!(s % cycle_length == 0);
                        return cycle_length;
                    } else {
                        steps_to_z = Some(n_steps);
                    }
                }
            }
        })
        .fold(1, |acc, v| acc.lcm(&v));
    (p1_steps, p2_steps)
}

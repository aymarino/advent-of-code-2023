use cached::{proc_macro::cached, Cached};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct WalkState<'a> {
    map: &'a [u8],
    pattern: &'a [u8],
    current_chain: u8,
}

impl WalkState<'_> {
    fn get_key(&self) -> (usize, usize, u8) {
        (self.map.len(), self.pattern.len(), self.current_chain)
    }
}

#[cached(key = "(usize, usize, u8)", convert = r#"{ arrangement.get_key() }"#)]
fn num_ways_to_arrange(arrangement: WalkState) -> u64 {
    let possible_remaining = arrangement
        .map
        .iter()
        .filter(|c| **c == b'#' || **c == b'?')
        .count() as u8;
    let required = arrangement.pattern.iter().sum::<u8>();
    if possible_remaining + arrangement.current_chain < required {
        return 0;
    }

    if required == 0 && arrangement.current_chain > 0 {
        return 0;
    }

    let ways_as_ground = || {
        if arrangement.current_chain > 0
            && *arrangement.pattern.first().unwrap() != arrangement.current_chain
        {
            return 0;
        }
        let pattern_idx = if arrangement.current_chain > 0 { 1 } else { 0 };
        num_ways_to_arrange(WalkState {
            map: &arrangement.map[1..],
            pattern: &arrangement.pattern[pattern_idx..],
            current_chain: 0,
        })
    };

    let ways_as_hash = || {
        num_ways_to_arrange(WalkState {
            map: &arrangement.map[1..],
            pattern: arrangement.pattern,
            current_chain: arrangement.current_chain + 1,
        })
    };

    if let Some(m) = arrangement.map.first() {
        match m {
            b'.' => {
                // End current chain, if any. Check if it matches the current pattern
                ways_as_ground()
            }
            b'#' => {
                // Continue or start current chain
                ways_as_hash()
            }
            b'?' => {
                // Try as both '#' and '.'
                ways_as_hash() + ways_as_ground()
            }
            _ => panic!("Invalid char {m}"),
        }
    } else {
        // End of map: return 1 if current chain matches pattern, 0 otherwise
        if let Some(p) = arrangement.pattern.first() {
            // Current chain must match pattern, and that pattern must be the last
            if arrangement.current_chain > 0
                && arrangement.pattern.len() == 1
                && arrangement.current_chain == *p
            {
                1
            } else {
                0
            }
        } else {
            // No remaining pattern: return 1 iff there is no chain
            if arrangement.current_chain == 0 {
                1
            } else {
                0
            }
        }
    }
}

fn get_input(line: &str, n_copies: u32) -> (Vec<u8>, Vec<u8>) {
    let (map, pattern) = line.split_once(' ').unwrap();

    let mut map = map.as_bytes().to_vec();
    let orig_line = map.clone();
    for _ in 0..n_copies {
        map.push(b'?');
        map.append(&mut orig_line.clone());
    }

    let mut pattern = pattern
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>();
    let orig_ranges = pattern.clone();
    for _ in 0..n_copies {
        pattern.append(&mut orig_ranges.clone());
    }

    (map, pattern)
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/12.txt");
    let p1 = input
        .lines()
        .map(|line| get_input(line, 0))
        .map(|(map, pattern)| {
            let n = num_ways_to_arrange(WalkState {
                map: map.as_slice(),
                pattern: pattern.as_slice(),
                current_chain: 0,
            });
            NUM_WAYS_TO_ARRANGE.lock().unwrap().cache_clear();
            n
        })
        .sum::<u64>();
    let p2 = input
        .lines()
        .map(|line| get_input(line, 4))
        .map(|(map, pattern)| {
            let n = num_ways_to_arrange(WalkState {
                map: map.as_slice(),
                pattern: pattern.as_slice(),
                current_chain: 0,
            });
            NUM_WAYS_TO_ARRANGE.lock().unwrap().cache_clear();
            n
        })
        .sum::<u64>();
    (p1, p2)
}

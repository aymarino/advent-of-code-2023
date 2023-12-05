use itertools::Itertools;

type Range = (u64, u64);

struct Mapping {
    src: Range,
    dst: Range,
}

pub fn soln() -> (u64, u64) {
    let input = std::fs::read_to_string("input/5.txt").unwrap();
    let (seeds, mappings) = input.split_once('\n').unwrap();

    let seeds: Vec<_> = seeds
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let categories: Vec<_> = mappings
        .trim()
        .split("\n\n")
        .map(|category| {
            let (_, mappings) = category.split_once('\n').unwrap();
            mappings
                .split('\n')
                .map(|mapping| {
                    let (dest_start, source_start, range_length) = mapping
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .tuples()
                        .next()
                        .unwrap();
                    Mapping {
                        src: (source_start, source_start + range_length),
                        dst: (dest_start, dest_start + range_length),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let p1 = seeds
        .iter()
        .map(|seed| {
            // For each seed, find the mapped location (last mapped) value
            let mut current_number = *seed;
            for c in &categories {
                if let Some(mapping) = c.iter().find_map(|m| {
                    if (m.src.0..m.src.1).contains(&current_number) {
                        Some(m.dst.0 + (current_number - m.src.0))
                    } else {
                        None
                    }
                }) {
                    current_number = mapping;
                }
                // else, current_numebr is unchanged
            }
            current_number
        })
        .min()
        .unwrap();

    let seeds: Vec<_> = seeds
        .into_iter()
        .tuples::<(u64, u64)>()
        .map(|(start, len)| (start, start + len))
        .collect();
    let p2 = seeds
        .into_iter()
        .map(|(start, end)| {
            let mut ranges = vec![(start, end)];
            for c in &categories {
                let mut mapped_ranges = Vec::new();
                for m in c {
                    let mut unmapped_ranges = Vec::new();
                    while !ranges.is_empty() {
                        let r = ranges.pop().unwrap();
                        let before = (r.0, r.1.min(m.src.0)); // the part of `r` before `m.src`
                        let between = (r.0.max(m.src.0), r.1.min(m.src.1)); // the part of `r` within `m.src`
                        let after = (r.0.max(m.src.1), r.1); // the part of `r` after `m.src`
                        if before.0 < before.1 {
                            unmapped_ranges.push(before);
                        }
                        if between.0 < between.1 {
                            mapped_ranges.push((
                                m.dst.0 + (between.0 - m.src.0),
                                m.dst.0 + (between.1 - m.src.0),
                            ));
                        }
                        if after.0 < after.1 {
                            unmapped_ranges.push(after);
                        }
                    }
                    ranges = unmapped_ranges;
                }
                ranges.append(&mut mapped_ranges);
            }
            ranges.iter().map(|r| r.0).min().unwrap()
        })
        .min()
        .unwrap();

    (p1, p2)
}

use std::collections::HashSet;

use itertools::Itertools;

struct MapInfo {
    stars: HashSet<(usize, usize)>,
    cols_with_star: HashSet<usize>,
    rows_with_star: HashSet<usize>,
}

fn get_index_adjustments(star_set: &HashSet<usize>, expansion_factor: u32) -> Vec<usize> {
    let max_index = *star_set.iter().max().unwrap();
    let mut cumulative_adjustments = Vec::<usize>::new();
    for i in 0..=max_index {
        let has_star = star_set.contains(&i);
        let last = cumulative_adjustments.last().unwrap_or(&0usize);
        cumulative_adjustments.push(if has_star {
            *last
        } else {
            *last + expansion_factor as usize - 1
        });
    }
    cumulative_adjustments
}

fn sum_shortest_paths(map_info: &MapInfo, factor: u32) -> u64 {
    let cumulative_col_adjustment = get_index_adjustments(&map_info.cols_with_star, factor);
    let cumulative_row_adjustment = get_index_adjustments(&map_info.rows_with_star, factor);

    map_info
        .stars
        .iter()
        .map(|(i, j)| {
            (
                i + cumulative_row_adjustment[*i],
                j + cumulative_col_adjustment[*j],
            )
        })
        .combinations(2)
        .map(|v| v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1))
        .sum::<usize>() as u64
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/11.txt");
    let stars = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| match c {
                '#' => Some((i, j)),
                _ => None,
            })
        })
        .flatten()
        .collect::<HashSet<_>>();
    let rows_with_star = stars.iter().map(|(i, _)| *i).collect::<HashSet<_>>();
    let cols_with_star = stars.iter().map(|(_, j)| *j).collect::<HashSet<_>>();

    let map_info = MapInfo {
        stars,
        cols_with_star,
        rows_with_star,
    };
    let p1 = sum_shortest_paths(&map_info, 2);
    let p2 = sum_shortest_paths(&map_info, 1000000);
    (p1, p2)
}

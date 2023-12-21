use std::collections::HashSet;

use itertools::Itertools;

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/21.txt");
    let map = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<_>>>();

    let starting_point = map
        .iter()
        .enumerate()
        .find_map(|(row_i, row)| {
            if let Some((col_i, _)) = row.iter().find_position(|b| **b == b'S') {
                Some((row_i as i64, col_i as i64))
            } else {
                None
            }
        })
        .unwrap();

    let n_rows = map.len() as i64;
    let n_cols = map[0].len() as i64;
    assert!(n_rows == n_cols); // Logic below expects square input

    let adjust = |(x, y): (i64, i64)| {
        let mut x = x % n_rows;
        if x < 0 {
            x += n_rows;
        }

        let mut y = y % n_rows;
        if y < 0 {
            y += n_cols;
        }

        (x, y)
    };

    let mut current_points = HashSet::from([starting_point]);
    let mut p1 = 0;
    let p1_target_steps = 64;

    let p2_target_steps = 26501365;
    let mut p2_points = Vec::new();
    for steps in 0..p2_target_steps {
        if steps == p1_target_steps {
            p1 = current_points.len();
        }

        if steps % n_rows == p2_target_steps % n_rows {
            p2_points.push(current_points.len());
            if p2_points.len() == 3 {
                // Need at least 3 points to extrapolate quadratic relation
                break;
            }
        }

        let mut next_points = HashSet::new();
        let mut add_if_not_rock = |(x, y)| {
            let real_coords = adjust((x, y));
            if map[real_coords.0 as usize][real_coords.1 as usize] != b'#' {
                next_points.insert((x, y));
            }
        };
        for p in &current_points {
            add_if_not_rock((p.0 - 1, p.1));
            add_if_not_rock((p.0 + 1, p.1));
            add_if_not_rock((p.0, p.1 - 1));
            add_if_not_rock((p.0, p.1 + 1));
        }
        current_points = next_points;
    }

    // The # of points reachable by `n` steps is perfectly quadratic at intervals of `n_rows`.
    // Since we have the result at each of `target % n_rows` steps, we can evaluate the fn
    // at the `target / n_rows`th step to get the answer.
    let (f0, f1, f2) = p2_points.iter().tuples().next().unwrap();
    dbg!(&f0, &f1, &f2);
    let (c0, c1, c2) = (*f0 as i64, (f1 - f0) as i64, (f2 - f1) as i64);

    let x = p2_target_steps / n_rows;
    let p2 = c0 + c1 * x + (x * (x - 1) / 2) * (c2 - c1);

    (p1 as u64, p2 as u64)
}

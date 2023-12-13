use std::collections::HashSet;

fn solve(input: &str, num_mistakes: usize) -> u32 {
    input
        .split("\n\n")
        .map(|case| {
            let n_rows = case.lines().count() as i32;
            let n_cols = case.find('\n').unwrap() as i32;
            let rocks = case
                .lines()
                .enumerate()
                .flat_map(|(i, line)| {
                    line.chars().enumerate().filter_map(move |(j, c)| match c {
                        '#' => Some((i as i32, j as i32)),
                        _ => None,
                    })
                })
                .collect::<HashSet<_>>();

            if let Some(row) = (0..n_rows - 1).find(|row| {
                rocks
                    .iter()
                    .filter(|r| {
                        let rock_row = r.0;
                        let reflected_row = if rock_row > *row {
                            row - (rock_row - row) + 1
                        } else {
                            row + (row - rock_row) + 1
                        };
                        let out_of_bounds = reflected_row < 0 || reflected_row >= n_rows;
                        !out_of_bounds && !rocks.contains(&(reflected_row, r.1))
                    })
                    .count()
                    == num_mistakes
            }) {
                (row + 1) * 100
            } else if let Some(col) = (0..n_cols - 1).find(|col| {
                rocks
                    .iter()
                    .filter(|r| {
                        let rock_col = r.1;
                        let reflected_col = if rock_col > *col {
                            col - (rock_col - col) + 1
                        } else {
                            col + (col - rock_col) + 1
                        };
                        let out_of_bounds = reflected_col < 0 || reflected_col >= n_cols;
                        !out_of_bounds && !rocks.contains(&(r.0, reflected_col))
                    })
                    .count()
                    == num_mistakes
            }) {
                col + 1
            } else {
                panic!("Did not find reflective col or row")
            }
        })
        .sum::<i32>() as u32
}

pub fn soln() -> (u32, u32) {
    let input = include_str!("../input/13.txt");
    let p1 = solve(input, 0);
    let p2 = solve(input, 1);
    (p1, p2)
}

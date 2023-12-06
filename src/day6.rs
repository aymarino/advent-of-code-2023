use itertools::Itertools;

fn solve_num_ways(t: u64, d: u64) -> u64 {
    // Quadratic equation: need c s.t. c * (t - c) > d
    // -> points between 0.5 (t +/- sqrt(t^2 - 4d))
    let (t, d) = (t as f64, d as f64);
    let discriminant = (t * t - 4.0 * d).sqrt();
    let (low, high) = (0.5 * (t - discriminant), 0.5 * (t + discriminant));

    // To convert float solutions to ints which satisfy the inequality,
    // take inclusive range [floor(low) + 1, ceil(high) - 1].
    let (low, high) = (low.floor() as u64 + 1, high.ceil() as u64 - 1);
    high - low + 1 // +1 for inclusive range
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/6.txt");
    let (times, distances) = input
        .trim()
        .split('\n')
        .map(|s| {
            s.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .tuples()
        .next()
        .unwrap();

    let p1 = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| solve_num_ways(*time, *distance))
        .product();

    let (time, distance) = [times, distances]
        .iter()
        .map(|v| {
            v.iter()
                .map(u64::to_string)
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .tuples()
        .next()
        .unwrap();
    let p2 = solve_num_ways(time, distance);
    (p1, p2)
}

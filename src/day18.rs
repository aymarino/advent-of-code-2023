use itertools::Itertools;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    dir: Direction,
    len: u64,
}

fn get_area(instrs: Vec<Instruction>) -> i64 {
    let mut current = (0, 0);
    let mut points = vec![current];
    let mut path_len = 0;
    points.extend(instrs.into_iter().map(|i| {
        let delta = match i.dir {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        current = (
            current.0 + delta.0 * i.len as i64,
            current.1 + delta.1 * i.len as i64,
        );
        path_len += i.len;
        current
    }));

    // Use Shoelace formula to calculate interior area of the polygon
    // created by the points.
    let polygon_area = points
        .windows(2)
        .map(|w| (w[0].1 + w[1].1) * (w[0].0 - w[1].0))
        .sum::<i64>()
        .abs()
        / 2;
    // Pick's theorem relates polygon area to boundary points (b) + interior points (i):
    // A = i + b/2 - 1
    // Area of pit is `b + i`:
    //    i + b/2 - 1 = A
    // => i + b       = A + b/2 + 1   (add b/2 + 1 to both sides)
    polygon_area + path_len as i64 / 2 + 1
}

pub fn soln() -> (i64, i64) {
    let input = include_str!("../input/18.txt");
    let p1_instructions = input
        .lines()
        .map(|line| {
            let (dir, len, _) = line.split(' ').tuples().next().unwrap();
            let dir = match dir {
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "U" => Direction::Up,
                _ => panic!("Unexpected direction value in instructions"),
            };
            let len = len.parse().unwrap();
            Instruction { dir, len }
        })
        .collect::<Vec<_>>();
    let p2_instructions = input
        .lines()
        .map(|line| {
            let (_, hex) = line.split_once('#').unwrap();
            let len = u64::from_str_radix(&hex[..5], 16).unwrap();
            let dir = match &hex[5..6] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Unexpected direction value in hex encoding"),
            };
            Instruction { dir, len }
        })
        .collect::<Vec<_>>();

    (get_area(p1_instructions), get_area(p2_instructions))
}

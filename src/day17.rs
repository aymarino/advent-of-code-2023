use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap: flip the ordering on costs.
        // In case of a tie we compare positions -- this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    coords: (usize, usize),
    direction: Direction,
    steps_in_dir: u32,
}

fn shortest_path(map: &Vec<Vec<u32>>, min_before_turning: u32, max_before_turning: u32) -> u32 {
    let mut visited = HashSet::<Position>::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        pos: Position {
            coords: (0, 0),
            direction: Direction::Right,
            steps_in_dir: 0,
        },
    });

    loop {
        let v = queue.pop().unwrap();
        if v.pos.coords == (map.len() - 1, map[0].len() - 1) {
            // Target
            return v.cost;
        }

        if visited.contains(&v.pos) {
            continue;
        } else {
            visited.insert(v.pos);
        }

        let pos = v.pos;
        let coords = v.pos.coords;

        let valid_neighbor = |new_dir, opposite_dir| {
            pos.direction != opposite_dir
                && !(pos.direction == new_dir && pos.steps_in_dir == max_before_turning)
                && !(pos.direction != new_dir && pos.steps_in_dir < min_before_turning)
        };

        let mut add_neighbor = |new_dir, opposite_dir, new_coord: (usize, usize)| {
            if !valid_neighbor(new_dir, opposite_dir) {
                return;
            }

            queue.push(State {
                cost: v.cost + map[new_coord.0][new_coord.1],
                pos: Position {
                    coords: new_coord,
                    direction: new_dir,
                    steps_in_dir: if pos.direction == new_dir {
                        pos.steps_in_dir + 1
                    } else {
                        1
                    },
                },
            });
        };

        // Right
        if coords.1 < map[0].len() - 1 {
            add_neighbor(Direction::Right, Direction::Left, (coords.0, coords.1 + 1));
        }
        // Left
        if coords.1 > 0 {
            add_neighbor(Direction::Left, Direction::Right, (coords.0, coords.1 - 1));
        }
        // Up
        if coords.0 > 0 {
            add_neighbor(Direction::Up, Direction::Down, (coords.0 - 1, coords.1));
        }
        // Down
        if coords.0 < map.len() - 1 {
            add_neighbor(Direction::Down, Direction::Up, (coords.0 + 1, coords.1));
        }
    }
}

pub fn soln() -> (u32, u32) {
    let input = include_str!("../input/17.txt");
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let p1 = shortest_path(&map, 0, 3);
    let p2 = shortest_path(&map, 4, 10);
    (p1, p2)
}

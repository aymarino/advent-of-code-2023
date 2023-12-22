use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Brick {
    start: (u32, u32, u32),
    end: (u32, u32, u32),
}

impl Brick {
    fn overlaps(&self, b: &Brick) -> bool {
        let x_intersects = self.start.0 <= b.end.0 && self.end.0 >= b.start.0;
        let y_intersects = self.start.1 <= b.end.1 && self.end.1 >= b.start.1;
        let z_intersects = self.start.2 <= b.end.2 && self.end.2 >= b.start.2;
        x_intersects && y_intersects && z_intersects
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .2
            .cmp(&other.start.2)
            .then_with(|| self.end.2.cmp(&other.end.2))
            .then_with(|| self.start.cmp(&other.start))
            .then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/22.txt");
    let mut bricks = input
        .lines()
        .map(|brick| {
            let (start, end) = brick.split_once('~').unwrap();
            let (x1, y1, z1) = start
                .split(',')
                .map(|coord| coord.parse::<u32>().unwrap())
                .tuples()
                .next()
                .unwrap();
            let (x2, y2, z2) = end
                .split(',')
                .map(|coord| coord.parse::<u32>().unwrap())
                .tuples()
                .next()
                .unwrap();
            Brick {
                start: (x1, y1, z1),
                end: (x2, y2, z2),
            }
        })
        .collect::<Vec<_>>();

    bricks.sort();

    let mut fallen_bricks = Vec::new();
    bricks.into_iter().for_each(|b| {
        let mut b = b;
        while b.start.2 > 1 {
            let mut new_brick = b;
            new_brick.start.2 -= 1;
            new_brick.end.2 -= 1;
            if fallen_bricks.iter().rev().any(|b| new_brick.overlaps(b)) {
                break;
            }
            b = new_brick;
        }
        fallen_bricks.push(b);
    });

    let supports = fallen_bricks
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let mut bricks_atop = Vec::new();
            for next_brick in &fallen_bricks[i + 1..] {
                let mut n = *next_brick;
                n.start.2 -= 1;
                n.end.2 -= 1;
                if n.overlaps(&b) {
                    bricks_atop.push(n);
                }
            }
            (b, bricks_atop)
        })
        .collect::<HashMap<Brick, Vec<_>>>();

    let mut supported_by = HashMap::<Brick, u32>::new();
    for supported_bricks in supports.values() {
        for s in supported_bricks {
            match supported_by.entry(*s) {
                Entry::Occupied(mut e) => {
                    *e.get_mut() += 1;
                }
                Entry::Vacant(e) => {
                    e.insert(1);
                }
            }
        }
    }

    let critical_bricks: Vec<_> = fallen_bricks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if let Some(supports) = supports.get(b) {
                for s in supports {
                    if *supported_by.get(s).unwrap() == 1 {
                        return Some(i);
                    }
                }
            }
            None
        })
        .collect();
    let p1 = fallen_bricks.len() - critical_bricks.len();

    let p2 = critical_bricks
        .into_iter()
        .map(|i| {
            let mut bricks = fallen_bricks.clone();
            bricks.remove(i);
            let mut new_fallen_bricks = Vec::new();
            let mut num_fallen = 0;
            for b in &bricks {
                let mut falling = *b;
                while falling.start.2 > 1 {
                    let mut new_brick = falling;
                    new_brick.start.2 -= 1;
                    new_brick.end.2 -= 1;
                    if new_fallen_bricks
                        .iter()
                        .rev()
                        .any(|b| new_brick.overlaps(b))
                    {
                        break;
                    }
                    falling = new_brick;
                }
                if falling != *b {
                    num_fallen += 1;
                }
                new_fallen_bricks.push(falling);
            }
            num_fallen
        })
        .sum();

    (p1 as u64, p2)
}

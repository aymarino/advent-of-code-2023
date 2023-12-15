use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone, Hash, Eq, PartialEq)]
enum MapItem {
    Ground,
    RollingRock,
    Cube,
}

impl MapItem {
    fn from(c: &u8) -> Self {
        match c {
            b'.' => Self::Ground,
            b'O' => Self::RollingRock,
            b'#' => Self::Cube,
            _ => panic!("Invalid map item"),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Map {
    pub map: Vec<Vec<MapItem>>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl Map {
    fn new(map: Vec<Vec<MapItem>>) -> Self {
        let n_rows = map.len();
        let n_cols = map[0].len();
        Self {
            map,
            n_rows,
            n_cols,
        }
    }

    fn roll_north(&mut self) {
        for col in 0..self.n_cols {
            let mut next_rock_row = 0;
            for row in 0..self.n_rows {
                match &self.map[row][col] {
                    MapItem::RollingRock => {
                        self.map[row][col] = MapItem::Ground;
                        self.map[next_rock_row][col] = MapItem::RollingRock;
                        next_rock_row += 1;
                    }
                    MapItem::Cube => {
                        next_rock_row = row + 1;
                    }
                    MapItem::Ground => {}
                }
            }
        }
    }

    fn roll_south(&mut self) {
        for col in 0..self.n_cols {
            let mut next_rock_row = self.n_rows - 1;
            for row in (0..self.n_rows).rev() {
                match &self.map[row][col] {
                    MapItem::RollingRock => {
                        self.map[row][col] = MapItem::Ground;
                        self.map[next_rock_row][col] = MapItem::RollingRock;
                        next_rock_row = next_rock_row.saturating_sub(1);
                    }
                    MapItem::Cube => {
                        next_rock_row = row.saturating_sub(1);
                    }
                    MapItem::Ground => {}
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for row in 0..self.n_rows {
            let mut next_rock_col = 0;
            for col in 0..self.n_cols {
                match &self.map[row][col] {
                    MapItem::RollingRock => {
                        self.map[row][col] = MapItem::Ground;
                        self.map[row][next_rock_col] = MapItem::RollingRock;
                        next_rock_col += 1;
                    }
                    MapItem::Cube => {
                        next_rock_col = col + 1;
                    }
                    MapItem::Ground => {}
                }
            }
        }
    }

    fn roll_east(&mut self) {
        for row in 0..self.n_rows {
            let mut next_rock_col = self.n_cols - 1;
            for col in (0..self.n_cols).rev() {
                match &self.map[row][col] {
                    MapItem::RollingRock => {
                        self.map[row][col] = MapItem::Ground;
                        self.map[row][next_rock_col] = MapItem::RollingRock;
                        next_rock_col = next_rock_col.saturating_sub(1);
                    }
                    MapItem::Cube => next_rock_col = col.saturating_sub(1),
                    MapItem::Ground => {}
                }
            }
        }
    }

    fn load(&self) -> u64 {
        let mut weight = 0;
        for col in 0..self.n_cols {
            for row in 0..self.n_rows {
                if let MapItem::RollingRock = &self.map[row][col] {
                    weight += self.n_rows - row;
                }
            }
        }
        weight as u64
    }

    #[allow(unused)]
    fn print(&self) {
        let as_str = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| match item {
                        MapItem::Ground => '.',
                        MapItem::RollingRock => 'O',
                        MapItem::Cube => '#',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("{}", as_str);
    }
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/14.txt");
    let map = input
        .split('\n')
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(MapItem::from)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = Map::new(map);
    let mut positions = HashMap::new();
    positions.insert(map.clone(), 0);
    map.roll_north();
    let p1 = map.load();

    // Complete rest of first cycle
    map.roll_west();
    map.roll_south();
    map.roll_east();

    let mut positions = HashMap::new();
    positions.insert(map.clone(), 1);

    let target_cycle = 1_000_000_000;
    let mut loop_start = 0;
    let mut loop_end = 0;
    for i in 2..=target_cycle {
        map.roll_north();
        map.roll_west();
        map.roll_south();
        map.roll_east();

        match positions.entry(map.clone()) {
            Entry::Occupied(entry) => {
                loop_start = *entry.get();
                loop_end = i;
                break;
            }
            Entry::Vacant(entry) => {
                entry.insert(i);
            }
        }
    }

    let cycle_length = loop_end - loop_start;
    let target_cycle = loop_start + (target_cycle - loop_start) % cycle_length;
    let target_map = positions
        .iter()
        .find(|(_, i)| **i == target_cycle)
        .unwrap()
        .0;
    let p2 = target_map.load();

    (p1, p2)
}

use std::collections::HashMap;

type Coord = (i32, i32);

struct Number {
    n: u32,
    start: Coord,
    length: i32,
}

impl Number {
    fn adjascent_indices(&self) -> Vec<Coord> {
        let mut adjascent_idx: Vec<Coord> = vec![
            (self.start.0, self.start.1 - 1),               // left
            (self.start.0 - 1, self.start.1 - 1),           // left-up
            (self.start.0 + 1, self.start.1 - 1),           // left-down
            (self.start.0, self.start.1 + self.length),     // right
            (self.start.0 - 1, self.start.1 + self.length), // right-up
            (self.start.0 + 1, self.start.1 + self.length), // right-down
        ];

        // Above and below
        for col_offset in 0..self.length {
            adjascent_idx.push((self.start.0 - 1, self.start.1 + col_offset));
            adjascent_idx.push((self.start.0 + 1, self.start.1 + col_offset));
        }

        adjascent_idx
    }
}

struct Map {
    numbers: Vec<Number>,
    symbols: HashMap<Coord, char>,
}

impl Map {
    fn from(input: &str) -> Self {
        let mut symbols = HashMap::new();
        let mut numbers = Vec::new();
        input.lines().enumerate().for_each(|(i, s)| {
            let mut j = 0;
            while j < s.len() {
                let c = s.as_bytes()[j] as char;
                match c {
                    '.' => {}
                    _ if c.is_digit(10) => {
                        let (n, len) = extract_number(&s[j..]);
                        numbers.push(Number {
                            n,
                            start: (i as i32, j as i32),
                            length: len as i32,
                        });
                        j += len - 1;
                    }
                    _ => {
                        symbols.insert((i as i32, j as i32), c);
                    }
                }
                j += 1;
            }
        });
        Self { numbers, symbols }
    }
}

/// Returns (n, len)
fn extract_number(input: &str) -> (u32, usize) {
    let number_end = input
        .chars()
        .position(|c| !c.is_digit(10))
        .or(Some(input.len()))
        .unwrap();
    (input[..number_end].parse().unwrap(), number_end)
}

pub fn soln_3_1() -> u32 {
    let input = std::fs::read_to_string("input/3.1.txt").unwrap();
    let map = Map::from(&input);

    map.numbers
        .iter()
        .filter_map(|n| {
            let adjascent_idx = n.adjascent_indices();
            if adjascent_idx
                .iter()
                .any(|idx| map.symbols.contains_key(idx))
            {
                Some(n.n)
            } else {
                None
            }
        })
        .sum()
}

pub fn soln_3_2() -> u32 {
    let input = std::fs::read_to_string("input/3.1.txt").unwrap();
    let map = Map::from(&input);

    let mut symbols: HashMap<Coord, Vec<_>> =
        HashMap::from_iter(map.symbols.iter().filter_map(|(idx, c)| match c {
            '*' => Some((idx.clone(), Vec::new())),
            _ => None,
        }));
    map.numbers.iter().for_each(|n| {
        let adjascent_idx = n.adjascent_indices();
        adjascent_idx.iter().for_each(|idx| {
            if let Some(v) = symbols.get_mut(idx) {
                (*v).push(n.n);
            }
        });
    });

    symbols
        .iter()
        .filter_map(|(_, v)| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum()
}

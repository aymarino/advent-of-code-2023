#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn position_offset(&self) -> (i64, i64) {
        match &self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Beam {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Beam {
    fn advance(self, max_row: usize, max_col: usize) -> Option<Self> {
        let offset = self.direction.position_offset();
        let (r, c) = (self.row as i64 + offset.0, self.col as i64 + offset.1);
        if r < 0 || r >= max_row as i64 || c < 0 || c >= max_col as i64 {
            None
        } else {
            Some(Self {
                row: r as usize,
                col: c as usize,
                direction: self.direction,
            })
        }
    }
}

fn get_energized_for_staring_position(map: &Vec<Vec<&u8>>, starting_position: Beam) -> u64 {
    let max_row = map.len();
    let max_col = map[0].len();
    let mut energized = vec![vec![None; max_col]; max_row];
    let mut positions = vec![starting_position];
    while let Some(mut c) = positions.pop() {
        if let Some(d) = &energized[c.row][c.col] {
            if *d == c.direction {
                continue;
            }
        }
        energized[c.row][c.col] = Some(c.direction);

        match &map[c.row][c.col] {
            b'.' => {
                if let Some(new_pos) = c.advance(max_row, max_col) {
                    positions.push(new_pos);
                }
            }
            b'/' => {
                match &c.direction {
                    Direction::Left => c.direction = Direction::Down,
                    Direction::Right => c.direction = Direction::Up,
                    Direction::Up => c.direction = Direction::Right,
                    Direction::Down => c.direction = Direction::Left,
                }
                if let Some(new_pos) = c.advance(max_row, max_col) {
                    positions.push(new_pos);
                }
            }
            b'\\' => {
                match &c.direction {
                    Direction::Left => c.direction = Direction::Up,
                    Direction::Right => c.direction = Direction::Down,
                    Direction::Up => c.direction = Direction::Left,
                    Direction::Down => c.direction = Direction::Right,
                }
                if let Some(new_pos) = c.advance(max_row, max_col) {
                    positions.push(new_pos);
                }
            }
            b'-' => match &c.direction {
                Direction::Left | Direction::Right => {
                    if let Some(new_pos) = c.advance(max_row, max_col) {
                        positions.push(new_pos);
                    }
                }
                Direction::Up | Direction::Down => {
                    c.direction = Direction::Left;
                    if let Some(left_pos) = c.advance(max_row, max_col) {
                        positions.push(left_pos);
                    }
                    c.direction = Direction::Right;
                    if let Some(right_pos) = c.advance(max_row, max_col) {
                        positions.push(right_pos);
                    }
                }
            },
            b'|' => match &c.direction {
                Direction::Up | Direction::Down => {
                    if let Some(new_pos) = c.advance(max_row, max_col) {
                        positions.push(new_pos);
                    }
                }
                Direction::Left | Direction::Right => {
                    c.direction = Direction::Up;
                    if let Some(up_pos) = c.advance(max_row, max_col) {
                        positions.push(up_pos);
                    }
                    c.direction = Direction::Down;
                    if let Some(down_pos) = c.advance(max_row, max_col) {
                        positions.push(down_pos);
                    }
                }
            },
            _ => panic!("Invalid square!"),
        }
    }

    energized
        .iter()
        .flat_map(|row| row.iter())
        .filter(|s| s.is_some())
        .count() as u64
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/16.txt");
    let map = input
        .lines()
        .map(|line| line.as_bytes().iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let p1 = get_energized_for_staring_position(
        &map,
        Beam {
            row: 0,
            col: 0,
            direction: Direction::Right,
        },
    );

    let mut starting_positions = Vec::new();
    // Top + bottom perimeter
    for col in 0..map[0].len() {
        starting_positions.push(Beam {
            row: 0,
            col,
            direction: Direction::Down,
        });
        starting_positions.push(Beam {
            row: map.len() - 1,
            col,
            direction: Direction::Up,
        });
    }
    // Left + right perimeter
    for row in 0..map.len() {
        starting_positions.push(Beam {
            row,
            col: 0,
            direction: Direction::Right,
        });
        starting_positions.push(Beam {
            row,
            col: map[0].len() - 1,
            direction: Direction::Left,
        });
    }

    let p2 = starting_positions
        .into_iter()
        .map(|pos| get_energized_for_staring_position(&map, pos))
        .max()
        .unwrap();
    (p1, p2)
}

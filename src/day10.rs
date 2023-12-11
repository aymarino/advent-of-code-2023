#[derive(Clone, Debug)]
enum PipeKind {
    Vert,
    Horz,
    L,
    J,
    Seven,
    F,
}

impl PipeKind {
    fn map_direction(&self, dir: (i32, i32)) -> Option<(i32, i32)> {
        match &self {
            PipeKind::Vert | PipeKind::Horz => Some(dir),
            PipeKind::L => {
                if dir.0 == 1 {
                    Some((0, 1))
                } else if dir.1 == -1 {
                    Some((-1, 0))
                } else {
                    None
                }
            }
            PipeKind::J => {
                if dir.0 == 1 {
                    Some((0, -1))
                } else if dir.1 == 1 {
                    Some((-1, 0))
                } else {
                    None
                }
            }
            PipeKind::Seven => {
                if dir.0 == -1 {
                    Some((0, -1))
                } else if dir.1 == 1 {
                    Some((1, 0))
                } else {
                    None
                }
            }
            PipeKind::F => {
                if dir.0 == -1 {
                    Some((0, 1))
                } else if dir.1 == -1 {
                    Some((1, 0))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Block {
    Ground,
    Start,
    Pipe(PipeKind),
}

fn move_coordinate(coord: &(usize, usize), dir: &(i32, i32)) -> (usize, usize) {
    (
        (coord.0 as i32 + dir.0) as usize,
        (coord.1 as i32 + dir.1) as usize,
    )
}

pub fn soln() -> (u32, u32) {
    let input = include_str!("../input/10.txt");

    let mut starting_pos = (0, 0);
    let map: Vec<Vec<Block>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Block::Ground,
                    'S' => {
                        starting_pos = (i, j);
                        Block::Start
                    }
                    '|' => Block::Pipe(PipeKind::Vert),
                    '-' => Block::Pipe(PipeKind::Horz),
                    'L' => Block::Pipe(PipeKind::L),
                    'J' => Block::Pipe(PipeKind::J),
                    '7' => Block::Pipe(PipeKind::Seven),
                    'F' => Block::Pipe(PipeKind::F),
                    _ => panic!("Invalid block letter {c}"),
                })
                .collect()
        })
        .collect();

    let mut direction = (|| {
        for d in [(-1i32, 0), (1i32, 0), (0, 1i32), (0, -1i32)] {
            if (starting_pos.0 == 0 && d.0 == -1)
                || (starting_pos.0 == (map.len() - 1) && d.0 == 1)
                || (starting_pos.1 == 0 && d.1 == -1)
                || (starting_pos.1 == (map[0].len() - 1) && d.1 == 1)
            {
                continue;
            }
            let coord = move_coordinate(&starting_pos, &d);
            match &map[coord.0][coord.1] {
                Block::Pipe(p) => {
                    if let Some(_) = p.map_direction(d) {
                        return d;
                    }
                }
                _ => {}
            }
        }
        panic!("Could not find starting dir");
    })();

    let mut in_path = vec![vec![false; map[0].len()]; map.len()];
    in_path[starting_pos.0][starting_pos.1] = true;
    let mut current_pos = move_coordinate(&starting_pos, &direction);
    let mut p1 = 1;
    while current_pos != starting_pos {
        in_path[current_pos.0][current_pos.1] = true;
        let current_block = &map[current_pos.0][current_pos.1];
        match &current_block {
            Block::Pipe(p) => {
                direction = p.map_direction(direction).unwrap();
            }
            _ => panic!("Invalid block in path: {:?}", current_block),
        }
        current_pos = move_coordinate(&current_pos, &direction);
        p1 += 1;
    }

    p1 = p1 / 2;

    let p2 = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut within_loop = false;
            row.iter()
                .enumerate()
                .map(|(j, block)| {
                    if !in_path[i][j] {
                        if within_loop {
                            1
                        } else {
                            0
                        }
                    } else {
                        match &block {
                            Block::Pipe(k) => match k {
                                PipeKind::Vert | PipeKind::L | PipeKind::J => {
                                    within_loop = !within_loop;
                                }
                                _ => {}
                            },
                            Block::Start => {}
                            _ => panic!("Unexpected Block kind in path"),
                        };
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();
    (p1, p2)
}

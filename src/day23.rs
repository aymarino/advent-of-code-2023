use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Path,
    SlopeLeft,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    Forest,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    n_rows: usize,
    n_cols: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let start = tiles[0]
            .iter()
            .find_position(|&&t| t == Tile::Path)
            .unwrap()
            .0;
        let end = tiles
            .last()
            .unwrap()
            .iter()
            .find_position(|&&t| t == Tile::Path)
            .unwrap()
            .0;
        let start = (0usize, start);
        let end = (tiles.len() - 1, end);
        let n_rows = tiles.len();
        let n_cols = tiles[0].len();
        Self {
            tiles,
            n_rows,
            n_cols,
            start,
            end,
        }
    }
}

fn dfs(
    map: &Map,
    visited: &mut Vec<Vec<bool>>,
    current: (usize, usize),
    dist: u64,
    max_dist: &mut u64,
    ignore_slopes: bool,
) {
    if current == map.end {
        *max_dist = (*max_dist).max(dist);
        return;
    }
    visited[current.0][current.1] = true;
    let neighbors = match &map.tiles[current.0][current.1] {
        _ if ignore_slopes => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
        Tile::Path => [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice(),
        Tile::SlopeUp => [(-1, 0)].as_slice(),
        Tile::SlopeRight => [(0, 1)].as_slice(),
        Tile::SlopeDown => [(1, 0)].as_slice(),
        Tile::SlopeLeft => [(0, -1)].as_slice(),
        _ => unreachable!(),
    };
    for d in neighbors {
        let next = (current.0 as isize + d.0, current.1 as isize + d.1);
        if next.0 < 0
            || next.0 >= map.n_rows as isize
            || next.1 < 0
            || next.1 >= map.n_cols as isize
        {
            continue;
        }
        let next = (next.0 as usize, next.1 as usize);
        if map.tiles[next.0][next.1] == Tile::Forest || visited[next.0][next.1] {
            continue;
        }
        dfs(map, visited, next, dist + 1, max_dist, ignore_slopes);
    }
    visited[current.0][current.1] = false;
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/23.txt");
    let map = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| match c {
                    b'.' => Tile::Path,
                    b'>' => Tile::SlopeRight,
                    b'<' => Tile::SlopeLeft,
                    b'^' => Tile::SlopeUp,
                    b'v' => Tile::SlopeDown,
                    b'#' => Tile::Forest,
                    _ => panic!("Unexpected char in input: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let map = Map::new(map);
    let p1 = {
        let mut visited = vec![vec![false; map.n_cols]; map.n_rows];
        let mut max_dist = 0;
        dfs(&map, &mut visited, map.start, 0, &mut max_dist, false);
        max_dist
    };
    let p2 = {
        let mut visited = vec![vec![false; map.n_cols]; map.n_rows];
        let mut max_dist = 0;
        dfs(&map, &mut visited, map.start, 0, &mut max_dist, true);
        max_dist
    };
    (p1, p2)
}

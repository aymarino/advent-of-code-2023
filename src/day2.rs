fn parse_game(game_str: &str) -> impl Iterator<Item = (u32, &str)> {
    game_str
        .split_once(": ")
        .unwrap()
        .1
        .split("; ")
        .flat_map(|set| set.split(", "))
        .map(|draw| {
            let (n, color) = draw.split_once(' ').unwrap();
            (n.parse().unwrap(), color)
        })
}

pub fn soln_2_1() -> usize {
    std::fs::read_to_string("input/2.1.txt")
        .unwrap()
        .lines()
        .map(parse_game)
        .enumerate()
        .filter_map(|(i, game)| {
            for (n, color) in game {
                match color {
                    "red" if n > 12 => return None,
                    "green" if n > 13 => return None,
                    "blue" if n > 14 => return None,
                    _ => {}
                }
            }
            Some(i + 1)
        })
        .sum()
}

pub fn soln_2_2() -> u32 {
    std::fs::read_to_string("input/2.2.txt")
        .unwrap()
        .lines()
        .map(parse_game)
        .map(|game| {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for (n, color) in game {
                match color {
                    "red" => red = red.max(n),
                    "green" => green = green.max(n),
                    "blue" => blue = blue.max(n),
                    _ => {}
                }
            }
            red * green * blue
        })
        .sum()
}

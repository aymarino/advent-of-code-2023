use itertools::Itertools;
use std::collections::HashSet;

fn n_winners(card: &str) -> usize {
    let card = card.split_once(": ").unwrap().1;
    let (winners, yours) = card
        .split(" | ")
        .map(|list| {
            list.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<HashSet<_>>()
        })
        .tuples()
        .next()
        .unwrap();
    winners.intersection(&yours).count()
}

pub fn soln_4_1() -> u32 {
    std::fs::read_to_string("input/4.txt")
        .unwrap()
        .lines()
        .map(n_winners)
        .filter(|n_winners| *n_winners != 0)
        .map(|n_winners| 2u32.pow(n_winners as u32 - 1))
        .sum()
}

pub fn soln_4_2() -> u32 {
    let n_winners: Vec<_> = std::fs::read_to_string("input/4.txt")
        .unwrap()
        .lines()
        .map(n_winners)
        .collect();
    let mut n_cards = vec![1u32; n_winners.len()];
    for (i, n) in n_winners.iter().enumerate() {
        let n_copies = n_cards[i];
        n_cards[(i + 1)..=(i + *n)]
            .iter_mut()
            .for_each(|n| *n += n_copies);
    }

    n_cards.iter().sum()
}

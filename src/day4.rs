use std::collections::HashSet;

fn n_winners(card: &str) -> u32 {
    let card = card.split_once(": ").unwrap().1;
    let mut card_lists = card.split(" | ").map(|list| {
        list.split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<_>>()
    });
    let winners = card_lists.next().unwrap();
    let yours = card_lists.next().unwrap();
    winners.intersection(&yours).count() as u32
}

pub fn soln_4_1() -> u32 {
    std::fs::read_to_string("input/4.txt")
        .unwrap()
        .lines()
        .map(n_winners)
        .map(|n_winners| {
            if n_winners == 0 {
                0
            } else {
                2u32.pow(n_winners - 1)
            }
        })
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
        for j in 1..=*n {
            n_cards[i + j as usize] += n_copies;
        }
    }

    n_cards.iter().sum()
}

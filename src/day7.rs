use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandKind {
    fn from(hand: &Vec<Card>) -> Self {
        let hand_map = {
            let mut counts = HashMap::new();
            let mut n_jokers = 0u8;
            for c in hand {
                if *c == Card::Joker {
                    n_jokers += 1;
                } else {
                    *counts.entry(c).or_insert(0) += 1;
                }
            }

            // Swap Joker -> card with the highest count
            if let Some(max_char) = counts.iter().max_by_key(|e| e.1) {
                let max_char = max_char.0;
                *(counts.get_mut(*max_char).unwrap()) += n_jokers;
            } else {
                // This is the case where `counts` is empty because the hand is all Jokers.
                // Map it to 5-of-a-kind Aces (lol).
                counts.insert(&Card::A, 5);
            }
            counts
        };

        let max_card_count = hand_map.values().max().unwrap();
        if hand_map.len() == 1 {
            Self::FiveKind
        } else if hand_map.len() == 2 {
            if *max_card_count == 4 {
                Self::FourKind
            } else {
                Self::FullHouse
            }
        } else if hand_map.len() == 3 {
            if *max_card_count == 3 {
                Self::ThreeKind
            } else {
                Self::TwoPair
            }
        } else if hand_map.len() == 4 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Joker,
    N(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from(c: char) -> Self {
        if c.is_digit(10) {
            return Card::N(c.to_digit(10).unwrap() as u8);
        }
        match c {
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("invalid card"),
        }
    }

    fn to_p2(self) -> Self {
        match self {
            Card::J => Card::Joker,
            _ => self,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Hand {
    kind: HandKind,
    hand: Vec<Card>,
    bid: u64,
}

fn solve_hands(hands: &mut Vec<Hand>) -> u64 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u64 * h.bid)
        .sum()
}

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/7.txt");
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand_str, bid) = l.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let hand: Vec<_> = hand_str.chars().map(Card::from).collect();
            Hand {
                kind: HandKind::from(&hand),
                hand,
                bid,
            }
        })
        .collect::<Vec<_>>();
    let p1 = solve_hands(&mut hands);

    // Convert J -> Joker and re-evaluate hand kinds for part 2
    let mut hands: Vec<_> = hands
        .into_iter()
        .map(|h| {
            let hand_p2: Vec<_> = h.hand.into_iter().map(Card::to_p2).collect();
            Hand {
                kind: HandKind::from(&hand_p2),
                hand: hand_p2,
                bid: h.bid,
            }
        })
        .collect();
    let p2 = solve_hands(&mut hands);

    (p1, p2)
}

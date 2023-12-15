use std::fs::read_to_string;

fn first_last(digits: &[u32]) -> u32 {
    assert!(!digits.is_empty());
    if digits.len() == 1 {
        let d = digits.first().unwrap();
        d * 10 + d
    } else {
        digits.first().unwrap() * 10 + digits.last().unwrap()
    }
}

pub fn soln_1_1() -> u32 {
    read_to_string("input/1.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let digits: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();
            first_last(&digits)
        })
        .sum()
}

fn get_digits(s: &str) -> Vec<u32> {
    let mut result = vec![];
    'string_walk: for i in 0..s.len() {
        if let Some(d) = (s.as_bytes()[i] as char).to_digit(10) {
            result.push(d);
            continue 'string_walk;
        }

        for (digit_name, digit) in [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ] {
            if s[i..].starts_with(digit_name) {
                result.push(digit);
                continue 'string_walk;
            }
        }
    }
    result
}

pub fn soln_1_2() -> u32 {
    read_to_string("input/1.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let digits = get_digits(s);
            first_last(&digits)
        })
        .sum()
}

use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
struct Card {
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, value) = value.split_once(':').unwrap();
        let (winning_numbers, numbers) = value.split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<HashSet<_>>();
        let numbers = numbers
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<HashSet<_>>();

        Self {
            numbers,
            winning_numbers,
        }
    }
}

impl Card {
    fn score(&self) -> u32 {
        let matches = self.matches() as u32;

        if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        }
    }

    fn matches(&self) -> usize {
        self.numbers.intersection(&self.winning_numbers).count()
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    input.lines().map(Card::from).map(|c| c.score()).sum()
}

fn solve_second_part(input: &str) -> u32 {
    let matches = input.lines().map(Card::from).map(|c| c.matches());

    matches
        .clone()
        .enumerate()
        .fold(vec![1; matches.count()], |mut acc, (index, matches)| {
            for i in index + 1..index + matches + 1 {
                acc[i] += acc[index];
            }
            acc
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_first_part() {
        let answer = 13;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 30;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(25571, 8805731);
}

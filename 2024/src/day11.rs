use std::{collections::HashMap, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u64 {
    blinks(input, 25)
}

fn solve_second_part(input: &str) -> u64 {
    blinks(input, 75)
}

fn blinks(input: &str, n: u32) -> u64 {
    let mut stones = input
        .split_whitespace()
        .map(|n| (n.parse().unwrap(), 1))
        .collect::<HashMap<u64, u64>>();

    for _ in 0..n {
        stones = blink(&stones);
    }

    stones.values().sum()
}

fn blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result = HashMap::new();

    for (stone, count) in stones.iter().flat_map(|(&stone, &count)| {
        if stone == 0 {
            return vec![(1, count)];
        }

        let digits = stone.ilog10() + 1;

        if digits % 2 == 0 {
            vec![
                (stone / 10_u64.pow(digits / 2), count),
                (stone % 10_u64.pow(digits / 2), count),
            ]
        } else {
            vec![(stone * 2024, count)]
        }
    }) {
        *result.entry(stone).or_insert(0) += count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17
";

    #[test]
    fn test_first_part() {
        let answer = 55312;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 65601038650482;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(209412, 248967696501656);
}

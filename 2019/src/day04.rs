use std::{collections::HashMap, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let (from, to) = input.trim().split_once('-').unwrap();
    let from = from.parse::<u32>().unwrap();
    let to = to.parse::<u32>().unwrap();

    (from..=to)
        .filter(|password| is_valid_password(&password.to_string(), false))
        .count()
}

fn solve_second_part(input: &str) -> usize {
    let (from, to) = input.trim().split_once('-').unwrap();
    let from = from.parse::<u32>().unwrap();
    let to = to.parse::<u32>().unwrap();

    (from..=to)
        .filter(|password| is_valid_password(&password.to_string(), true))
        .count()
}

fn is_valid_password(password: &str, strict: bool) -> bool {
    if password.len() != 6 {
        return false;
    }

    for pair in password.chars().collect::<Vec<_>>().windows(2) {
        if pair[0].to_digit(10) > pair[1].to_digit(10) {
            return false;
        }
    }

    let mut counts = HashMap::new();
    for c in password.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts
        .values()
        .any(|&len| if strict { len == 2 } else { len >= 2 })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "171309-643603";

    #[test]
    fn test_first_part() {
        let answer = 1625;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password("111111", false));
        assert!(is_valid_password("122345", false));
        assert!(!is_valid_password("223450", false));
        assert!(!is_valid_password("123789", false));

        assert!(is_valid_password("112233", true));
        assert!(!is_valid_password("123444", true));
        assert!(is_valid_password("111122", true));
    }

    check_answers!(1625, 1111);
}

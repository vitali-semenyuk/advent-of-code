use std::{collections::HashSet, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u16 {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            let a = a.chars().collect::<HashSet<_>>();
            let b = b.chars().collect::<HashSet<_>>();

            let char = *a.intersection(&b).next().unwrap();
            get_priority(char)
        })
        .sum()
}

fn solve_second_part(input: &str) -> u16 {
    let binding: Vec<_> = input.lines().collect();
    binding
        .chunks(3)
        .map(|ch| {
            let hs = ch
                .iter()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap();
            let mut qwe = hs.iter();
            get_priority(*qwe.next().unwrap())
        })
        .sum()
}

fn get_priority(c: char) -> u16 {
    if c.is_lowercase() {
        c as u16 - 'a' as u16 + 1
    } else {
        c as u16 - 'A' as u16 + 27
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_first_part() {
        let answer = 157;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 70;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(8039, 2510);
}

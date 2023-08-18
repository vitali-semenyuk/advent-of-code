use std::{collections::HashSet, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.union(&b).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}

fn solve_second_part(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_first_part() {
        let answer = 11;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 6;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(6416, 3050);
}

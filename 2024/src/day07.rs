use std::fmt::Display;

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn is_correct(&self, use_concat: bool) -> bool {
        is_correct(
            self.result,
            &self.operands[1..],
            self.operands[0],
            use_concat,
        )
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (result, operands) = value.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let operands = operands
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self { result, operands }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u64 {
    solve_all(input, false)
}

fn solve_second_part(input: &str) -> u64 {
    solve_all(input, true)
}

fn solve_all(input: &str, use_concat: bool) -> u64 {
    input
        .lines()
        .map(Equation::from)
        .filter_map(|eq| {
            if eq.is_correct(use_concat) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum()
}

fn is_correct(target: u64, stack: &[u64], value: u64, use_concat: bool) -> bool {
    if stack.is_empty() {
        return value == target;
    }

    is_correct(target, &stack[1..], value + stack[0], use_concat)
        || is_correct(target, &stack[1..], value * stack[0], use_concat)
        || (use_concat
            && is_correct(
                target,
                &stack[1..],
                value * 10u64.pow(stack[0].ilog10() + 1) + stack[0],
                use_concat,
            ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_first_part() {
        let answer = 3749;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 11387;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(12940396350192, 106016735664498);
}

use std::fmt::Display;

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let mut parts = string.split_whitespace();
        let count = parts.nth(1).unwrap().parse().unwrap();
        let from = parts.nth(1).unwrap().parse().unwrap();
        let to = parts.nth(1).unwrap().parse().unwrap();

        Instruction { count, from, to }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let crates: Vec<_> = crates.lines().rev().collect();
    let (header, crates) = crates.split_at(1);

    let mut stacks = Vec::new();
    for i in (1..header[0].len()).step_by(4) {
        let mut stack = Vec::new();
        for crates_stack in crates {
            if let Some(char) = crates_stack.chars().nth(i) {
                if char != ' ' {
                    stack.push(char);
                }
            }
        }
        stacks.push(stack);
    }

    let instructions: Vec<_> = instructions.lines().map(|l| Instruction::from(l)).collect();

    for instruction in instructions {
        for _ in 0..instruction.count {
            let crat = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(crat)
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn solve_second_part(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let crates: Vec<_> = crates.lines().rev().collect();
    let (header, crates) = crates.split_at(1);

    let mut stacks = Vec::new();
    for i in (1..header[0].len()).step_by(4) {
        let mut stack = Vec::new();
        for crates_stack in crates {
            if let Some(char) = crates_stack.chars().nth(i) {
                if char != ' ' {
                    stack.push(char);
                }
            }
        }
        stacks.push(stack);
    }

    let instructions: Vec<_> = instructions.lines().map(|l| Instruction::from(l)).collect();

    for instruction in instructions {
        let source = &mut stacks[instruction.from - 1];
        let crates = source.split_off(source.len() - instruction.count);
        let destination = &mut stacks[instruction.to - 1];
        destination.extend_from_slice(&crates);
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_first_part() {
        let answer = "CMZ";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "MCD";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!("JDTMRWCQJ", "VHJDDCWRD");
}

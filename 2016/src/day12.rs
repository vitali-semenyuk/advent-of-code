use std::fmt::Display;

use crate::shared::computer::{parse_code, Computer, ExecutionResult, Integer, Register};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    match evaluate(input, 0) {
        Ok(a) => a,
        Err(error) => panic!("{error}"),
    }
}

fn solve_second_part(input: &str) -> i32 {
    match evaluate(input, 1) {
        Ok(a) => a,
        Err(error) => panic!("{error}"),
    }
}

fn evaluate(source_code: &str, c: Integer) -> ExecutionResult {
    let instructions = parse_code(source_code)?;

    let mut computer = Computer::new();
    computer.set_register(&Register::C, c);
    computer.evaluate(&instructions);

    Ok(computer.get_register(&Register::A))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
";

    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(318077, 9227731);
}

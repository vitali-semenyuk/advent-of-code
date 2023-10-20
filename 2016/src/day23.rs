use std::fmt::Display;

use crate::shared::computer::{parse_code, Computer, Integer, ParseError, Register};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    match evaluate(input, 7) {
        Ok(a) => a,
        Err(error) => panic!("{error}"),
    }
}

fn solve_second_part(input: &str) -> i32 {
    match evaluate(input, 12) {
        Ok(a) => a,
        Err(error) => panic!("{error}"),
    }
}

fn evaluate(source_code: &str, a: Integer) -> Result<Integer, ParseError> {
    let instructions = parse_code(source_code)?;

    let mut computer = Computer::new();
    computer.set_register(&Register::A, a);
    computer.evaluate(&instructions);

    Ok(computer.get_register(&Register::A))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
";

    #[test]
    fn test_first_part() {
        let answer = 3;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 3;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(11514, 479008074);
}

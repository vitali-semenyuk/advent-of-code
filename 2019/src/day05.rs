use std::fmt::Display;

use crate::shared::intcode::Intcode;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i64 {
    let mut intcode = Intcode::from(input);

    intcode.input(1);

    if let Err(err) = intcode.run() {
        panic!("Error! {err}")
    }

    *intcode.buffered_output().last().expect("No output")
}

fn solve_second_part(input: &str) -> i64 {
    let mut intcode = Intcode::from(input);

    intcode.input(5);

    if let Err(err) = intcode.run() {
        panic!("Error! {err}")
    }

    intcode.output().expect("No output")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,0,4,0,99
";

    #[test]
    fn test_first_part() {
        let answer = 1;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(13346482, 12111395);
}

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

    intcode.set(1, 12);
    intcode.set(2, 2);

    intcode.run().expect("Runtime error");

    intcode.get(0).unwrap()
}

fn solve_second_part(input: &str) -> i64 {
    let intcode = Intcode::from(input);

    let (noun, verb) = brute_force_solution(&intcode, 19690720);

    noun * 100 + verb
}

fn brute_force_solution(intcode: &Intcode, target: i64) -> (i64, i64) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = intcode.clone();

            intcode.set(1, noun);
            intcode.set(2, verb);

            intcode.run().expect("Runtime error");

            if intcode.get(0).unwrap() == target {
                return (noun, verb);
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,12,2,0,99,0,0,0,0,0,0,0,67
";

    #[test]
    fn test_first_part() {
        let answer = 69;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    check_answers!(3058646, 8976);
}

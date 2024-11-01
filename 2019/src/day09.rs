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
        panic!("{}", err);
    }

    intcode.output().expect("No output")
}

fn solve_second_part(input: &str) -> i64 {
    let mut intcode = Intcode::from(input);

    intcode.input(2);

    if let Err(err) = intcode.run() {
        panic!("{}", err);
    }

    intcode.output().expect("No output")
}

#[cfg(test)]
mod tests {
    use super::*;

    check_answers!(2171728567, 49815);
}

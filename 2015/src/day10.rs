use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(_input: &str) -> i32 {
    0
}

fn solve_second_part(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
";

    #[ignore]
    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(42, 42);
}

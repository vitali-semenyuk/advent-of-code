use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    0
}

fn solve_second_part(input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "
";

    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(0, solve_first_part("(())"));
        assert_eq!(0, solve_first_part("()()"));
        assert_eq!(3, solve_first_part("((("));
        assert_eq!(3, solve_first_part("(()(()("));
        assert_eq!(3, solve_first_part("))((((("));
        assert_eq!(-1, solve_first_part("())"));
        assert_eq!(-1, solve_first_part("))("));
        assert_eq!(-3, solve_first_part(")))"));
        assert_eq!(-3, solve_first_part(")())())"));
    }

    // #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(42, 42);
}

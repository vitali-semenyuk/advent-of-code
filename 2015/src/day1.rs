use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    input.chars().fold(0, step)
}

fn solve_second_part(input: &str) -> i32 {
    let mut floor = 0;

    for (index, instr) in input.chars().enumerate() {
        floor = step(floor, instr);

        if floor < 0 {
            return (index + 1) as i32;
        }
    }

    -1
}

fn step(floor: i32, instr: char) -> i32 {
    match instr {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => panic!("Unexpected value"),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_first_part() {
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

    #[test]
    fn test_second_part() {
        assert_eq!(1, solve_second_part(")"));
        assert_eq!(5, solve_second_part("()())"));
    }

    check_answers!(280, 1797);
}

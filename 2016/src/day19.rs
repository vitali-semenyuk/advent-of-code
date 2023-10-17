use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let number = input.strip_suffix('\n').unwrap().parse::<u32>().unwrap();

    let l = number - 2_u32.pow(number.ilog2());
    l * 2 + 1
}

fn solve_second_part(input: &str) -> i32 {
    let number = input.strip_suffix('\n').unwrap().parse::<i32>().unwrap();

    let pow = 3_i32.pow(number.ilog(3));
    let rest = number - pow;
    let result = rest + (rest - pow).max(0);

    if result > 0 {
        result
    } else {
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5
";

    #[test]
    fn test_first_part() {
        let answer = 3;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT));
        assert_eq!(5, solve_second_part("14\n"));
    }

    check_answers!(1841611, 1423634);
}

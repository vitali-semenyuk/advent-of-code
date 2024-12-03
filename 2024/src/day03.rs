use regex::Regex;
use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            c.extract::<2>()
                .1
                .map(|n| n.parse::<i32>().unwrap())
                .into_iter()
                .product::<i32>()
        })
        .sum()
}

fn solve_second_part(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut result = 0;
    let mut multi_enabled = true;
    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => multi_enabled = true,
            "don't()" => multi_enabled = false,
            _ => {
                if multi_enabled {
                    let a = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    result += a * b;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let answer = 161;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let answer = 48;

        assert_eq!(answer, solve_second_part(input))
    }

    check_answers!(192767529, 104083373);
}

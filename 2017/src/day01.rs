use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    captcha(input.trim_end(), false)
}

fn solve_second_part(input: &str) -> u32 {
    captcha(input.trim_end(), true)
}

fn captcha(string: &str, halfway_shift: bool) -> u32 {
    let shift = if halfway_shift { string.len() / 2 } else { 1 };
    let digits = string.chars().map(|c| c.to_digit(10).unwrap());

    digits
        .clone()
        .zip(digits.cycle().skip(shift))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "91212129
";

    #[test]
    fn test_first_part() {
        let answer = 9;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 6;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_captcha() {
        assert_eq!(captcha("1122", false), 3);
        assert_eq!(captcha("1111", false), 4);
        assert_eq!(captcha("1234", false), 0);
        assert_eq!(captcha("91212129", false), 9);

        assert_eq!(captcha("1212", true), 6);
        assert_eq!(captcha("1221", true), 0);
        assert_eq!(captcha("123425", true), 4);
        assert_eq!(captcha("123123", true), 12);
        assert_eq!(captcha("12131415", true), 4);
    }

    check_answers!(997, 1358);
}

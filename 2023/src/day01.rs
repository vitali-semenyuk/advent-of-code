use std::fmt::Display;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    input.lines().map(parse_string).sum()
}

fn solve_second_part(input: &str) -> u32 {
    input.lines().map(parse_string_advanced).sum()
}

fn parse_string(string: &str) -> u32 {
    let digits = string.chars().filter(|ch| ch.is_numeric());
    let first = digits.clone().next().unwrap().to_digit(10).unwrap();
    let last = digits.last().unwrap().to_digit(10).unwrap();

    first * 10 + last
}

fn parse_string_advanced(string: &str) -> u32 {
    let digits = string.chars().filter(|ch| ch.is_numeric());
    let first_digit = digits.clone().next();
    let last_digit = digits.last();

    let mut first = first_digit.map_or(0, |ch| ch.to_digit(10).unwrap());
    let mut last = last_digit.map_or(0, |ch| ch.to_digit(10).unwrap());
    let mut i_first = first_digit.map_or(999, |ch| string.find(ch).unwrap());
    let mut i_last = last_digit.map_or(0, |ch| string.rfind(ch).unwrap());

    for (i, digit) in DIGITS.iter().enumerate() {
        if let Some(index) = string.find(digit) {
            if index < i_first {
                i_first = index;
                first = i as u32 + 1;
            }
        }

        if let Some(index) = string.rfind(digit) {
            if index >= i_last {
                i_last = index;
                last = i as u32 + 1;
            }
        }
    }

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let answer = 142;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let answer = 281;

        assert_eq!(answer, solve_second_part(input))
    }

    #[test]
    fn test_parse_string_advanced() {
        assert_eq!(26, parse_string_advanced("26fmrrhhpthree6b"))
    }

    check_answers!(54388, 53515);
}

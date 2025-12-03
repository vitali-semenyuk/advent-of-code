use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u64 {
    input.lines().map(|b| calculate_joltage(b, 2)).sum()
}

fn solve_second_part(input: &str) -> u64 {
    input.lines().map(|b| calculate_joltage(b, 12)).sum()
}

fn calculate_joltage(battery: &str, n: usize) -> u64 {
    let mut result = 0;
    let cells = battery
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let len = battery.len();
    let mut start = 0;

    for i in 0..n {
        let left = n - i - 1;
        let end = len - left;
        let window = &cells[start..end];

        let max = window.iter().max().unwrap();
        result = result * 10 + max;

        start += window.iter().position(|c| c == max).unwrap() + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_first_part() {
        let answer = 357;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 3121910778619;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(16854, 167526011932478);
}

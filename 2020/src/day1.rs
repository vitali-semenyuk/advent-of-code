use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i64 {
    let numbers: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut result = 0;

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                result = numbers[i] * numbers[j];
                break;
            }
        }
    }

    result
}

fn solve_second_part(input: &str) -> i64 {
    let numbers: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut result = 0;

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    result = numbers[i] * numbers[j] * numbers[k];
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_first_part() {
        let answer = 514579;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 241861950;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(927684, 292093004);
}

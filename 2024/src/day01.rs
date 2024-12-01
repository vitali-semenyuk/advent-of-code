use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let (mut first, mut second) = get_arrays(input);

    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn solve_second_part(input: &str) -> i32 {
    let (first, second) = get_arrays(input);

    first
        .into_iter()
        .map(|a| second.iter().filter(|b| **b == a).count() as i32 * a)
        .sum()
}

fn get_arrays(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut first, mut second), cur| {
            let mut parts = cur.split_whitespace();
            first.push(parts.next().unwrap().parse().unwrap());
            second.push(parts.next().unwrap().parse().unwrap());
            (first, second)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_first_part() {
        let answer = 11;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 31;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(1320851, 26859182);
}

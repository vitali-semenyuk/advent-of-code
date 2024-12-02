use std::{collections::HashSet, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let report = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            is_safe(&report)
        })
        .filter(|x| *x)
        .count()
}

fn solve_second_part(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let report = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            is_safe_problem_dumpener(&report)
        })
        .filter(|x| *x)
        .count()
}

fn is_safe(report: &[i32]) -> bool {
    let diffs = report.windows(2).map(|w| w[0] - w[1]);
    diffs.clone().all(|d| (1..=3).contains(&d.abs()))
        && diffs.map(|d| d.signum()).collect::<HashSet<_>>().len() == 1
}

fn is_safe_problem_dumpener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut cl = report.to_vec();
        cl.remove(i);
        if is_safe(&cl) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_first_part() {
        let answer = 2;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 4;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[7, 6, 4, 2, 1,]));
        assert!(!is_safe(&[1, 2, 7, 8, 9,]));
        assert!(!is_safe(&[9, 7, 6, 2, 1,]));
        assert!(!is_safe(&[1, 3, 2, 4, 5,]));
        assert!(!is_safe(&[8, 6, 4, 4, 1,]));
        assert!(is_safe(&[1, 3, 6, 7, 9,]));
    }

    #[test]
    fn test_is_safe_problem_dumpener() {
        assert!(is_safe_problem_dumpener(&[7, 6, 4, 2, 1,]));
        assert!(!is_safe_problem_dumpener(&[1, 2, 7, 8, 9,]));
        assert!(!is_safe_problem_dumpener(&[9, 7, 6, 2, 1,]));
        assert!(is_safe_problem_dumpener(&[1, 3, 2, 4, 5,]));
        assert!(is_safe_problem_dumpener(&[8, 6, 4, 4, 1,]));
        assert!(is_safe_problem_dumpener(&[1, 3, 6, 7, 9,]));
    }

    check_answers!(639, 674);
}

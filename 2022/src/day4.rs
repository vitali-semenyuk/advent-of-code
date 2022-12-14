#[derive(Debug)]
struct Range {
    begin: u32,
    end: u32,
}

impl From<&str> for Range {
    fn from(string: &str) -> Self {
        let (begin, end) = string.split_once("-").unwrap();
        let begin = begin.parse().unwrap();
        let end = end.parse().unwrap();

        Range { begin, end }
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (Range::from(a), Range::from(b))
        })
        .filter(|(a, b)| {
            (a.begin <= b.begin && a.end >= b.end) || (b.begin <= a.begin && b.end >= a.end)
        })
        .count() as i64
}

fn solve_second_part(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (Range::from(a), Range::from(b))
        })
        .filter(|(a, b)| (a.begin <= b.end && b.begin <= a.end))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

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

    check_answers!(448, 794);
}

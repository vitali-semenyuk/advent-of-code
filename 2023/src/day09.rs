use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    parse(input)
        .into_iter()
        .map(|sequence| extrapolate(&sequence, false))
        .sum()
}

fn solve_second_part(input: &str) -> i32 {
    parse(input)
        .into_iter()
        .map(|sequence| extrapolate(&sequence, true))
        .sum()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(sequence: &[i32], backwards: bool) -> i32 {
    let diffs = get_diffs(sequence);

    if diffs.iter().all(|n| *n == 0) {
        return *sequence.first().unwrap();
    }

    let extrapolated = extrapolate(&diffs, backwards);

    if backwards {
        sequence.first().unwrap() - extrapolated
    } else {
        sequence.last().unwrap() + extrapolated
    }
}

fn get_diffs(sequence: &[i32]) -> Vec<i32> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_first_part() {
        let answer = 114;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(2101499000, 1089);
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    get_calories(input).max().unwrap() as i64
}

fn solve_second_part(input: &str) -> i64 {
    let mut calories: Vec<_> = get_calories(input).collect();

    calories.sort();
    calories.reverse();

    calories[..3].iter().sum::<u32>() as i64
}

fn get_calories(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_first_part() {
        let answer = 24000;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 45000;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(72478, 210367);
}

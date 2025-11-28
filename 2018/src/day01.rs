use std::{collections::HashSet, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    input.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

fn solve_second_part(input: &str) -> i32 {
    let mut results = HashSet::new();
    let mut sum = 0;

    results.insert(sum);

    for num in input.lines().map(|l| l.parse::<i32>().unwrap()).cycle() {
        sum += num;

        if !results.insert(sum) {
            return sum;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "+1
-2
+3
+1
";

    #[test]
    fn test_first_part() {
        let answer = 3;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(547, 76414);
}

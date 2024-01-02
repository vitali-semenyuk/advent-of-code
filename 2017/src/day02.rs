use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let row = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap());
            let min = row.clone().min().unwrap();
            let max = row.max().unwrap();
            max - min
        })
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let row = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let length = row.len();

            (0..length - 1)
                .flat_map(|i| (i + 1..length).map(move |j| (i, j)))
                .filter_map(|(i, j)| {
                    let a = row[i].max(row[j]);
                    let b = row[i].min(row[j]);

                    if a % b == 0 {
                        Some(a / b)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "5 1 9 5
7 5 3
2 4 6 8
";
        let answer = 18;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part() {
        let input = "5 9 2 8
9 4 7 3
3 8 6 5
";
        let answer = 9;

        assert_eq!(answer, solve_second_part(input))
    }

    check_answers!(46402, 265);
}

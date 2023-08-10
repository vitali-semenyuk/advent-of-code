use std::{collections::HashMap, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let attempts = input.lines().collect::<Vec<_>>();

    restore_string(&attempts, false)
}

fn solve_second_part(input: &str) -> String {
    let attempts = input.lines().collect::<Vec<_>>();

    restore_string(&attempts, true)
}

fn restore_string(attempts: &Vec<&str>, least: bool) -> String {
    let length = attempts[0].len();

    let transposed: Vec<Vec<_>> = (0..length)
        .map(|i| {
            attempts
                .iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect()
        })
        .collect();

    transposed
        .into_iter()
        .map(|seq| {
            let frequences = seq.into_iter().fold(HashMap::new(), |mut acc, char| {
                let count = acc.get(&char).unwrap_or(&0);
                acc.insert(char, count + 1);
                acc
            });
            let mut frequences = frequences.into_iter().collect::<Vec<_>>();
            frequences.sort_by(|a, b| a.1.cmp(&b.1));
            if !least {
                frequences.reverse();
            }
            frequences.first().unwrap().0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";

    #[test]
    fn test_first_part() {
        let answer = "easter";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "advent";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!("umcvzsmw", "rwqoacfz");
}

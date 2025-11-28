use std::{collections::HashMap, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let ids = input.lines().map(|line| {
        line.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
    });

    let doubles = ids
        .clone()
        .filter(|id| id.values().any(|v| *v == 2))
        .count();
    let triples = ids
        .clone()
        .filter(|id| id.values().any(|v| *v == 3))
        .count();

    doubles * triples
}

fn solve_second_part(input: &str) -> String {
    let ids = input.lines();

    for (index, id) in ids.clone().enumerate() {
        for other in ids.clone().skip(index + 1) {
            if diff(id, other) == 1 {
                return common_string(id, other);
            }
        }
    }

    unreachable!()
}

fn common_string(str1: &str, str2: &str) -> String {
    str1.chars()
        .zip(str2.chars())
        .filter(|(a, b)| a == b)
        .map(|(s, _)| s)
        .collect()
}

fn diff(str1: &str, str2: &str) -> usize {
    str1.chars()
        .zip(str2.chars())
        .filter(|(a, b)| a != b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
        let answer = 12;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        let answer = "fgij";

        assert_eq!(answer, solve_second_part(input))
    }

    check_answers!(7410, "cnjxoritzhvbosyewrmqhgkul");
}

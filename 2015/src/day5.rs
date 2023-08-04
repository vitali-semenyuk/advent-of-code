use std::fmt::Display;

const VOWELS: &str = "aeiou";
const RESTRICTED: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input.lines().filter(|line| is_nice_legacy(line)).count()
}

fn solve_second_part(input: &str) -> usize {
    input.lines().filter(|line| is_nice(line)).count()
}

fn is_nice_legacy(string: &str) -> bool {
    check_vowels(string) && check_double(string) && check_restricted(string)
}

fn is_nice(string: &str) -> bool {
    check_double_pair(string) && check_tripple(string)
}

fn check_vowels(string: &str) -> bool {
    string.chars().filter(|&c| VOWELS.contains(c)).count() >= 3
}

fn check_double(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|chars| chars[0] == chars[1])
}

fn check_restricted(string: &str) -> bool {
    RESTRICTED
        .iter()
        .all(|restricted| !string.contains(restricted))
}

fn check_double_pair(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .enumerate()
        .any(|(index, chars)| {
            let pair: String = chars.iter().collect();
            string[index + 2..].contains(&pair)
        })
}

fn check_tripple(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|chars| chars[0] == chars[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        const INPUT: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb
";
        let answer = 2;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        const INPUT: &str = "
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
";
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_is_nice_legacy() {
        assert!(is_nice_legacy("ugknbfddgicrmopn"));
        assert!(is_nice_legacy("aaa"));
        assert!(!is_nice_legacy("jchzalrnumimnmhp"));
        assert!(!is_nice_legacy("haegwjzuvuyypxyu"));
        assert!(!is_nice_legacy("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_is_nice() {
        assert!(is_nice("qjhvhtzxzqqjkmpb"));
        assert!(is_nice("xxyxx"));
        assert!(!is_nice("uurcxstgmygtbstg"));
        assert!(!is_nice("ieodomkazucvgmuy"));
    }

    check_answers!(236, 51);
}

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
        .filter(|passphrase| is_valid_passphrase(passphrase))
        .count()
}

fn solve_second_part(input: &str) -> usize {
    input
        .lines()
        .filter(|passphrase| is_valid_passphrase_v2(passphrase))
        .count()
}

fn is_valid_passphrase(passphrase: &str) -> bool {
    let mut words = HashSet::new();

    for word in passphrase.split_ascii_whitespace() {
        if words.contains(word) {
            return false;
        }

        words.insert(word);
    }

    true
}

fn is_valid_passphrase_v2(passphrase: &str) -> bool {
    let mut words = HashSet::new();

    for word in passphrase.split_ascii_whitespace() {
        let mut chars = word.chars().collect::<Vec<_>>();
        chars.sort();

        if words.contains(&chars) {
            return false;
        }

        words.insert(chars);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa
";
        let answer = 2;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part() {
        let input = "abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio
";
        let answer = 3;

        assert_eq!(answer, solve_second_part(input))
    }

    #[test]
    fn test_is_valid_passphrase() {
        assert!(is_valid_passphrase("aa bb cc dd ee"));
        assert!(!is_valid_passphrase("aa bb cc dd aa"));
        assert!(is_valid_passphrase("aa bb cc dd aaa"));
    }

    #[test]
    fn test_is_valid_passphrase_v2() {
        assert!(is_valid_passphrase_v2("abcde fghij"));
        assert!(!is_valid_passphrase_v2("abcde xyz ecdab"));
        assert!(is_valid_passphrase_v2("a ab abc abd abf abj"));
        assert!(is_valid_passphrase_v2("iiii oiii ooii oooi oooo"));
        assert!(!is_valid_passphrase_v2("oiii ioii iioi iiio"));
    }

    check_answers!(455, 186);
}

use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
struct Password(u64);

impl Password {
    fn new(string: &str) -> Self {
        let value = string
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, char)| {
                acc + ((char as u64) - ('a' as u64)) * 26u64.pow(index as u32)
            });

        Password(value)
    }

    fn succ(&mut self) {
        self.0 += 1
    }

    fn to_string(&self) -> String {
        let mut value = self.0;
        let mut result = Vec::new();

        loop {
            let modulo = value % 26;
            value /= 26;

            result.push((modulo as u8 + 'a' as u8) as char);

            if value == 0 {
                break;
            }
        }

        result.iter().rev().collect()
    }
}

const RESTRICTED: [char; 3] = ['i', 'l', 'o'];

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let password = input.strip_suffix("\n").unwrap();

    get_next_password(password)
}

fn solve_second_part(input: &str) -> String {
    let password = input.strip_suffix("\n").unwrap();

    let new_password = get_next_password(password);
    get_next_password(&new_password)
}

fn get_next_password(password: &str) -> String {
    let mut password = Password::new(password);

    loop {
        password.succ();

        if validate_password(&password.to_string()) {
            break;
        }
    }

    password.to_string()
}

fn validate_password(password: &str) -> bool {
    check_straight(password) && check_restricted(password) && check_pairs(password)
}

fn check_straight(password: &str) -> bool {
    password
        .bytes()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|chars| chars[1] as i8 - chars[0] as i8 == 1 && chars[2] as i8 - chars[1] as i8 == 1)
}

fn check_restricted(password: &str) -> bool {
    RESTRICTED.iter().all(|&char| !password.contains(char))
}

fn check_pairs(password: &str) -> bool {
    let chars = password.chars().collect::<Vec<_>>();
    let mut pairs = chars.windows(2);
    let mut set = HashSet::new();

    while let Some(pair) = pairs.next() {
        if pair[0] != pair[1] {
            continue;
        }

        set.insert(pair);

        if set.len() == 2 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ghijklmn
";

    #[test]
    fn test_first_part() {
        let answer = "ghjaabcc";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "ghjbbcdd";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_get_next_password() {
        // assert_eq!(get_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(get_next_password("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn test_validate_password() {
        assert!(!validate_password("hijklmmn"));
        assert!(!validate_password("abbceffg"));
        assert!(!validate_password("abbcegjk"));
        assert!(!validate_password("abcdefgh"));
        assert!(validate_password("abcdffaa"));
        assert!(!validate_password("ghijklmn"));
        assert!(validate_password("ghjaabcc"));
    }

    #[test]
    fn test_check_straight() {
        assert!(check_straight("hijklmmn"));
        assert!(!check_straight("abbceffg"));
    }

    #[test]
    fn test_check_restricted() {
        assert!(check_restricted("abbceffg"));
        assert!(!check_restricted("hijklmmn"));
    }

    #[test]
    fn test_check_pairs() {
        assert!(check_pairs("abbceffg"));
        assert!(!check_pairs("abbcegjk"));
    }

    check_answers!("hepxxyzz", "heqaabcc");
}

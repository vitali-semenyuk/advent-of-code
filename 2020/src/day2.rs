use std::fmt::Display;

#[derive(Debug)]
struct Password {
    password: String,
    char: char,
    a: u32,
    b: u32,
}

impl Password {
    fn is_valid_legacy(&self) -> bool {
        let count = self.password.matches(self.char).count() as u32;
        (self.a..=self.b).contains(&count)
    }

    fn is_valid(&self) -> bool {
        let a = self.a as usize - 1;
        let b = self.b as usize - 1;
        (self.password.chars().nth(a).unwrap() == self.char)
            ^ (self.password.chars().nth(b).unwrap() == self.char)
    }
}

impl From<&str> for Password {
    fn from(s: &str) -> Self {
        let (policy, password) = s.split_once(':').unwrap();
        let (range, rule) = policy.split_once(' ').unwrap();
        let (min_length, max_length) = range.split_once('-').unwrap();
        let password = password.trim().to_string();
        let char = rule.chars().next().unwrap();
        let min_length: u32 = min_length.parse().unwrap();
        let max_length: u32 = max_length.parse().unwrap();

        Password {
            password,
            char,
            a: min_length,
            b: max_length,
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(|p| p.is_valid_legacy())
        .count()
}

fn solve_second_part(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(|p| p.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    #[test]
    fn test_first_part() {
        let answer = 2;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 1;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(445, 491);
}

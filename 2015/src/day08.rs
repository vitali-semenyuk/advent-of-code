use std::fmt::Display;

#[derive(Debug)]
struct DoubleQuotedString(String);

impl From<&str> for DoubleQuotedString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

const INVALID_STRING: &str = "Invalid string";

impl DoubleQuotedString {
    fn decode(&self) -> String {
        let mut chars = self.0.chars();
        let mut string = String::new();

        let open_quote = chars.next().expect(INVALID_STRING);
        if open_quote != '"' {
            panic!("{INVALID_STRING}")
        }

        while let Some(char) = chars.next() {
            let ch = if char == '\\' {
                let escaped = chars.next().expect(INVALID_STRING);
                match escaped {
                    '\\' | '"' => escaped,
                    'x' => {
                        let hi = chars.next().expect(INVALID_STRING);
                        let lo = chars.next().expect(INVALID_STRING);
                        let hex = [hi, lo].iter().collect::<String>();
                        let hex = u8::from_str_radix(&hex, 16).expect(INVALID_STRING);

                        if hex.is_ascii() {
                            hex as char
                        } else {
                            '?'
                        }
                    }
                    _ => panic!("{INVALID_STRING}"),
                }
            } else {
                char
            };
            string.push(ch);
        }

        let close_quote = string.pop().expect(INVALID_STRING);
        if close_quote != '"' {
            panic!("{INVALID_STRING}")
        }

        string
    }

    fn encode(&self) -> String {
        let mut string = String::from("\"");

        for char in self.0.chars() {
            match char {
                '\\' | '"' => string.extend(['\\', char]),
                _ => string.push(char),
            }
        }

        string.push('\"');

        string
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn decoded_len(&self) -> usize {
        self.decode().len()
    }

    fn encoded_len(&self) -> usize {
        self.encode().len()
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
        .map(DoubleQuotedString::from)
        .map(|s| s.len() - s.decoded_len())
        .sum()
}

fn solve_second_part(input: &str) -> usize {
    input
        .lines()
        .map(DoubleQuotedString::from)
        .map(|s| s.encoded_len() - s.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"""
"abc"
"aaa\"aaa"
"\x27"
"#;

    #[test]
    fn test_first_part() {
        let answer = 12;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 19;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_len() {
        assert_eq!(2, DoubleQuotedString::from(r#""""#).len());
        assert_eq!(5, DoubleQuotedString::from(r#""abc""#).len());
        assert_eq!(10, DoubleQuotedString::from(r#""aaa\"aaa""#).len());
        assert_eq!(6, DoubleQuotedString::from(r#""\x27""#).len());
    }

    #[test]
    fn test_decoded_len() {
        assert_eq!(0, DoubleQuotedString::from(r#""""#).decoded_len());
        assert_eq!(3, DoubleQuotedString::from(r#""abc""#).decoded_len());
        assert_eq!(7, DoubleQuotedString::from(r#""aaa\"aaa""#).decoded_len());
        assert_eq!(1, DoubleQuotedString::from(r#""\x27""#).decoded_len());
    }

    #[test]
    fn test_encoded_len() {
        assert_eq!(6, DoubleQuotedString::from(r#""""#).encoded_len());
        assert_eq!(9, DoubleQuotedString::from(r#""abc""#).encoded_len());
        assert_eq!(16, DoubleQuotedString::from(r#""aaa\"aaa""#).encoded_len());
        assert_eq!(11, DoubleQuotedString::from(r#""\x27""#).encoded_len());
    }

    check_answers!(1350, 2085);
}

use std::fmt::Display;

#[derive(Debug)]
struct Marker(usize, usize);

impl From<String> for Marker {
    fn from(value: String) -> Self {
        let (range, count) = value.split_once("x").unwrap();
        Self(range.parse().unwrap(), count.parse().unwrap())
    }
}

#[derive(Debug)]
enum Token {
    Literal(String),
    Marker(Vec<Token>, usize),
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        let tokens = parse(value);
        Self::Marker(tokens, 1)
    }
}

impl Token {
    fn length(&self) -> usize {
        match self {
            Token::Literal(string) => string.len(),
            Token::Marker(tokens, count) => tokens.iter().map(Token::length).sum::<usize>() * count,
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
    let input = input.strip_suffix("\n").unwrap();
    decompress(input).len()
}

fn solve_second_part(input: &str) -> usize {
    let input = input.strip_suffix("\n").unwrap();
    decompress_v2_len(input)
}

fn decompress(data: &str) -> String {
    let mut chars = data.chars();
    let mut result = String::new();

    while let Some(char) = chars.next() {
        if char != '(' {
            result.push(char);
            continue;
        }

        let marker: Marker = chars
            .by_ref()
            .take_while(|ch| *ch != ')')
            .collect::<String>()
            .into();

        let sequence = chars.by_ref().take(marker.0).collect::<String>();
        let sequence = sequence.repeat(marker.1);
        result.push_str(&sequence);
    }

    result
}

fn decompress_v2_len(data: &str) -> usize {
    Token::from(data).length()
}

fn parse(data: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut chars = data.chars();
    let mut current_literal = String::new();

    while let Some(char) = chars.next() {
        if char != '(' {
            current_literal.push(char);
            continue;
        }

        if !current_literal.is_empty() {
            result.push(Token::Literal(current_literal.clone()));
            current_literal.clear();
        }

        let marker: Marker = chars
            .by_ref()
            .take_while(|ch| *ch != ')')
            .collect::<String>()
            .into();

        let sequence = chars.by_ref().take(marker.0).collect::<String>();
        result.push(Token::Marker(parse(&sequence), marker.1))
    }

    if !current_literal.is_empty() {
        result.push(Token::Literal(current_literal))
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "X(8x2)(3x3)ABCY
";

    #[test]
    fn test_first_part() {
        let answer = 18;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 20;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_decompress() {
        assert_eq!(decompress("ADVENT"), "ADVENT");
        assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
        assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }

    #[test]
    fn test_decompress_v2() {
        assert_eq!(decompress_v2_len("(3x3)XYZ"), 9);
        assert_eq!(decompress_v2_len("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(
            decompress_v2_len("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
        assert_eq!(
            decompress_v2_len("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }

    check_answers!(120765, 11658395076);
}

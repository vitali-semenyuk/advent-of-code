use std::fmt::Display;

#[derive(Debug, Clone)]
struct Number {
    x: usize,
    y: usize,
    value: String,
}

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
    value: char,
}

impl Number {
    fn new(first_digit: char, x: usize, y: usize) -> Self {
        Self {
            value: String::from(first_digit),
            x,
            y,
        }
    }

    fn append_digit(&mut self, digit: char) {
        self.value.push(digit)
    }

    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        let x_start = if self.x > 0 { self.x - 1 } else { 0 };
        let x_end = self.x + self.value.len();

        self.y.abs_diff(symbol.y) <= 1 && (x_start..=x_end).contains(&symbol.x)
    }

    fn get_value(&self) -> u32 {
        self.value.parse().unwrap()
    }
}

impl Symbol {
    fn new(value: char, x: usize, y: usize) -> Self {
        Self { value, x, y }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let (numbers, symbols) = parse_grid(input);

    numbers
        .iter()
        .filter(|num| symbols.iter().any(|sym| num.is_adjacent(sym)))
        .map(|num| num.get_value())
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
    let (numbers, symbols) = parse_grid(input);

    symbols
        .iter()
        .filter(|sym| sym.value == '*')
        .filter_map(|sym| {
            let adjacent = numbers.iter().filter(|num| num.is_adjacent(sym));

            if adjacent.clone().count() != 2 {
                return None;
            }

            Some(adjacent.map(|num| num.get_value()).product::<u32>())
        })
        .sum()
}

fn parse_grid(grid: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut current_number: Option<Number> = None;

    for (y, line) in grid.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if let Some(ref mut number) = current_number {
                if char.is_ascii_digit() {
                    number.append_digit(char)
                } else {
                    numbers.push(number.clone());
                    current_number = None;

                    if char != '.' {
                        symbols.push(Symbol::new(char, x, y))
                    }
                }
            } else {
                if char.is_ascii_digit() {
                    current_number = Some(Number::new(char, x, y))
                } else if char != '.' {
                    symbols.push(Symbol::new(char, x, y))
                }
            }
        }
    }

    (numbers, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_first_part() {
        let answer = 4361;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 467835;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(539433, 75847567);
}

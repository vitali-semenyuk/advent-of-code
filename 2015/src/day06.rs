use std::fmt::Display;

#[derive(Debug)]
enum CommandType {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Region {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    region: Region,
}

const ERROR_MESSAGE: &str = "Invalid command";

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        let command_type = match parts.next().expect(ERROR_MESSAGE) {
            "turn" => match parts.next().expect(ERROR_MESSAGE) {
                "on" => CommandType::TurnOn,
                "off" => CommandType::TurnOff,
                _ => panic!("{ERROR_MESSAGE}"),
            },
            "toggle" => CommandType::Toggle,
            _ => panic!("{ERROR_MESSAGE}"),
        };
        let start = parts.next().expect(ERROR_MESSAGE);
        parts.next();
        let end = parts.next().expect(ERROR_MESSAGE);

        let (x1, y1) = start.split_once(',').expect(ERROR_MESSAGE);
        let (x2, y2) = end.split_once(',').expect(ERROR_MESSAGE);
        let x1 = x1.parse().expect(ERROR_MESSAGE);
        let y1 = y1.parse().expect(ERROR_MESSAGE);
        let x2 = x2.parse().expect(ERROR_MESSAGE);
        let y2 = y2.parse().expect(ERROR_MESSAGE);

        let region = Region { x1, x2, y1, y2 };

        Self {
            command_type,
            region,
        }
    }
}

trait Matrix {
    fn turn_on(&mut self, region: &Region);
    fn turn_off(&mut self, region: &Region);
    fn toggle(&mut self, region: &Region);
}

struct LegacyMatrix(Vec<Vec<bool>>);

impl LegacyMatrix {
    fn new() -> Self {
        Self(vec![vec![false; 1000]; 1000])
    }
}

impl Matrix for LegacyMatrix {
    fn turn_on(&mut self, region: &Region) {
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                self.0[y][x] = true
            }
        }
    }

    fn turn_off(&mut self, region: &Region) {
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                self.0[y][x] = false
            }
        }
    }

    fn toggle(&mut self, region: &Region) {
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                self.0[y][x] = !self.0[y][x]
            }
        }
    }
}

struct NewMatrix(Vec<Vec<usize>>);

impl NewMatrix {
    fn new() -> Self {
        Self(vec![vec![0; 1000]; 1000])
    }
}

impl Matrix for NewMatrix {
    fn turn_on(&mut self, region: &Region) {
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                self.0[y][x] += 1
            }
        }
    }

    fn turn_off(&mut self, region: &Region) {
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                if self.0[y][x] > 0 {
                    self.0[y][x] -= 1
                }
            }
        }
    }

    fn toggle(&mut self, region: &Region) {
        for y in region.y1..=region.y2 {
            for x in region.x1..=region.x2 {
                self.0[y][x] += 2
            }
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
    let mut matrix = LegacyMatrix::new();

    for command in input.lines().map(Command::from) {
        match command.command_type {
            CommandType::TurnOn => matrix.turn_on(&command.region),
            CommandType::TurnOff => matrix.turn_off(&command.region),
            CommandType::Toggle => matrix.toggle(&command.region),
        }
    }

    matrix
        .0
        .iter()
        .map(|row| row.iter().filter(|el| **el).count())
        .sum()
}

fn solve_second_part(input: &str) -> usize {
    let mut matrix = NewMatrix::new();

    for command in input.lines().map(Command::from) {
        match command.command_type {
            CommandType::TurnOn => matrix.turn_on(&command.region),
            CommandType::TurnOff => matrix.turn_off(&command.region),
            CommandType::Toggle => matrix.toggle(&command.region),
        }
    }

    matrix.0.iter().map(|row| row.iter().sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        const INPUT: &str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
        let answer = 998_996;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        const INPUT: &str = "turn on 0,0 through 0,0
toggle 0,0 through 999,999
";
        let answer = 2_000_001;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(400410, 15343601);
}

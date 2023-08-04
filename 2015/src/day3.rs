use std::{collections::HashSet, fmt::Display};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Right,
            '<' => Self::Left,
            '^' => Self::Up,
            'v' => Self::Down,
            _ => panic!("Unexpected value"),
        }
    }
}

struct Santa {
    x: i32,
    y: i32,
}

impl Santa {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn turn(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let mut santa = Santa::new();
    let mut visited = HashSet::from([santa.get_position()]);

    for direction in input.chars().map(Direction::from) {
        santa.turn(&direction);
        visited.insert(santa.get_position());
    }

    visited.len()
}

fn solve_second_part(input: &str) -> usize {
    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();
    let mut visited = HashSet::from([santa.get_position()]);

    for directions in input
        .chars()
        .map(Direction::from)
        .collect::<Vec<_>>()
        .chunks(2)
    {
        santa.turn(&directions[0]);
        robo_santa.turn(&directions[1]);

        visited.insert(santa.get_position());
        visited.insert(robo_santa.get_position());
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(2, solve_first_part(">"));
        assert_eq!(4, solve_first_part("^>v<"));
        assert_eq!(2, solve_first_part("^v^v^v^v^v"));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(3, solve_second_part("^v"));
        assert_eq!(3, solve_second_part("^>v<"));
        assert_eq!(11, solve_second_part("^v^v^v^v^v"));
    }

    check_answers!(2081, 2341);
}

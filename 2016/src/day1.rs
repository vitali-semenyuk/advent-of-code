use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
struct Movement(Turn, u32);

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        let (turn, distance) = value.split_at(1);
        let turn = match turn {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => panic!("Invalid value"),
        };
        let distance = distance.parse().unwrap();

        Self(turn, distance)
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct Actor {
    x: i32,
    y: i32,
    direction: Direction,
    path: HashSet<(i32, i32)>,
}

impl Actor {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::North,
            path: HashSet::new(),
        }
    }

    fn step(&mut self, movement: Movement, with_repetitions: bool) -> bool {
        self.direction = match movement.0 {
            Turn::Left => self.direction.turn_left(),
            Turn::Right => self.direction.turn_right(),
        };

        match self.direction {
            Direction::North => {
                for _ in 0..movement.1 {
                    self.y -= 1;
                    if !self.path.insert(self.get_position()) && !with_repetitions {
                        return false;
                    }
                }
                true
            }
            Direction::East => {
                for _ in 0..movement.1 {
                    self.x += 1;
                    if !self.path.insert(self.get_position()) && !with_repetitions {
                        return false;
                    }
                }
                true
            }
            Direction::South => {
                for _ in 0..movement.1 {
                    self.y += 1;
                    if !self.path.insert(self.get_position()) && !with_repetitions {
                        return false;
                    }
                }
                true
            }
            Direction::West => {
                for _ in 0..movement.1 {
                    self.x -= 1;
                    if !self.path.insert(self.get_position()) && !with_repetitions {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn get_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
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

fn solve_first_part(input: &str) -> i32 {
    let input = input.lines().next().unwrap();
    let mut actor = Actor::new();

    for movement in input.split(", ").map(Movement::from) {
        actor.step(movement, true);
    }

    actor.get_distance()
}

fn solve_second_part(input: &str) -> i32 {
    let input = input.lines().next().unwrap();
    let mut actor = Actor::new();

    for movement in input.split(", ").map(Movement::from) {
        if !actor.step(movement, false) {
            break;
        }
    }

    actor.get_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(5, solve_first_part("R2, L3"));
        assert_eq!(2, solve_first_part("R2, R2, R2"));
        assert_eq!(12, solve_first_part("R5, L5, R5, R3"));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(4, solve_second_part("R8, R4, R4, R8"))
    }

    check_answers!(291, 159);
}

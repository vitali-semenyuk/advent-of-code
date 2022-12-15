use std::{collections::HashSet, fmt::Display};

#[derive(Debug)]
enum Motion {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl From<&str> for Motion {
    fn from(string: &str) -> Self {
        let (dir, len) = string.split_once(" ").unwrap();
        let len = len.parse().unwrap();

        match dir {
            "U" => Motion::Up(len),
            "D" => Motion::Down(len),
            "L" => Motion::Left(len),
            "R" => Motion::Right(len),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Point,
    tail: Point,
    visited: Vec<Point>,
}

impl Rope {
    fn new() -> Self {
        let head = Point { x: 0, y: 0 };
        let tail = Point { x: 0, y: 0 };

        Rope {
            head,
            tail,
            visited: Vec::new(),
        }
    }

    fn make_move(&mut self, motion: Motion) {
        match motion {
            Motion::Up(len) => {
                for _ in 0..len {
                    self.head.y += 1;
                    self.sync_tail()
                }
            }
            Motion::Down(len) => {
                for _ in 0..len {
                    self.head.y -= 1;
                    self.sync_tail()
                }
            }
            Motion::Left(len) => {
                for _ in 0..len {
                    self.head.x -= 1;
                    self.sync_tail()
                }
            }
            Motion::Right(len) => {
                for _ in 0..len {
                    self.head.x += 1;
                    self.sync_tail()
                }
            }
        }
    }

    fn sync_tail(&mut self) {
        let x_diff = self.head.x - self.tail.x;
        let y_diff = self.head.y - self.tail.y;

        if x_diff.abs() > 1 || y_diff.abs() > 1 {
            if x_diff == 0 {
                self.tail.y += y_diff.signum()
            } else if y_diff == 0 {
                self.tail.x += x_diff.signum()
            } else {
                self.tail.y += y_diff.signum();
                self.tail.x += x_diff.signum()
            }
        }

        self.visited.push(self.tail.clone());
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let motions: Vec<_> = input.lines().map(Motion::from).collect();

    let mut rope = Rope::new();

    for motion in motions {
        rope.make_move(motion);
    }

    HashSet::<Point>::from_iter(rope.visited.iter().cloned()).len()
}

fn solve_second_part(input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_first_part() {
        let answer = 13;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(1, solve_second_part(INPUT));
        assert_eq!(36, solve_second_part(input))
    }

    // check_answers!(5907, 42);
}

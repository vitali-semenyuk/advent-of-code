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
        let (dir, len) = string.split_once(' ').unwrap();
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
    knots: Vec<Point>,
    visited: Vec<Point>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let mut knots = Vec::new();
        for _ in 0..size {
            knots.push(Point { x: 0, y: 0 })
        }

        Rope {
            knots,
            visited: Vec::new(),
        }
    }

    fn make_move(&mut self, motion: Motion) {
        match motion {
            Motion::Up(len) => {
                for _ in 0..len {
                    self.knots[0].y += 1;
                    for i in 1..self.knots.len() {
                        self.sync_tail(i)
                    }
                    self.visited.push(self.knots.last().unwrap().clone());
                }
            }
            Motion::Down(len) => {
                for _ in 0..len {
                    self.knots[0].y -= 1;
                    for i in 1..self.knots.len() {
                        self.sync_tail(i)
                    }
                    self.visited.push(self.knots.last().unwrap().clone());
                }
            }
            Motion::Left(len) => {
                for _ in 0..len {
                    self.knots[0].x -= 1;
                    for i in 1..self.knots.len() {
                        self.sync_tail(i)
                    }
                    self.visited.push(self.knots.last().unwrap().clone());
                }
            }
            Motion::Right(len) => {
                for _ in 0..len {
                    self.knots[0].x += 1;
                    for i in 1..self.knots.len() {
                        self.sync_tail(i)
                    }
                    self.visited.push(self.knots.last().unwrap().clone());
                }
            }
        }
    }

    fn sync_tail(&mut self, index: usize) {
        let head = self.knots.get(index - 1).unwrap();
        let tail = self.knots.get(index).unwrap();
        let x_diff = head.x - tail.x;
        let y_diff = head.y - tail.y;

        let tail = self.knots.get_mut(index).unwrap();

        if x_diff.abs() > 1 || y_diff.abs() > 1 {
            if x_diff == 0 {
                tail.y += y_diff.signum()
            } else if y_diff == 0 {
                tail.x += x_diff.signum()
            } else {
                tail.y += y_diff.signum();
                tail.x += x_diff.signum()
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
    let motions: Vec<_> = input.lines().map(Motion::from).collect();

    let mut rope = Rope::new(2);

    for motion in motions {
        rope.make_move(motion);
    }

    HashSet::<Point>::from_iter(rope.visited.iter().cloned()).len()
}

fn solve_second_part(input: &str) -> usize {
    let motions: Vec<_> = input.lines().map(Motion::from).collect();

    let mut rope = Rope::new(10);

    for motion in motions {
        rope.make_move(motion);
    }

    HashSet::<Point>::from_iter(rope.visited.iter().cloned()).len()
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

    check_answers!(5907, 2303);
}

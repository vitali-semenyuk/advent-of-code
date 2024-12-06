use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    obstructions: HashSet<Point>,
    position: Point,
    direction: Direction,
}

impl Map {
    fn run(&mut self) -> Option<HashSet<Point>> {
        let mut visited = HashSet::new();
        let mut visited_with_direction = HashSet::new();

        loop {
            visited.insert(self.position);
            visited_with_direction.insert((self.position, self.direction));

            let mut new_x = self.position.x as i32;
            let mut new_y = self.position.y as i32;

            match self.direction {
                Direction::Up => new_y -= 1,
                Direction::Down => new_y += 1,
                Direction::Left => new_x -= 1,
                Direction::Right => new_x += 1,
            }

            if new_x < 0 || new_x >= self.width as i32 || new_y < 0 || new_y >= self.height as i32 {
                return Some(visited);
            }

            let new_position = Point {
                x: new_x as usize,
                y: new_y as usize,
            };

            if self.obstructions.contains(&new_position) {
                self.direction = self.direction.turn_right();
            } else {
                self.position = new_position;
            }

            if visited_with_direction.contains(&(self.position, self.direction)) {
                return None;
            }
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let lines = value.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().chars().count();

        let mut obstructions = HashSet::new();
        let mut position = Point { x: 0, y: 0 };
        let direction = Direction::Up;

        for (y, line) in lines.enumerate() {
            for (x, cell) in line.chars().enumerate() {
                let point = Point { x, y };

                match cell {
                    '.' => (),
                    '#' => {
                        obstructions.insert(point);
                    }
                    '^' => {
                        position = point;
                    }
                    _ => unreachable!(),
                }
            }
        }

        Self {
            width,
            height,
            obstructions,
            position,
            direction,
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
    Map::from(input).run().unwrap().len()
}

fn solve_second_part(input: &str) -> i32 {
    let mut count = 0;
    let map = Map::from(input);

    for point in map.clone().run().unwrap() {
        if point == map.position {
            continue;
        }

        let mut m = map.clone();
        if m.obstructions.insert(point) && m.run().is_none() {
            count += 1
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_first_part() {
        let answer = 41;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 6;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(5080, 1919);
}

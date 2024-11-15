use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use crate::shared::intcode::Intcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Field {
    scaffold: HashSet<Point>,
    width: u32,
    height: u32,
    position: Point,
    intersections: Vec<Point>,
}

impl Field {
    fn new(input: &str) -> Self {
        let mut intcode = Intcode::from(input);
        intcode.run().expect("Runtime error");
        let output = intcode.buffered_output();

        let mut scaffold = HashSet::new();
        let mut position = None;

        let mut y = 0;
        let mut x = 0;
        for out in output {
            match (out as u8) as char {
                '#' => {
                    scaffold.insert(Point { x, y });
                }
                '.' => (),
                '\n' => {
                    x = 0;
                    y += 1;
                    continue;
                }
                '^' | 'v' | '<' | '>' => {
                    let point = Point { x, y };
                    scaffold.insert(point);
                    position = Some(point);
                }
                _ => unreachable!(),
            };
            x += 1;
        }

        let points = scaffold.clone().into_iter().collect::<Vec<_>>();
        let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
        let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;

        let width = max_x + 1;
        let height = max_y + 1;

        Self {
            scaffold,
            width,
            height,
            position: position.unwrap(),
            intersections: Vec::new(),
        }
    }

    fn draw(&self) {
        let mut canvas = vec![vec!['.'; self.width as usize]; self.height as usize];

        for point in &self.scaffold {
            let x = point.x;
            let y = point.y;
            canvas[y as usize][x as usize] = '█';
        }

        for point in &self.intersections {
            let x = point.x;
            let y = point.y;
            canvas[y as usize][x as usize] = 'X';
        }

        canvas[self.position.y as usize][self.position.x as usize] = '░';

        let image = canvas
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}", image);
    }

    fn find_intersections(&self) -> Vec<Point> {
        let mut result = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(self.position);
        let mut visited = HashSet::new();

        while let Some(point) = queue.pop_front() {
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            let adjacent = self
                .get_adjacent(point)
                .into_iter()
                .filter(|p| self.scaffold.contains(p));

            if adjacent.clone().count() == 4 {
                result.push(point);
            }

            for p in adjacent {
                queue.push_back(p);
            }
        }

        result
    }

    fn get_adjacent(&self, point: Point) -> Vec<Point> {
        let mut result = Vec::new();

        if point.x > 0 {
            result.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }

        if point.y > 0 {
            result.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }

        if point.x < self.width - 1 {
            result.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }

        if point.y < self.height - 1 {
            result.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }

        result
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let mut field = Field::new(input);

    let intersections = field.find_intersections();

    field.intersections = intersections.clone();
    dbg!(&field.intersections);

    field.draw();

    intersections.into_iter().map(|p| p.x * p.y).sum()
}

fn solve_second_part(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // check_answers!(3448, 42);
}

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    //thread,
    //time::Duration,
};

use crate::shared::intcode::{Intcode, RuntimeError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Solver {
    intcode: Intcode,
    position: Point,
    visited: HashSet<Point>,
    walls: HashSet<Point>,
    to_visit: HashSet<Point>,
    dead_ends: HashSet<Point>,
    result: Option<Point>,
}

impl Solver {
    fn new(intcode: Intcode) -> Self {
        Self {
            intcode,
            position: Point { x: 0, y: 0 },
            visited: HashSet::new(),
            walls: HashSet::new(),
            to_visit: HashSet::new(),
            dead_ends: HashSet::new(),
            result: None,
        }
    }

    fn explore(&mut self) {
        //thread::sleep(Duration::from_millis(10));
        //print!("{}[2J", 27 as char);
        //self.draw();

        if self.result.is_some() {
            return;
        }

        self.visited.insert(self.position);
        self.to_visit.remove(&self.position);

        for direction in 1..=4 {
            let position = Self::shift_position(self.position, direction);

            if self.visited.contains(&position) {
                continue;
            }
            if self.walls.contains(&position) {
                continue;
            }
            if self.dead_ends.contains(&position) {
                continue;
            }

            let output = self.run_intcode(direction);
            match output {
                0 => {
                    self.walls.insert(position);
                    self.to_visit.remove(&position);
                }
                1 => {
                    for i in (direction + 1)..=4 {
                        let p = Self::shift_position(self.position, i);
                        if !self.visited.contains(&p)
                            && !self.dead_ends.contains(&p)
                            && !self.walls.contains(&p)
                        {
                            self.run_intcode(Self::reverse(direction));
                            if self.check_wall(i) {
                                self.walls.insert(p);
                            } else {
                                self.to_visit.insert(p);
                            }
                            self.run_intcode(direction);
                        }
                    }
                    self.position = position;
                    self.explore();
                }
                2 => {
                    self.to_visit.remove(&position);
                    self.result = Some(position);
                }
                _ => unreachable!(),
            }
        }
    }

    fn check_wall(&mut self, direction: i64) -> bool {
        let output = self.run_intcode(direction);
        match output {
            0 => true,
            1 | 2 => {
                self.run_intcode(Self::reverse(direction));
                false
            }
            _ => unreachable!(),
        }
    }

    fn reverse(direction: i64) -> i64 {
        match direction {
            1 => 2,
            2 => 1,
            3 => 4,
            4 => 3,
            _ => unreachable!(),
        }
    }

    fn run_intcode(&mut self, input: i64) -> i64 {
        self.intcode.input(input);

        if let Err(err) = self.intcode.run() {
            match err {
                RuntimeError::MissingInput { ip: _ } => (),
                _ => panic!("{}", err),
            }
        }

        self.intcode.output().unwrap()
    }

    fn shift_position(mut position: Point, direction: i64) -> Point {
        match direction {
            1 => position.y -= 1,
            2 => position.y += 1,
            3 => position.x -= 1,
            4 => position.x += 1,
            _ => unreachable!(),
        };

        position
    }

    fn dead_end(&mut self) {
        self.visited.clear();
        self.dead_ends.insert(self.position);
    }

    fn get_space(&self, start: Point) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut visited = HashSet::new();

        let mut result = 0;
        while let Some((point, distance)) = queue.pop_front() {
            result = result.max(distance);
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            for dir in 1..=4 {
                let p = Self::shift_position(point, dir);

                if !self.walls.contains(&p) && !visited.contains(&p) {
                    queue.push_back((p, distance + 1));
                }
            }
        }

        result
    }

    fn get_distance(&self, start: Point, target: Point) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        let mut visited = HashSet::new();

        while let Some((point, distance)) = queue.pop_front() {
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            for dir in 1..=4 {
                let p = Self::shift_position(point, dir);

                if p == target {
                    return distance + 1;
                }

                if !self.walls.contains(&p) && !visited.contains(&p) {
                    queue.push_back((p, distance + 1));
                }
            }
        }

        unreachable!()
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let mut points = self.visited.clone().into_iter().collect::<Vec<_>>();
        points.append(&mut self.walls.clone().into_iter().collect());
        points.append(&mut self.to_visit.clone().into_iter().collect());
        points.push(self.position);

        let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
        let min_x = points.iter().min_by_key(|p| p.x).unwrap().x;
        let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;
        let min_y = points.iter().min_by_key(|p| p.y).unwrap().y;

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        let offset_x = -min_x;
        let offset_y = -min_y;

        let mut canvas = vec![vec![' '; width]; height];

        for point in &self.visited {
            let x = point.x + offset_x;
            let y = point.y + offset_y;
            canvas[y as usize][x as usize] = '◦';
        }

        for point in &self.walls {
            let x = point.x + offset_x;
            let y = point.y + offset_y;
            canvas[y as usize][x as usize] = '█';
        }

        for point in &self.dead_ends {
            let x = point.x + offset_x;
            let y = point.y + offset_y;
            canvas[y as usize][x as usize] = '·';
        }

        for point in &self.to_visit {
            let x = point.x + offset_x;
            let y = point.y + offset_y;
            canvas[y as usize][x as usize] = '░';
        }

        canvas[offset_y as usize][offset_x as usize] = 'o';
        canvas[(self.position.y + offset_y) as usize][(self.position.x + offset_x) as usize] = '*';
        if let Some(result) = self.result {
            canvas[(result.y + offset_y) as usize][(result.x + offset_x) as usize] = 'X';
        }

        let image = canvas
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}", image);
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let intcode = Intcode::from(input);
    let mut solver = Solver::new(intcode);

    for _ in 0..600 {
        solver.explore();
        if let Some(point) = solver.result {
            //solver.draw();
            return solver.get_distance(Point { x: 0, y: 0 }, point);
        }
        solver.dead_end();
    }

    unreachable!()
}

fn solve_second_part(input: &str) -> u32 {
    let intcode = Intcode::from(input);
    let mut solver = Solver::new(intcode);

    for _ in 0..600 {
        solver.explore();
        if let Some(point) = solver.result {
            //solver.draw();
            return solver.get_space(point);
        }
        solver.dead_end();
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    check_answers!(336, 360);
}

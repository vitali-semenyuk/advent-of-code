use std::{collections::HashSet, fmt::Display};

use crate::shared::intcode::{Intcode, RuntimeError};

type Point = (i32, i32);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
            Direction::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
            Direction::Right => Self::Down,
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
    paint(input, false).0.len()
}

fn solve_second_part(input: &str) -> String {
    let points = paint(input, true).1;

    let max_x = points.iter().max_by_key(|p| p.0).unwrap().0;
    let min_x = points.iter().min_by_key(|p| p.0).unwrap().0;
    let max_y = points.iter().max_by_key(|p| p.1).unwrap().1;
    let min_y = points.iter().min_by_key(|p| p.1).unwrap().1;

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let offset_x = -min_x;
    let offset_y = -min_y;

    let mut canvas = vec![vec![' '; width]; height];

    for (x, y) in points {
        let x = x + offset_x;
        let y = y + offset_y;
        canvas[y as usize][x as usize] = '0';
    }

    let number = canvas
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", number);

    "RFEPCFEB".to_string()
}

fn paint(code: &str, start_white: bool) -> (HashSet<Point>, HashSet<Point>) {
    let mut whites = HashSet::new();
    let mut painted = HashSet::new();
    let mut position = (0, 0);
    let mut direction = Direction::Up;

    if start_white {
        whites.insert(position);
    }

    let mut intcode = Intcode::from(code);

    loop {
        let color = if whites.contains(&position) { 1 } else { 0 };

        intcode.input(color);

        if let Err(err) = intcode.run() {
            match err {
                RuntimeError::MissingInput { ip: _ } => (),
                _ => panic!("{}", err),
            }
        }

        let color = intcode.output().expect("No output");
        let turn = intcode.output().expect("No output");

        painted.insert(position);
        if color == 1 {
            whites.insert(position);
        } else {
            whites.remove(&position);
        }
        if turn == 1 {
            direction = direction.turn_right();
        } else {
            direction = direction.turn_left();
        }

        match direction {
            Direction::Up => position.1 -= 1,
            Direction::Down => position.1 += 1,
            Direction::Left => position.0 -= 1,
            Direction::Right => position.0 += 1,
        }

        if intcode.is_halted() {
            break;
        }
    }

    (painted, whites)
}

#[cfg(test)]
mod tests {
    use super::*;

    check_answers!(1564, "RFEPCFEB");
}

use std::{collections::VecDeque, fmt::Display};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Cell {
    x: u32,
    y: u32,
}

impl Cell {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn get_neighbors(&self) -> Vec<Self> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets
            .map(|(x, y)| (self.x as i32 + x, self.y as i32 + y))
            .iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| Self::new(*x as u32, *y as u32))
            .collect()
    }

    fn is_wall(&self, seed: u32) -> bool {
        let Self { x, y } = self;
        let value = x * x + 3 * x + 2 * x * y + y + y * y + seed;
        let bits = (0..32).map(|i| (value >> i) & 1).filter(|x| *x > 0).count();

        bits % 2 != 0
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let seed = input.trim_end().parse().unwrap();
    let start = Cell::new(1, 1);
    let finish = Cell::new(31, 39);

    get_distance(start, finish, seed)
}

fn solve_second_part(input: &str) -> usize {
    let seed = input.trim_end().parse().unwrap();
    let start = Cell::new(1, 1);

    get_visited_count(start, 50, seed)
}

fn get_distance(start: Cell, finish: Cell, seed: u32) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();

    queue.push_back((start, 0));
    visited.push(start);

    while let Some((cell, distance)) = queue.pop_front() {
        for neighbor in cell.get_neighbors() {
            if neighbor == finish {
                return distance + 1;
            }

            if !visited.contains(&neighbor) && !neighbor.is_wall(seed) {
                queue.push_back((neighbor, distance + 1));
                visited.push(neighbor);
            }
        }
    }

    0
}

fn get_visited_count(start: Cell, limit: u32, seed: u32) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();

    queue.push_back((start, 0));
    visited.push(start);

    while let Some((cell, distance)) = queue.pop_front() {
        for neighbor in cell.get_neighbors() {
            if !visited.contains(&neighbor) && !neighbor.is_wall(seed) && distance < limit {
                queue.push_back((neighbor, distance + 1));
                visited.push(neighbor);
            }
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "10
";

    #[test]
    fn test_first_part() {
        let answer = 0;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 151;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_is_wall() {
        const SEED: u32 = 10;

        assert!(!Cell::new(0, 0).is_wall(SEED));
        assert!(Cell::new(1, 0).is_wall(SEED));
        assert!(!Cell::new(2, 0).is_wall(SEED));
        assert!(!Cell::new(0, 1).is_wall(SEED));
        assert!(!Cell::new(1, 1).is_wall(SEED));
        assert!(Cell::new(2, 1).is_wall(SEED));
    }

    #[test]
    fn test_get_distance() {
        assert_eq!(get_distance(Cell::new(1, 1), Cell::new(7, 4), 10), 11)
    }

    check_answers!(90, 135);
}

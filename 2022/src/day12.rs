use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

type Vertex = (usize, usize);

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let field: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let start_y = field.iter().position(|row| row.contains(&'S')).unwrap();
    let start_x = field[start_y].iter().position(|&char| char == 'S').unwrap();

    bfs(&field, (start_x, start_y))
}

fn solve_second_part(input: &str) -> i32 {
    let field: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut starts = Vec::new();

    for (y, row) in field.iter().enumerate() {
        for (x, &char) in row.iter().enumerate() {
            if char == 'S' || char == 'a' {
                starts.push((x, y));
            }
        }
    }

    starts
        .iter()
        .map(|&start| bfs(&field, start))
        .min()
        .unwrap()
}

fn bfs(field: &[Vec<char>], start: Vertex) -> i32 {
    let mut queue = VecDeque::from([start]);
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut distances = vec![vec![-1; field[0].len()]; field.len()];
    distances[start.1][start.0] = 0;

    while let Some(vertex @ (vx, vy)) = queue.pop_front() {
        if field[vy][vx] == 'E' {
            return distances[vy][vx];
        }

        for v in get_adjacent(field, vertex) {
            if visited.contains(&v) {
                continue;
            };

            distances[v.1][v.0] = distances[vy][vx] + 1;
            visited.insert(v);
            queue.push_back(v);
        }
    }

    999999999
}

fn get_adjacent(field: &[Vec<char>], (vx, vy): Vertex) -> Vec<Vertex> {
    let mut result = Vec::new();

    let current = field[vy][vx];
    if let Some(&destination) = field.get(vy).and_then(|row| row.get(vx + 1)) {
        if get_elevation(current, destination) <= 1 {
            result.push((vx + 1, vy));
        }
    }
    if vx > 0 {
        if let Some(&destination) = field.get(vy).and_then(|row| row.get(vx - 1)) {
            if get_elevation(current, destination) <= 1 {
                result.push((vx - 1, vy));
            }
        }
    }
    if let Some(&destination) = field.get(vy + 1).and_then(|row| row.get(vx)) {
        if get_elevation(current, destination) <= 1 {
            result.push((vx, vy + 1));
        }
    }
    if vy > 0 {
        if let Some(&destination) = field.get(vy - 1).and_then(|row| row.get(vx)) {
            if get_elevation(current, destination) <= 1 {
                result.push((vx, vy - 1));
            }
        }
    }

    result
}

fn get_elevation(current: char, destination: char) -> i8 {
    let current = if current == 'S' { 'a' } else { current };
    let destination = if destination == 'E' { 'z' } else { destination };
    destination as i8 - current as i8
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_first_part() {
        let answer = 31;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 29;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(504, 500);
}

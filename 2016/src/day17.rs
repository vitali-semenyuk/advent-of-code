use std::{fmt::Display, collections::VecDeque};

type Position = (u32, u32);

const FIELD_SIZE: u32 = 4;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(_input: &str) -> i32 {
    bfs((3, 3));

    0
}

fn solve_second_part(_input: &str) -> i32 {
    0
}

fn bfs(target: Position) {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some(position) = queue.pop_front() {
        if position == target {
            break;
        }

        let neighbors = get_neighbors(position);

        for neighbor in neighbors {
            queue.push_back(neighbor);
        }
    }
}

fn get_neighbors(position: Position) -> Vec<Position> {
    let mut neighbors = Vec::new();

    if position.0 > 0 {
        neighbors.push((position.0 - 1, position.1));
    }

    if position.1 > 0 {
        neighbors.push((position.0, position.1 - 1));
    }

    if position.0 < FIELD_SIZE - 1 {
        neighbors.push((position.0 + 1, position.1));
    }

    if position.1 < FIELD_SIZE - 1 {
        neighbors.push((position.0, position.1 + 1));
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
";

    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(42, 42);
}

use std::{collections::VecDeque, fmt::Display};

type Position = (i32, i32);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{}", str)
    }
}

const FIELD_SIZE: i32 = 4;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let passcode = input.trim_end();
    let path = bfs((3, 3), passcode, true);

    path.unwrap()
}

fn solve_second_part(input: &str) -> usize {
    let passcode = input.trim_end();
    let path = bfs((3, 3), passcode, false);

    path.unwrap().len()
}

fn bfs(target: Position, passcode: &str, short: bool) -> Option<String> {
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), String::new()));
    let mut pathes = Vec::new();

    while let Some((position, path)) = queue.pop_front() {
        if position == target {
            pathes.push(path.len());
            if short {
                return Some(path);
            } else {
                continue;
            }
        }

        let neighbors = get_neighbors(position);
        let open_directions = get_open_directions(&path, passcode);

        for (neighbor, direction) in neighbors {
            if !open_directions.contains(&direction) {
                continue;
            }

            let new_path = format!("{}{}", path, direction);
            queue.push_back((neighbor, new_path));
        }
    }

    pathes.iter().max().map(|&n| "a".repeat(n))
}

fn get_neighbors(position: Position) -> Vec<(Position, Direction)> {
    [
        (Direction::Left, (-1, 0)),
        (Direction::Up, (0, -1)),
        (Direction::Right, (1, 0)),
        (Direction::Down, (0, 1)),
    ]
    .map(|(direction, offset)| ((position.0 + offset.0, position.1 + offset.1), direction))
    .into_iter()
    .filter(|((x, y), _)| *x >= 0 && *y >= 0 && *x < FIELD_SIZE && *y < FIELD_SIZE)
    .collect()
}

fn get_open_directions(path: &str, passcode: &str) -> Vec<Direction> {
    let input = format!("{}{}", passcode, path);
    let hash = md5::compute(input);
    let hash = format!("{:x}", hash);
    let mut chars = hash.chars();

    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .filter(|_| {
        let char = chars.next().unwrap();
        !(char.is_ascii_digit() || char == 'a')
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!("DDRRRD", solve_first_part("ihgpwlah"));
        assert_eq!("DDUDRLRRUDRD", solve_first_part("kglvqrro"));
        assert_eq!(
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR",
            solve_first_part("ulqzkmiv")
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(370, solve_second_part("ihgpwlah"));
        assert_eq!(492, solve_second_part("kglvqrro"));
        assert_eq!(830, solve_second_part("ulqzkmiv"));
    }

    check_answers!("RRRLDRDUDD", 706);
}

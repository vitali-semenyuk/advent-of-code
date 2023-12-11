use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

type Position = (usize, usize);

const OFFSETS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

const PIPES: [[Pipe; 3]; 4] = [
    [Pipe::NorthSouth, Pipe::SouthEast, Pipe::SouthWest],
    [Pipe::NorthSouth, Pipe::NorthEast, Pipe::NorthWest],
    [Pipe::EastWest, Pipe::NorthEast, Pipe::SouthEast],
    [Pipe::EastWest, Pipe::NorthWest, Pipe::SouthWest],
];

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Unexpected symbol"),
        }
    }
}

#[derive(Debug)]
struct Field(Vec<Vec<Pipe>>);

impl Field {
    fn get_longest_path(&self) -> u32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start = self.find_start();
        let mut max_distance = 0;

        queue.push_back((start, 0));

        while let Some((pipe, distance)) = queue.pop_front() {
            if visited.contains(&pipe) {
                continue;
            }

            visited.insert(pipe);

            if distance > max_distance {
                max_distance = distance
            }

            for neighbor in self.get_neighbors(pipe) {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, distance + 1))
                }
            }
        }

        max_distance
    }

    fn find_start(&self) -> Position {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().position(|c| c == &Pipe::Start).map(|x| (x, y)))
            .next()
            .unwrap()
    }

    fn get_start_neighbors(&self, position: Position) -> Vec<Position> {
        let neighbors = OFFSETS
            .map(|offset| (position.0 as i32 + offset.0, position.1 as i32 + offset.1))
            .into_iter()
            .zip(PIPES)
            .filter_map(|((x, y), pipes)| {
                if x < 0 || y < 0 {
                    return None;
                }

                let x = x as usize;
                let y = y as usize;
                let pipe = self.0.get(y).and_then(|row| row.get(x));

                pipe.and_then(|p| {
                    if pipes.contains(p) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            });

        neighbors.collect()
    }

    fn get_neighbors(&self, position: Position) -> Vec<Position> {
        let pipe = &self.0[position.1][position.0];

        match pipe {
            Pipe::NorthSouth => vec![OFFSETS[0], OFFSETS[1]],
            Pipe::EastWest => vec![OFFSETS[2], OFFSETS[3]],
            Pipe::NorthEast => vec![OFFSETS[0], OFFSETS[3]],
            Pipe::NorthWest => vec![OFFSETS[0], OFFSETS[2]],
            Pipe::SouthWest => vec![OFFSETS[1], OFFSETS[2]],
            Pipe::SouthEast => vec![OFFSETS[1], OFFSETS[3]],
            Pipe::Ground => vec![],
            Pipe::Start => return self.get_start_neighbors(position),
        }
        .into_iter()
        .filter_map(|(x, y)| {
            let x = position.0 as i32 + x;
            let y = position.1 as i32 + y;
            if x < 0 || y < 0 {
                return None;
            }

            let x = x as usize;
            let y = y as usize;
            self.0.get(y).and_then(|row| row.get(x)).map(|_| (x, y))
        })
        .collect()
    }
}

impl From<&str> for Field {
    fn from(value: &str) -> Self {
        let pipes = value
            .lines()
            .map(|line| line.chars().map(Pipe::from).collect())
            .collect();
        Self(pipes)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let field = Field::from(input);

    field.get_longest_path()
}

fn solve_second_part(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
";

    #[test]
    fn test_first_part() {
        let input_1 = ".....
.S-7.
.|.|.
.L-J.
.....
";
        let input_2 = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        let input_3 = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let input_4 = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

        assert_eq!(4, solve_first_part(input_1));
        assert_eq!(4, solve_first_part(input_2));
        assert_eq!(8, solve_first_part(input_3));
        assert_eq!(8, solve_first_part(input_4));
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(7063, 42);
}

// 7065 - high
// 7053 - low

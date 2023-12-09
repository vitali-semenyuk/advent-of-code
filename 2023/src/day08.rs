use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Map {
    path: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn navigate(&self) -> u32 {
        let mut count = 0;
        let mut current = &"AAA".to_string();
        let mut index = 0;

        loop {
            let direction = self.path.get(index).unwrap();
            let (left, right) = self.nodes.get(current).unwrap();

            current = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };

            count += 1;

            if current == "ZZZ" {
                break;
            }

            index += 1;
            if index >= self.path.len() {
                index = 0;
            }
        }

        count
    }

    fn navigate_multiple(&self) -> u64 {
        let mut count = 0;
        let mut currents = self
            .nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .collect::<Vec<_>>();
        let mut index = 0;
        let mut periods = currents.iter().map(|_| 0).collect::<Vec<_>>();

        loop {
            let direction = self.path.get(index).unwrap();
            currents = currents
                .into_iter()
                .enumerate()
                .map(|(i, current)| {
                    let (left, right) = self.nodes.get(current).unwrap();
                    let new = match direction {
                        Direction::Left => left,
                        Direction::Right => right,
                    };

                    if new.ends_with('Z') {
                        periods[i] = count + 1;
                    }

                    new
                })
                .collect();

            count += 1;

            if periods.iter().all(|current| *current > 0) {
                break;
            }

            index += 1;
            if index >= self.path.len() {
                index = 0;
            }
        }

        periods.into_iter().reduce(lcm).unwrap()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let path = lines.next().unwrap().chars().map(Direction::from).collect();
        let nodes = lines
            .skip(1)
            .map(|str| {
                let (from, to) = str.split_once(" = ").unwrap();
                let (left, right) = to.split_once(", ").unwrap();
                let left = left.strip_prefix('(').unwrap();
                let right = right.strip_suffix(')').unwrap();

                (from.to_string(), (left.to_string(), right.to_string()))
            })
            .collect();

        Self { path, nodes }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let map = Map::from(input);

    map.navigate()
}

fn solve_second_part(input: &str) -> u64 {
    let map = Map::from(input);

    map.navigate_multiple()
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input_1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let input_2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(2, solve_first_part(input_1));
        assert_eq!(6, solve_first_part(input_2))
    }

    #[test]
    fn test_second_part() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        assert_eq!(6, solve_second_part(input))
    }

    check_answers!(16531, 24035773251517);
}

use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug)]
struct Map {
    map: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn get_trailheads(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            for (x, &position) in row.iter().enumerate() {
                if position == 0 {
                    result.push((x, y));
                }
            }
        }

        result
    }

    fn count_trails(&self, trailhead: (usize, usize)) -> (usize, usize) {
        let mut trailends = HashSet::new();
        let mut count = 0;
        let mut queue = VecDeque::new();

        queue.push_back(trailhead);

        while let Some(position) = queue.pop_front() {
            let current = self.map[position.1][position.0];
            if current == 9 {
                trailends.insert(position);
                count += 1;
                continue;
            }

            if position.0 > 0 && self.map[position.1][position.0 - 1] - current == 1 {
                queue.push_back((position.0 - 1, position.1));
            }

            if position.0 < self.width - 1 && self.map[position.1][position.0 + 1] - current == 1 {
                queue.push_back((position.0 + 1, position.1));
            }

            if position.1 > 0 && self.map[position.1 - 1][position.0] - current == 1 {
                queue.push_back((position.0, position.1 - 1));
            }

            if position.1 < self.height - 1 && self.map[position.1 + 1][position.0] - current == 1 {
                queue.push_back((position.0, position.1 + 1));
            }
        }

        (trailends.len(), count)
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect();
        let height = value.lines().count();
        let width = value.lines().next().unwrap().len();

        Self { map, width, height }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let map = Map::from(input);

    map.get_trailheads()
        .into_iter()
        .map(|trailhead| map.count_trails(trailhead).0)
        .sum()
}

fn solve_second_part(input: &str) -> usize {
    let map = Map::from(input);

    map.get_trailheads()
        .into_iter()
        .map(|trailhead| map.count_trails(trailhead).1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_first_part() {
        let answer = 36;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 81;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(796, 1942);
}

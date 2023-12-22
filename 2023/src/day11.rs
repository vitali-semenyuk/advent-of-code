use std::fmt::Display;

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    Empty,
    Galaxy,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Invalid value"),
        }
    }
}

#[derive(Debug)]
struct Universe(Vec<Vec<Space>>);

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let space = value
            .lines()
            .map(|line| line.chars().map(Space::from).collect())
            .collect();

        Self(space)
    }
}

impl Universe {
    fn get_distances_sum(&self, expansion_coefficient: usize) -> usize {
        let rows = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|s| *s == Space::Empty))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let width = self.0[0].len();
        let columns = (0..width)
            .filter(|&index| {
                self.0
                    .iter()
                    .map(|row| row[index])
                    .all(|s| s == Space::Empty)
            })
            .collect::<Vec<_>>();

        let galaxies = self
            .0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, s)| **s == Space::Galaxy)
                    .map(|(x, _)| {
                        let offset_x = columns.iter().filter(|n| x > **n).count()
                            * (expansion_coefficient - 1);
                        let offset_y =
                            rows.iter().filter(|n| y > **n).count() * (expansion_coefficient - 1);
                        (x + offset_x, y + offset_y)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut pairs = Vec::new();
        for i in 0..galaxies.len() - 1 {
            for j in i + 1..galaxies.len() {
                pairs.push((galaxies[i], galaxies[j]))
            }
        }

        pairs.into_iter().map(|(a, b)| distance(a, b)).sum()
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let universe = Universe::from(input);

    universe.get_distances_sum(2)
}

fn solve_second_part(input: &str) -> usize {
    let universe = Universe::from(input);

    universe.get_distances_sum(1_000_000)
}

fn distance(a: Position, b: Position) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_first_part() {
        let answer = 374;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 82000210;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(10885634, 707505470642);
}

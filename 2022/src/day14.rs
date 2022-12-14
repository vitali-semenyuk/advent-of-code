use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Point { x, y }
    }
}

#[derive(Clone, PartialEq)]
enum Space {
    Rock,
    Sand,
    Void,
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "o"),
            Self::Void => write!(f, "."),
        }
    }
}

struct Grid(Vec<Vec<Space>>);

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = writeln!(f);

        for line in &self.0 {
            for cell in line {
                result = result.and_then(|_| write!(f, "{:?}", cell))
            }
            result = result.and_then(|_| writeln!(f))
        }

        result
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let pathes: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split(" -> ").map(|p| Point::from(p)).collect())
        .collect();
    let mut grid = Grid(vec![vec![Space::Void; 550]; 500]);

    for path in pathes {
        for line in path.iter().zip(path.iter().skip(1)) {
            draw_path(&mut grid, line)
        }
    }

    let mut n = 0;
    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y > 400 {
                break 'outer;
            }

            if grid.0[y + 1][x] == Space::Void {
                y = y + 1;
            } else if grid.0[y + 1][x - 1] == Space::Void {
                y = y + 1;
                x = x - 1;
            } else if grid.0[y + 1][x + 1] == Space::Void {
                y = y + 1;
                x = x + 1;
            } else {
                break;
            }
        }
        grid.0[y][x] = Space::Sand;
        n = n + 1;
    }

    n
}

fn solve_second_part(input: &str) -> u32 {
    let pathes: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.split(" -> ").map(|p| Point::from(p)).collect())
        .collect();
    let max_y = pathes
        .iter()
        .flat_map(|path| path.iter().map(|p| p.y))
        .max()
        .unwrap();
    dbg!(max_y);
    let mut grid = Grid(vec![vec![Space::Void; 1000]; max_y + 2]);

    for path in pathes {
        for line in path.iter().zip(path.iter().skip(1)) {
            draw_path(&mut grid, line)
        }
    }

    let mut n = 0;
    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y == max_y + 1 {
                break;
            }

            if grid.0[y + 1][x] == Space::Void {
                y = y + 1;
            } else if grid.0[y + 1][x - 1] == Space::Void {
                y = y + 1;
                x = x - 1;
            } else if grid.0[y + 1][x + 1] == Space::Void {
                y = y + 1;
                x = x + 1;
            } else {
                if y == 0 {
                    break 'outer;
                }
                break;
            }
        }
        grid.0[y][x] = Space::Sand;
        n = n + 1;
    }

    dbg!(grid);

    n + 1
}

fn draw_path(grid: &mut Grid, path: (&Point, &Point)) {
    if path.0.x == path.1.x {
        let x = path.0.x;
        for y in range(path.0.y, path.1.y) {
            grid.0[y][x] = Space::Rock
        }
    } else {
        let y = path.0.y;
        for x in range(path.0.x, path.1.x) {
            grid.0[y][x] = Space::Rock
        }
    }
}

fn range(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_first_part() {
        let answer = 24;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 93;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(610, 27194);
}

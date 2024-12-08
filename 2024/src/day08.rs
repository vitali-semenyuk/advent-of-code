use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Map {
    antenas: HashMap<char, Vec<Point>>,
    width: usize,
    height: usize,
}

impl Map {
    fn count_antinodes(&self, resonant_harmonics: bool) -> usize {
        let mut antinodes = HashSet::new();

        for antenas in self.antenas.values() {
            for &start in antenas {
                for &end in antenas {
                    if !is_direct_point(start, end, antenas) {
                        continue;
                    }

                    if resonant_harmonics {
                        antinodes.insert(start);
                    }

                    let diff_x = start.x - end.x;
                    let diff_y = start.y - end.y;

                    let mut x = start.x;
                    let mut y = start.y;

                    loop {
                        x += diff_x;
                        y += diff_y;

                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            antinodes.insert(Point { x, y });
                            if !resonant_harmonics {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    let mut x = end.x;
                    let mut y = end.y;

                    loop {
                        x -= diff_x;
                        y -= diff_y;

                        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                            antinodes.insert(Point { x, y });
                            if !resonant_harmonics {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        antinodes.len()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut antenas = HashMap::new();
        let mut width = 0;
        let height = value.lines().count();

        for (y, row) in value.lines().enumerate() {
            width = row.len();

            for (x, antena) in row.chars().enumerate() {
                if antena == '.' {
                    continue;
                }

                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                antenas.entry(antena).or_insert(Vec::new()).push(point);
            }
        }

        Self {
            antenas,
            width,
            height,
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
    Map::from(input).count_antinodes(false)
}

fn solve_second_part(input: &str) -> usize {
    Map::from(input).count_antinodes(true)
}

fn is_direct_point(point: Point, target: Point, points: &[Point]) -> bool {
    if point == target {
        return false;
    }

    !points.iter().any(|&p| {
        if p == point || p == target {
            return false;
        }

        is_on_line(point, target, p)
    })
}

fn is_on_line(line_start: Point, line_end: Point, point: Point) -> bool {
    let ab = distance(line_start, line_end);
    let ap = distance(line_start, point);
    let pb = distance(point, line_end);

    (ap + pb - ab).abs() < 0.0001
}

fn distance(p1: Point, p2: Point) -> f64 {
    (((p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y)) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_first_part() {
        let answer = 14;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 34;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(369, 1169);
}

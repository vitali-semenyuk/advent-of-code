use std::{f64::consts::PI, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let points = get_points(input);

    get_best_point(&points).1
}

fn solve_second_part(input: &str) -> i32 {
    let points = get_points(input);
    let center = get_best_point(&points).0;

    let point = get_nth_point(center, &points, 200);

    point.x * 100 + point.y
}

fn get_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_nth_point(center: Point, points: &[Point], n: usize) -> Point {
    let mut ordered_points = points
        .iter()
        .map(|&point| {
            if point == center {
                return (point, None);
            }

            if !is_direct_point(center, point, points) {
                return (point, None);
            }

            let dx = point.x - center.x;
            let dy = point.y - center.y;
            let theta = f64::atan2(dy.into(), dx.into());

            (point, Some(theta))
        })
        .filter_map(|item| item.1.map(|angle| (item.0, angle)))
        .collect::<Vec<_>>();
    ordered_points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let start = ordered_points
        .iter()
        .position(|(_, angle)| *angle == -PI / 2.0)
        .unwrap();
    ordered_points.rotate_left(start);

    ordered_points[n - 1].0
}

fn get_best_point(points: &[Point]) -> (Point, usize) {
    points
        .iter()
        .map(|point| {
            let count = points
                .iter()
                .filter(|target| is_direct_point(*point, **target, points))
                .count();
            (*point, count)
        })
        .max_by_key(|x| x.1)
        .unwrap()
}

fn is_direct_point(point: Point, target: Point, points: &[Point]) -> bool {
    if point == target {
        return false;
    }

    let has_blockers = points.iter().any(|&p| {
        if p == point || p == target {
            return false;
        }

        is_on_line(point, target, p)
    });

    !has_blockers
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

    const INPUT: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";

    #[test]
    fn test_first_part() {
        let input = ".#..#
.....
#####
....#
...##
";
        assert_eq!(8, solve_first_part(input));

        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
        assert_eq!(33, solve_first_part(input));

        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
        assert_eq!(35, solve_first_part(input));

        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
        assert_eq!(41, solve_first_part(input));

        assert_eq!(210, solve_first_part(INPUT));
    }

    #[test]
    fn test_second_part() {
        let answer = 802;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(227, 604);
}

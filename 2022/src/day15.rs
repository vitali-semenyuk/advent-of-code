use std::fmt::Display;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    beacon: Point,
    radius: i32,
}

impl Sensor {
    fn distance_to(&self, point: &Point) -> i32 {
        distance(&self.position, point)
    }
}

impl From<&str> for Sensor {
    fn from(string: &str) -> Self {
        let mut parts = string.split_whitespace();

        let x = parts.nth(2).unwrap();
        let y = parts.next().unwrap();

        let x = x.trim_end_matches(',');
        let (_, x) = x.split_once('=').unwrap();
        let x = x.parse().unwrap();
        let y = y.trim_end_matches(':');
        let (_, y) = y.split_once('=').unwrap();
        let y = y.parse().unwrap();

        let position = Point { x, y };

        let x = parts.nth(4).unwrap();
        let y = parts.next().unwrap();

        let x = x.trim_end_matches(',');
        let (_, x) = x.split_once('=').unwrap();
        let x = x.parse().unwrap();
        let y = y.trim_end_matches(':');
        let (_, y) = y.split_once('=').unwrap();
        let y = y.parse().unwrap();

        let beacon = Point { x, y };

        let radius = distance(&position, &beacon);

        Sensor {
            position,
            beacon,
            radius,
        }
    }
}

fn distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    solve_first_part_inner(input, 2_000_000)
}

fn solve_second_part(input: &str) -> i64 {
    solve_second_part_inner(input, 4_000_000)
}

fn solve_first_part_inner(input: &str, y: i32) -> i32 {
    let sensors: Vec<_> = input.lines().map(Sensor::from).collect();

    let min_x = sensors
        .iter()
        .min_by(|a, b| (a.position.x - a.radius).cmp(&(b.position.x - b.radius)))
        .unwrap();
    let min_x = min_x.position.x - min_x.radius;
    let max_x = sensors
        .iter()
        .max_by(|a, b| (a.position.x + a.radius).cmp(&(b.position.x + b.radius)))
        .unwrap();
    let max_x = max_x.position.x + max_x.radius;

    let mut count = 0;
    for x in min_x..=max_x {
        let point = Point { x, y };
        for sensor in &sensors {
            if sensor.distance_to(&point) <= sensor.radius && sensor.beacon != point {
                count += 1;
                break;
            }
        }
    }

    count
}

fn solve_second_part_inner(input: &str, r: i32) -> i64 {
    let sensors: Vec<_> = input.lines().map(Sensor::from).collect();

    let mut points = Vec::new();
    for sensor in &sensors {
        let x0 = sensor.position.x - sensor.radius - 1;
        let x1 = sensor.position.x + sensor.radius + 1;
        let y0 = sensor.position.y - sensor.radius - 1;
        let y1 = sensor.position.y + sensor.radius + 1;

        for (i, x) in (x0..sensor.position.x).enumerate() {
            points.push(Point {
                x,
                y: sensor.position.y - i as i32,
            });
            points.push(Point {
                x,
                y: sensor.position.y + i as i32,
            });
        }

        for (i, x) in (sensor.position.x..=x1).enumerate() {
            points.push(Point {
                x,
                y: y0 + i as i32,
            });
            points.push(Point {
                x,
                y: y1 - i as i32,
            });
        }
    }

    for point in points
        .iter()
        .filter(|p| p.x >= 0 && p.x <= r && p.y >= 0 && p.y <= r)
    {
        if sensors.iter().all(|s| s.distance_to(point) > s.radius) {
            return point.x as i64 * 4000000 + point.y as i64;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_first_part() {
        let answer = 26;

        assert_eq!(answer, solve_first_part_inner(INPUT, 10))
    }

    #[test]
    fn test_second_part() {
        let answer = 56000011;

        assert_eq!(answer, solve_second_part_inner(INPUT, 20))
    }

    check_answers!(5809294, 10693731308112);
}

use std::fmt::Display;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    length: u32,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let direction = match chars.next().expect("empty string") {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        };

        let length = chars.collect::<String>().parse().expect("invalid number");

        Move { direction, length }
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn det(&self, other: Self) -> i32 {
        (self.x as i64 * other.y as i64 - self.y as i64 * other.x as i64) as i32
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn manhattan(&self, other: &Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

#[derive(Debug)]
struct Segment {
    from: Vector,
    to: Vector,
    length: u32,
}

impl Segment {
    fn get_intersection(&self, other: &Self) -> Option<Vector> {
        let diff_x = Vector::new(self.from.x - self.to.x, other.from.x - other.to.x);
        let diff_y = Vector::new(self.from.y - self.to.y, other.from.y - other.to.y);

        let div = diff_x.det(diff_y);
        if div == 0 {
            return None;
        }

        let d = Vector::new(self.from.det(self.to), other.from.det(other.to));
        let x = d.det(diff_x) / div;
        let y = d.det(diff_y) / div;

        let point = Vector::new(x, y);
        if self.contains(point) && other.contains(point) {
            Some(point)
        } else {
            None
        }
    }

    fn contains(&self, point: Vector) -> bool {
        let x_start = self.from.x.min(self.to.x);
        let x_end = self.from.x.max(self.to.x);
        let y_start = self.from.y.min(self.to.y);
        let y_end = self.from.y.max(self.to.y);

        point.x >= x_start && point.x <= x_end && point.y >= y_start && point.y <= y_end
    }
}

#[derive(Debug)]
struct Path {
    moves: Vec<Move>,
}

impl Path {
    fn get_segments(&self) -> Vec<Segment> {
        let mut segments = Vec::new();
        let mut from = Vector::new(0, 0);

        for m in &self.moves {
            let mut to = from.clone();

            match m.direction {
                Direction::Up => to.y += m.length as i32,
                Direction::Down => to.y -= m.length as i32,
                Direction::Left => to.x -= m.length as i32,
                Direction::Right => to.x += m.length as i32,
            }

            segments.push(Segment {
                from: from.clone(),
                to: to.clone(),
                length: m.length,
            });

            from = to;
        }

        segments
    }

    fn get_intersections(&self, other: &Self) -> Vec<(Vector, u32)> {
        let mut points = Vec::new();

        let mut self_length = 0;

        for segment in self.get_segments() {
            self_length += segment.length;
            let mut other_length = 0;

            for s in other.get_segments() {
                other_length += s.length;

                if let Some(intersection) = segment.get_intersection(&s) {
                    if intersection.distance() > 0 {
                        let length = self_length + other_length
                            - segment.to.manhattan(&intersection)
                            - s.to.manhattan(&intersection);
                        points.push((intersection, length));
                    }
                }
            }
        }

        points
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let moves = value.split(',').map(Move::from).collect();
        Path { moves }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let mut lines = input.lines();
    let first = lines.next().expect("expected first wire string");
    let second = lines.next().expect("expected first wire string");

    let first = Path::from(first);
    let second = Path::from(second);

    let intersections = first.get_intersections(&second);
    let (intersection, _) = intersections
        .iter()
        .min_by_key(|(i, _)| i.distance())
        .expect("no intersections");

    intersection.distance()
}

fn solve_second_part(input: &str) -> u32 {
    let mut lines = input.lines();
    let first = lines.next().expect("expected first wire string");
    let second = lines.next().expect("expected first wire string");

    let first = Path::from(first);
    let second = Path::from(second);

    let intersections = first.get_intersections(&second);
    let (_, steps) = intersections
        .iter()
        .min_by_key(|(_, s)| s)
        .expect("no intersections");

    *steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            6,
            solve_first_part(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            )
        );
        assert_eq!(
            159,
            solve_first_part(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            135,
            solve_first_part(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            30,
            solve_second_part(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            )
        );
        assert_eq!(
            610,
            solve_second_part(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            410,
            solve_second_part(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    check_answers!(3247, 48054);
}

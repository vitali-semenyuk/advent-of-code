use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();

        let direction = match chars.next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected direction"),
        };
        let distance = chars.collect::<String>().parse().unwrap();

        Self {
            direction,
            distance,
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }?;
        write!(f, "{}", self.distance)
    }
}

impl Rotation {
    fn diff(&self) -> i32 {
        match self.direction {
            Direction::Left => -self.distance,
            Direction::Right => self.distance,
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
    input
        .lines()
        .map(Rotation::from)
        .fold(vec![50], |mut acc, cur| {
            let new_value = (acc.last().unwrap() + cur.diff()).rem_euclid(100);
            acc.push(new_value);
            acc
        })
        .into_iter()
        .filter(|v| *v == 0)
        .count()
}

fn solve_second_part(input: &str) -> i32 {
    let mut counter = 0;
    let mut current = 50;

    for rotation in input.lines().map(Rotation::from) {
        let new_value = (current + rotation.diff()).rem_euclid(100);
        let mut turns = (current + rotation.diff()).div_euclid(100).abs();

        if current == 0 && rotation.direction == Direction::Left {
            turns -= 1;
        }

        if new_value == 0 && rotation.direction == Direction::Left {
            turns += 1;
        }

        current = new_value;
        counter += turns;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_first_part() {
        let answer = 3;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 6;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(1172, 6932);
}

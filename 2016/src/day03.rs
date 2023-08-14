use std::fmt::Display;

#[derive(Debug)]
struct Triangle(u32, u32, u32);

impl From<&str> for Triangle {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

impl From<Vec<&str>> for Triangle {
    fn from(value: Vec<&str>) -> Self {
        let mut parts = value.iter();

        Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2 && self.1 + self.2 > self.0 && self.2 + self.0 > self.1
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
        .map(Triangle::from)
        .filter(Triangle::is_valid)
        .count()
}

fn solve_second_part(input: &str) -> usize {
    let triangles = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    triangles
        .chunks(3)
        .flat_map(transpose)
        .map(Triangle::from)
        .filter(Triangle::is_valid)
        .count()
}

fn transpose<'a>(matrix: &'a [Vec<&str>]) -> Vec<Vec<&'a str>> {
    (0..matrix.len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5 10 25
3 4 3
3 7 5
1 34 5
6 8 10
37 92 56
";

    #[test]
    fn test_first_part() {
        let answer = 4;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(982, 1826);
}

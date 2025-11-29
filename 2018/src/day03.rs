use std::fmt::Display;

#[derive(Debug)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl From<&str> for Claim {
    fn from(value: &str) -> Self {
        let (id, rest) = value.split_once(" @ ").unwrap();
        let (pos, rect) = rest.split_once(": ").unwrap();
        let (x, y) = pos.split_once(",").unwrap();
        let (width, height) = rect.split_once("x").unwrap();

        Self {
            id: id.strip_prefix("#").unwrap().parse().unwrap(),
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
        }
    }
}

impl Claim {
    fn intersects(&self, other: &Claim) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let mut matrix = vec![vec![0; 1000]; 1000];

    for claim in input.lines().map(Claim::from) {
        (claim.x..claim.x + claim.width).for_each(|i| {
            (claim.y..claim.y + claim.height).for_each(|j| {
                matrix[j][i] += 1;
            });
        });
    }

    matrix.iter().flatten().filter(|c| **c > 1).count()
}

fn solve_second_part(input: &str) -> u32 {
    let claims = input.lines().map(Claim::from);

    for claim in claims.clone() {
        if !claims
            .clone()
            .filter(|other| other.id != claim.id)
            .any(|other| claim.intersects(&other))
        {
            return claim.id;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

    #[test]
    fn test_first_part() {
        let answer = 4;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 3;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(118322, 1178);
}

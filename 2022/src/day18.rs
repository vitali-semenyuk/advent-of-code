use std::fmt::Display;

#[derive(Debug)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

impl From<&str> for Cube {
    fn from(string: &str) -> Self {
        let mut parts = string.split(',');

        Cube {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Cube {
    fn distance(a: &Self, b: &Self) -> u32 {
        a.x.abs_diff(b.x) + a.y.abs_diff(b.y) + a.z.abs_diff(b.z)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let cubes: Vec<_> = input.lines().map(Cube::from).collect();

    let res: usize = cubes
        .iter()
        .map(|cube| {
            cubes
                .iter()
                .filter(|c| Cube::distance(cube, c) == 1)
                .count()
        })
        .sum();

    cubes.len() * 6 - res
}

fn solve_second_part(_input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_first_part() {
        let answer = 64;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(4192, 42);
}

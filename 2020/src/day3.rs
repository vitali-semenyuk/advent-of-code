use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u64 {
    count_trees(input, (3, 1))
}

fn solve_second_part(input: &str) -> u64 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .map(|s| count_trees(input, s))
        .iter()
        .product()
}

fn count_trees(input: &str, steps: (usize, usize)) -> u64 {
    let grid: Vec<_> = input.lines().collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0;
    let mut x = 0;
    let mut y = 0;

    while y < height {
        if grid[y].chars().nth(x).unwrap() == '#' {
            count += 1;
        }

        x += steps.0;
        x %= width;
        y += steps.1;
    }

    count
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_first_part() {
        let answer = 7;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 336;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(294, 5774564250);
}

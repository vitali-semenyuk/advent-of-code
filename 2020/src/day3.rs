pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    count_trees(input, (3, 1)) as i64
}

fn solve_second_part(input: &str) -> i64 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .map(|s| count_trees(input, s) as i64)
        .iter()
        .product()
}

fn count_trees(input: &str, steps: (usize, usize)) -> i32 {
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
}

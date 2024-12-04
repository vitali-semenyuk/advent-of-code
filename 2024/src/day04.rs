use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map({
                let value = matrix.clone();
                move |(x, c)| {
                    if *c == 'X' {
                        count_xmas(&value, x, y)
                    } else {
                        0
                    }
                }
            })
        })
        .sum()
}

fn solve_second_part(input: &str) -> usize {
    let matrix = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    matrix
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map({
                let value = matrix.clone();
                move |(x, c)| {
                    if *c == 'A' {
                        count_mas(&value, x, y)
                    } else {
                        0
                    }
                }
            })
        })
        .sum()
}

fn count_xmas(matrix: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut counter = 0;

    let height = matrix.len();
    let width = matrix[0].len();

    if x >= 3 && matrix[y][x - 1] == 'M' && matrix[y][x - 2] == 'A' && matrix[y][x - 3] == 'S' {
        counter += 1;
    }

    if x <= width - 4
        && matrix[y][x + 1] == 'M'
        && matrix[y][x + 2] == 'A'
        && matrix[y][x + 3] == 'S'
    {
        counter += 1;
    }

    if y >= 3 && matrix[y - 1][x] == 'M' && matrix[y - 2][x] == 'A' && matrix[y - 3][x] == 'S' {
        counter += 1;
    }

    if y <= height - 4
        && matrix[y + 1][x] == 'M'
        && matrix[y + 2][x] == 'A'
        && matrix[y + 3][x] == 'S'
    {
        counter += 1;
    }

    if x >= 3
        && y >= 3
        && matrix[y - 1][x - 1] == 'M'
        && matrix[y - 2][x - 2] == 'A'
        && matrix[y - 3][x - 3] == 'S'
    {
        counter += 1;
    }

    if x >= 3
        && y <= height - 4
        && matrix[y + 1][x - 1] == 'M'
        && matrix[y + 2][x - 2] == 'A'
        && matrix[y + 3][x - 3] == 'S'
    {
        counter += 1;
    }

    if x <= width - 4
        && y >= 3
        && matrix[y - 1][x + 1] == 'M'
        && matrix[y - 2][x + 2] == 'A'
        && matrix[y - 3][x + 3] == 'S'
    {
        counter += 1;
    }

    if x <= width - 4
        && y <= height - 4
        && matrix[y + 1][x + 1] == 'M'
        && matrix[y + 2][x + 2] == 'A'
        && matrix[y + 3][x + 3] == 'S'
    {
        counter += 1;
    }

    counter
}

fn count_mas(matrix: &[Vec<char>], x: usize, y: usize) -> usize {
    let mut counter = 0;

    let height = matrix.len();
    let width = matrix[0].len();

    if x >= 1 && y >= 1 && x <= width - 2 && y <= height - 2 {
        let tl = matrix[y - 1][x - 1];
        let tr = matrix[y - 1][x + 1];
        let bl = matrix[y + 1][x - 1];
        let br = matrix[y + 1][x + 1];
        if ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
            && ((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M'))
        {
            counter += 1;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_first_part() {
        let answer = 18;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 9;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(2458, 1945);
}

use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let size = width * height;
    let forest: Vec<u32> = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    let mut count = 0;
    for (i, tree) in forest.iter().enumerate() {
        let mut visible = true;
        for t in &forest[(i / width * width)..i] {
            if t >= tree {
                visible = false;
                break;
            }
        }
        if visible {
            count += 1;
            continue;
        }

        visible = true;
        for t in &forest[(i + 1)..((i / width + 1) * width)] {
            if t >= tree {
                visible = false;
                break;
            }
        }
        if visible {
            count += 1;
            continue;
        }

        visible = true;
        for j in ((i % width)..i).step_by(width) {
            let t = forest[j];
            if t >= *tree {
                visible = false;
                break;
            }
        }
        if visible {
            count += 1;
            continue;
        }

        visible = true;
        for j in (i..size).step_by(width).skip(1) {
            let t = forest[j];
            if t >= *tree {
                visible = false;
                break;
            }
        }
        if visible {
            count += 1;
            continue;
        }
    }

    count
}

fn solve_second_part(input: &str) -> i32 {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let size = width * height;
    let forest: Vec<u32> = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    let mut max_score = 0;
    for (i, tree) in forest.iter().enumerate() {
        let mut d_left = 0;
        for t in forest[(i / width * width)..i].iter().rev() {
            d_left += 1;
            if t >= tree {
                break;
            }
        }

        let mut d_right = 0;
        for t in &forest[(i + 1)..((i / width + 1) * width)] {
            d_right += 1;
            if t >= tree {
                break;
            }
        }

        let mut d_up = 0;
        for j in ((i % width)..i).step_by(width).rev() {
            d_up += 1;
            let t = forest[j];
            if t >= *tree {
                break;
            }
        }

        let mut d_down = 0;
        for j in (i..size).step_by(width).skip(1) {
            d_down += 1;
            let t = forest[j];
            if t >= *tree {
                break;
            }
        }

        let score = d_left * d_right * d_up * d_down;
        if score > max_score {
            max_score = score
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_first_part() {
        let answer = 21;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 8;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(1782, 474606);
}

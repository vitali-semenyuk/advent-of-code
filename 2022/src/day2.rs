#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Win,
    Lose,
    Draw,
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (opponent, me) = l.split_once(" ").unwrap();
            score_round((parse(me), parse(opponent)))
        })
        .sum::<u32>() as i64
}

fn solve_second_part(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            let (opponent, result) = l.split_once(" ").unwrap();
            let opponent = parse(opponent);
            let result = parse_result(result);
            let me = find_shape(opponent, result);
            score_round((me, opponent))
        })
        .sum::<u32>() as i64
}

fn find_shape(opponent: Shape, result: Result) -> Shape {
    match result {
        Result::Win => match opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        Result::Lose => match opponent {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
        Result::Draw => opponent,
    }
}

fn score_round(round: (Shape, Shape)) -> u32 {
    let shape = match round.0 {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let score = match round.0 {
        Shape::Rock => match round.1 {
            Shape::Rock => 3,
            Shape::Paper => 0,
            Shape::Scissors => 6,
        },
        Shape::Paper => match round.1 {
            Shape::Rock => 6,
            Shape::Paper => 3,
            Shape::Scissors => 0,
        },
        Shape::Scissors => match round.1 {
            Shape::Rock => 0,
            Shape::Paper => 6,
            Shape::Scissors => 3,
        },
    };

    score + shape
}

fn parse(string: &str) -> Shape {
    match string {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!(),
    }
}

fn parse_result(string: &str) -> Result {
    match string {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_first_part() {
        let answer = 15;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 12;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(12740, 11980);
}

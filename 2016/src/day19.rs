use std::{collections::VecDeque, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let number = input.strip_suffix('\n').unwrap().parse().unwrap();
    let mut players = (1..=number).collect::<VecDeque<_>>();

    loop {
        let len = players.len();
        let mut filtered = players
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, n)| n)
            .collect::<VecDeque<_>>();
        if len % 2 != 0 {
            filtered.pop_front();
        }
        players = filtered;

        if players.len() == 1 {
            break;
        }
    }

    players.pop_back().unwrap()
}

fn solve_second_part(input: &str) -> i32 {
    let number = input.strip_suffix('\n').unwrap().parse().unwrap();
    let mut range = (1..=number).collect::<Vec<_>>();
    let mut current_index = 0;

    loop {
        let size = range.len();
        if size == 1 {
            break;
        }

        let opposite_index = (current_index + size / 2) % size;
        range.remove(opposite_index);

        if opposite_index > current_index {
            current_index = (current_index + 1) % (size - 1);
        } else if current_index == size - 1 {
            current_index = 0
        }
    }

    range.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5
";

    #[test]
    fn test_first_part() {
        let answer = 3;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT));
        assert_eq!(5, solve_second_part("14\n"));
    }

    check_answers!(1841611, 1423634);
}

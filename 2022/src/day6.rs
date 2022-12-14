use std::{collections::HashSet, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let binding: Vec<_> = input.chars().collect();
    for (i, chars) in binding.windows(4).enumerate() {
        let hs: HashSet<_> = HashSet::from_iter(chars);
        if hs.len() == chars.len() {
            return i + 4;
        }
    }

    0
}

fn solve_second_part(input: &str) -> usize {
    let binding: Vec<_> = input.chars().collect();
    for (i, chars) in binding.windows(14).enumerate() {
        let hs: HashSet<_> = HashSet::from_iter(chars);
        if hs.len() == chars.len() {
            return i + 14;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(7, solve_first_part("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, solve_first_part("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, solve_first_part("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, solve_first_part("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, solve_first_part("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(19, solve_second_part("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, solve_second_part("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, solve_second_part("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, solve_second_part("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, solve_second_part("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    check_answers!(1080, 3645);
}

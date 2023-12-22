use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();
            expand(springs)
                .into_iter()
                .map(get_damaged_groups)
                .filter(|variant| *variant == groups)
                .count()
        })
        .sum()
}

fn solve_second_part(_input: &str) -> i32 {
    0
}

fn expand(string: &str) -> Vec<String> {
    if let Some(index) = string.chars().position(|ch| ch == '?') {
        let mut first = string.to_string();
        first.replace_range(index..index + 1, ".");
        let mut second = string.to_string();
        second.replace_range(index..index + 1, "#");

        let mut result = expand(&first);
        result.extend(expand(&second));
        result
    } else {
        vec![string.to_string()]
    }
}

fn get_damaged_groups(string: String) -> Vec<usize> {
    string
        .split('.')
        .map(|g| g.len())
        .filter(|&n| n > 0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_first_part() {
        let answer = 21;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 525152;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(7857, 42);
}

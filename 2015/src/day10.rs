use std::fmt::Display;
use std::fmt::Write;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let input = input.strip_suffix('\n').unwrap();

    look_and_say_n_times(input, 40).len()
}

fn solve_second_part(input: &str) -> usize {
    let input = input.strip_suffix('\n').unwrap();

    look_and_say_n_times(input, 50).len()
}

fn look_and_say_n_times(string: &str, iterations: u32) -> String {
    let mut string = string.to_string();

    for _ in 0..iterations {
        string = look_and_say(&string)
    }

    string
}

fn look_and_say(string: &str) -> String {
    let chars = string.chars();

    let mut groups = Vec::new();
    let mut current_group = Vec::new();

    for char in chars {
        if !current_group.is_empty() && char != *current_group.first().unwrap() {
            groups.push(current_group.clone());
            current_group.clear();
        }

        current_group.push(char)
    }

    groups.push(current_group);

    groups.iter().fold(String::new(), |mut str, group| {
        let _ = write!(str, "{}{}", group.len(), group.first().unwrap());
        str
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
";

    #[test]
    fn test_first_part() {
        let answer = 82350;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 1166642;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_look_and_say_n_times() {
        assert_eq!(look_and_say_n_times("1", 5), "312211");
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }

    check_answers!(492982, 6989950);
}

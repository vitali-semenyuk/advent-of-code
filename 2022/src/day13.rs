#[derive(Debug)]
enum PacketValue {
    Integer(i32),
    List(Vec<PacketValue>),
}

fn parse(string: &str) -> PacketValue {

    PacketValue::Integer(1)
}

impl From<&str> for PacketValue {
    fn from(string: &str) -> Self {
        parse(string)
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    let pairs: Vec<_> = input
        .split("\n\n")
        .map(|s| {
            let (first, second) = s.split_once("\n").unwrap();
            PacketValue::from(first)
        })
        .collect();

    // dbg!(pairs);

    42
}

fn solve_second_part(input: &str) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(6428, 22464);
}

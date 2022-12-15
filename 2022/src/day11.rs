use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    0
}

fn solve_second_part(input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn test_first_part() {
        let answer = 10605;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(42, 42);
}

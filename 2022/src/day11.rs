use std::fmt::Display;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    branch_true: usize,
    branch_false: usize,
}

impl From<&str> for Monkey {
    fn from(string: &str) -> Self {
        let mut lines = string.lines().skip(1);

        let items = lines
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let mut operation_parts = lines.next().unwrap().split_whitespace().skip(4);
        let operator = operation_parts.next().unwrap();
        let operand = operation_parts.next().unwrap();
        let operation = if operand == "old" {
            Operation::Square
        } else if operator == "*" {
            Operation::Multiply(operand.parse().unwrap())
        } else if operator == "+" {
            Operation::Add(operand.parse().unwrap())
        } else {
            panic!()
        };
        let test = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let branch_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let branch_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            test,
            branch_true,
            branch_false,
        }
    }
}

impl Monkey {
    fn do_operation(&self, value: u64, reduce: bool) -> u64 {
        let level = match self.operation {
            Operation::Add(n) => value + n,
            Operation::Multiply(n) => value * n,
            Operation::Square => value * value,
        };

        let level = if reduce { level / 3 } else { level };

        level % (23 * 19 * 17 * 13 * 11 * 7 * 5 * 3 * 2)
    }

    fn check(&self, value: u64) -> (u64, usize) {
        if value % self.test == 0 {
            (value, self.branch_true)
        } else {
            (value, self.branch_false)
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();
    let mut counter = vec![0; monkeys.len()];

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let items: Vec<_> = monkey
                .items
                .iter()
                .map(|i| monkey.check(monkey.do_operation(*i, true)))
                .collect();
            counter[index] += items.len();
            monkey.items.clear();
            for (value, index) in items {
                monkeys[index].items.push(value)
            }
        }
    }

    counter.sort();
    counter.reverse();

    counter[0] * counter[1]
}

fn solve_second_part(input: &str) -> usize {
    let mut monkeys: Vec<_> = input.split("\n\n").map(Monkey::from).collect();
    let mut counter = vec![0; monkeys.len()];

    for _ in 0..10_000 {
        for index in 0..monkeys.len() {
            let monkey = &mut monkeys[index];
            let items: Vec<_> = monkey
                .items
                .iter()
                .map(|i| monkey.check(monkey.do_operation(*i, false)))
                .collect();
            counter[index] += items.len();
            monkey.items.clear();
            for (value, index) in items {
                monkeys[index].items.push(value)
            }
        }
    }

    counter.sort();
    counter.reverse();

    counter[0] * counter[1]
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
        let answer = 2713310158;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(58322, 13937702909);
}

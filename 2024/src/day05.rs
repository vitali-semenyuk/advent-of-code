use std::fmt::Display;

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

impl Rule {
    fn is_valid_update(&self, update: &[u32]) -> bool {
        let before = update.iter().position(|&i| i == self.before);
        let after = update.iter().position(|&i| i == self.after);

        before.map_or(true, |before| after.map_or(true, |after| before < after))
    }

    fn fix(&self, update: &mut [u32]) {
        let before = update.iter().position(|&i| i == self.before).unwrap();
        let after = update.iter().position(|&i| i == self.after).unwrap();

        (update[before], update[after]) = (update[after], update[before])
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (before, after) = value.split_once('|').unwrap();
        let before = before.parse::<u32>().unwrap();
        let after = after.parse::<u32>().unwrap();

        Self { before, after }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    let valid_updates = updates
        .into_iter()
        .filter(|update| rules.iter().all(|rule| rule.is_valid_update(update)))
        .collect::<Vec<_>>();

    middle_page_sum(&valid_updates)
}

fn solve_second_part(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    let mut invalid_updates = updates
        .into_iter()
        .filter(|update| !rules.iter().all(|rule| rule.is_valid_update(update)))
        .collect::<Vec<_>>();

    for update in &mut invalid_updates {
        loop {
            let mut valid = true;
            for rule in &rules {
                if !rule.is_valid_update(update) {
                    rule.fix(update);
                    valid = false
                }
            }

            if valid {
                break;
            }
        }
    }

    middle_page_sum(&invalid_updates)
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(Rule::from).collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    (rules, updates)
}

fn middle_page_sum(updates: &[Vec<u32>]) -> u32 {
    updates.iter().map(|update| update[update.len() / 2]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_first_part() {
        let answer = 143;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 123;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(7307, 4713);
}

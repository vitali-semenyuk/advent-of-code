use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Sue {
    id: u32,
    items: HashMap<String, u32>,
}

impl From<&str> for Sue {
    fn from(value: &str) -> Self {
        let (id, rest) = value.split_once(": ").unwrap();
        let (_, id) = id.split_once(" ").unwrap();
        let id = id.parse().unwrap();

        let items = rest.split(", ").fold(HashMap::new(), |mut acc, pair| {
            let (item, count) = pair.split_once(": ").unwrap();
            let item = item.to_string();
            let count = count.parse().unwrap();
            acc.insert(item, count);
            acc
        });

        Self { id, items }
    }
}

impl Sue {
    fn new(id: u32) -> Self {
        Self {
            id,
            items: HashMap::new(),
        }
    }

    fn parse_items(&mut self, items: &str) {
        let items = items.lines().fold(HashMap::new(), |mut acc, pair| {
            let (item, count) = pair.split_once(": ").unwrap();
            let item = item.to_string();
            let count = count.parse().unwrap();
            acc.insert(item, count);
            acc
        });

        self.items = items;
    }

    fn is_match_legacy(&self, other: &Sue) -> bool {
        self.items.iter().all(|(item, count)| {
            if other.items.contains_key(item) {
                other.items.get(item).unwrap() == count
            } else {
                true
            }
        })
    }

    fn is_match(&self, other: &Sue) -> bool {
        self.items.iter().all(|(item, count)| {
            if other.items.contains_key(item) {
                match item.as_str() {
                    "cats" | "trees" => other.items.get(item).unwrap() > count,
                    "pomeranians" | "goldfish" => other.items.get(item).unwrap() < count,
                    _ => other.items.get(item).unwrap() == count,
                }
            } else {
                true
            }
        })
    }
}

const REQUIRED_ITEMS: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
";

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let mut sue = Sue::new(0);
    sue.parse_items(REQUIRED_ITEMS);

    input
        .lines()
        .map(Sue::from)
        .find(|s| sue.is_match_legacy(s))
        .unwrap()
        .id
}

fn solve_second_part(input: &str) -> u32 {
    let mut sue = Sue::new(0);
    sue.parse_items(REQUIRED_ITEMS);

    input
        .lines()
        .map(Sue::from)
        .find(|s| sue.is_match(s))
        .unwrap()
        .id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_first_part() {
        assert!(true)
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        assert!(true)
    }

    check_answers!(40, 241);
}

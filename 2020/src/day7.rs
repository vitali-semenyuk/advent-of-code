use std::collections::HashMap;

pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    let bags = input.lines().fold(HashMap::new(), |mut hash, line| {
        let (bag, right) = line.split_once(" bags contain ").unwrap();
        let content: Vec<_> = right
            .split(", ")
            .map(|c| c.split_once(" ").unwrap().1.rsplit_once(" ").unwrap().0)
            .filter(|c| *c != "other")
            .collect();
        hash.insert(bag, content);
        hash
    });

    bags.values()
        .map(|b| count(&bags, b, "shiny gold"))
        .filter(|c| *c > 0)
        .count() as i64
}

fn solve_second_part(input: &str) -> i64 {
    42
}

fn count(hash: &HashMap<&str, Vec<&str>>, bags: &[&str], bag: &str) -> u32 {
    let c = if bags.contains(&bag) { 1 } else { 0 };
    bags.iter()
        .map(|b| count(&hash, hash.get(b).unwrap(), bag))
        .sum::<u32>()
        + c
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_first_part() {
        let answer = 4;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 126;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(185, 42);
}

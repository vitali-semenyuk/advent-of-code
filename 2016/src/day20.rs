use std::fmt::Display;

type IPAddress = u32;

#[derive(Debug)]
struct IPRange(IPAddress, IPAddress);

impl IPRange {
    fn intersects(&self, other: &IPRange) -> bool {
        self.0 < other.1 && self.1 > other.0
    }

    fn count(&self) -> u32 {
        self.1 - self.0 + 1
    }
}

impl From<&str> for IPRange {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        IPRange(start, end)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let mut ips = parse_ranges(input);
    ips.sort_by_key(|r| r.0);
    ips.windows(2).find(|tmp| tmp[1].0 - tmp[0].1 > 1).unwrap()[0].1 + 1
}

fn solve_second_part(input: &str) -> u32 {
    let ips = parse_ranges(input);
    u32::MAX - ips.iter().map(IPRange::count).sum::<u32>() + 1
}

fn parse_ranges(input: &str) -> Vec<IPRange> {
    input
        .lines()
        .map(IPRange::from)
        .fold(Vec::new(), |acc, range| {
            let (affected, mut unaffected): (Vec<_>, Vec<_>) =
                acc.into_iter().partition(|rng| range.intersects(rng));

            let new_range = merge_ranges(&affected, &range);

            unaffected.push(new_range);
            unaffected
        })
}

fn merge_ranges(ranges: &[IPRange], new_range: &IPRange) -> IPRange {
    let start = ranges
        .iter()
        .min_by_key(|r| r.0)
        .map_or(new_range.0, |v| new_range.0.min(v.0));
    let end = ranges
        .iter()
        .max_by_key(|r| r.1)
        .map_or(new_range.1, |v| new_range.1.max(v.1));

    IPRange(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5-8
0-2
4-7
";

    #[test]
    fn test_first_part() {
        let answer = 3;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2_i64;

        assert_eq!(answer - 9 + u32::MAX as i64, solve_second_part(INPUT) as i64)
    }

    #[test]
    fn test_ip_range_intersects() {
        assert!(IPRange(5, 8).intersects(&IPRange(4, 7)));
        assert!(IPRange(4, 7).intersects(&IPRange(5, 8)));
        assert!(!IPRange(0, 2).intersects(&IPRange(4, 7)));
        assert!(!IPRange(4, 7).intersects(&IPRange(0, 2)));
    }

    check_answers!(32259706, 113);
}

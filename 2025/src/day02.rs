use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u64 {
    input
        .trim()
        .split(",")
        .flat_map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            (start..=end).filter(|id| {
                let id_str = id.to_string();
                if id_str.len() % 2 != 0 {
                    return false;
                }

                let (a, b) = id_str.split_at(id_str.len() / 2);
                a == b
            })
        })
        .sum()
}

fn solve_second_part(input: &str) -> u64 {
    input
        .trim()
        .split(",")
        .flat_map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            (start..=end).filter(|id| {
                let id_str = id.to_string();
                let len = id_str.len();
                let seqs = id_str.chars().collect::<Vec<_>>();

                for parts in 2..=len {
                    if len % parts != 0 {
                        continue;
                    }

                    let mut seqs = seqs
                        .chunks(len / parts)
                        .map(|chunk| chunk.iter().collect::<String>());
                    let first = seqs.next().unwrap();

                    if seqs.all(|c| c == first) {
                        return true;
                    }
                }

                false
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_first_part() {
        let answer = 1227775554;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 4174379265;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(8576933996, 25663320831);
}

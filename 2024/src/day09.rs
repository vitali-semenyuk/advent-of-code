use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let blocks = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut result = 0;

    let mut l = 0;
    let mut r = blocks.len() / 2;
    let mut i = 0;

    let mut file_end = blocks[r * 2];

    while l < r {
        let file_start = blocks[l * 2];
        let empty = blocks[l * 2 + 1];

        for _ in 0..file_start {
            result += i * l;
            i += 1;
        }

        for _ in 0..empty {
            if file_end == 0 {
                r -= 1;
                file_end = blocks[r * 2];
            }

            file_end -= 1;
            result += i * r;
            i += 1;
        }

        l += 1;
    }

    for _ in 0..file_end {
        result += i * r;
        i += 1;
    }

    result
}

fn solve_second_part(input: &str) -> usize {
    let mut blocks = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut gaps = vec![Vec::new(); blocks.len() / 2];

    let mut result = 0;

    let mut r = blocks.len() / 2;

    loop {
        let file = blocks[r * 2];

        let mut i = 1;
        while i < r * 2 {
            if blocks[i] >= file {
                gaps[i / 2].push((r, file));
                blocks[i] -= file;
                blocks[r * 2] *= 10;
                break;
            }

            i += 2;
        }

        if r == 0 {
            break;
        }

        r -= 1;
    }

    let mut l = 0;
    for (i, block) in blocks.iter().enumerate() {
        if i % 2 == 0 {
            if *block < 10 {
                for _ in 0..(*block) {
                    result += l * i / 2;
                    l += 1;
                }
            } else {
                l += *block as usize / 10;
            }
        } else {
            let gap = gaps.get(i / 2).unwrap();
            for (b, b_size) in gap {
                for _ in 0..(*b_size) {
                    result += l * b;
                    l += 1;
                }
            }
            l += *block as usize;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402
";

    #[test]
    fn test_first_part() {
        let answer = 1928;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2858;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(6461289671426, 6488291456470);
}

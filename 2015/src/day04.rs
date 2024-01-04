use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let key = input.trim_end();
    mine(key, 5).expect("Mining unsuccessful")
}

fn solve_second_part(input: &str) -> u32 {
    let key = input.trim_end();
    mine(key, 6).expect("Mining unsuccessful")
}

fn mine(key: &str, padding: usize) -> Option<u32> {
    let whole_bytes = padding / 2;
    let has_leftover = padding % 2 != 0;
    let mask = [0x00].repeat(whole_bytes);

    for nonce in 0..10_000_000 {
        let hash = md5::compute(format!("{key}{nonce}"));

        if !hash.starts_with(&mask) {
            continue;
        }

        if has_leftover && hash[whole_bytes] & 0xF0 != 0 {
            continue;
        }

        return Some(nonce);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(609043, solve_first_part("abcdef"));
        assert_eq!(1048970, solve_first_part("pqrstuv"));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(6742839, solve_second_part("abcdef"));
        assert_eq!(5714438, solve_second_part("pqrstuv"));
    }

    check_answers!(346386, 9958218);
}

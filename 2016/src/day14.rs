use std::fmt::Display;

#[derive(Debug)]
struct Generator {
    salt: String,
    nonce: usize,
    rounds: u32,
    hashes: Vec<String>,
}

impl Generator {
    fn new(salt: &str, rounds: u32) -> Self {
        Self {
            salt: salt.to_string(),
            nonce: 0,
            rounds,
            hashes: Vec::new(),
        }
    }

    fn compute_hash(&mut self, nonce: usize) -> &str {
        if nonce >= self.hashes.len() {
            let hash = md5::compute(format!("{}{}", self.salt, nonce));
            let mut hash = format!("{:x}", hash);

            for _ in 0..self.rounds {
                let h = md5::compute(&hash);
                hash = format!("{:x}", h);
            }

            self.hashes.insert(nonce, hash);
        }

        &self.hashes[nonce]
    }
}

impl Iterator for Generator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let nonce = self.nonce;
            self.nonce += 1;
            let hash = self.compute_hash(nonce);
            let hash = hash.to_string();

            if let Some(char) = find_triplet(&hash) {
                let ts = char.to_string().repeat(5);

                for n in (nonce + 1)..=(nonce + 1000) {
                    let h = self.compute_hash(n);
                    let h = h.to_string();

                    if h.contains(&ts) {
                        return Some(hash);
                    }
                }
            }
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
    let salt = input.strip_suffix('\n').unwrap();
    let mut generator = Generator::new(salt, 0);

    generator.nth(63);

    generator.nonce - 1
}

fn solve_second_part(input: &str) -> usize {
    let salt = input.strip_suffix('\n').unwrap();
    let mut generator = Generator::new(salt, 2016);

    generator.nth(63);

    generator.nonce - 1
}

fn find_triplet(string: &str) -> Option<char> {
    for chars in string.chars().collect::<Vec<_>>().windows(3) {
        if chars[0] == chars[1] && chars[1] == chars[2] {
            return Some(chars[0]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc
";

    #[test]
    fn test_first_part() {
        let answer = 22728;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 22551;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(16106, 22423);
}

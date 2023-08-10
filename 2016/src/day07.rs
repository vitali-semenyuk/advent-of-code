use std::fmt::Display;

#[derive(Debug)]
struct IpV7 {
    sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl From<&str> for IpV7 {
    fn from(value: &str) -> Self {
        let mut sequences = Vec::new();
        let mut hypernet_sequences = Vec::new();
        let parts = value.split(&['[', ']']);

        for (index, string) in parts.enumerate() {
            if index % 2 == 0 {
                sequences.push(string.to_string())
            } else {
                hypernet_sequences.push(string.to_string())
            }
        }

        Self {
            sequences,
            hypernet_sequences,
        }
    }
}

impl IpV7 {
    fn supports_tls(&self) -> bool {
        self.sequences.iter().any(|seq| IpV7::is_abba_string(seq))
            && self
                .hypernet_sequences
                .iter()
                .all(|seq| !IpV7::is_abba_string(seq))
    }

    fn supports_ssl(&self) -> bool {
        self.sequences
            .iter()
            .map(|seq| IpV7::get_aba_strings(seq))
            .flatten()
            .any(|aba| {
                self.hypernet_sequences
                    .iter()
                    .any(|seq| IpV7::is_bab_pair(seq, &aba))
            })
    }

    fn is_abba_string(string: &str) -> bool {
        string
            .chars()
            .collect::<Vec<_>>()
            .windows(4)
            .any(|seq| seq[0] == seq[3] && seq[1] == seq[2] && seq[0] != seq[1])
    }

    fn get_aba_strings(string: &str) -> Vec<String> {
        let mut result = Vec::new();

        for seq in string.chars().collect::<Vec<_>>().windows(3) {
            if seq[0] == seq[2] && seq[0] != seq[1] {
                result.push(seq.iter().collect())
            }
        }

        result
    }

    fn is_bab_pair(string: &str, aba_string: &str) -> bool {
        let mut chars = aba_string.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let bab_string = [b, a, b].iter().collect::<String>();

        string
            .chars()
            .collect::<Vec<_>>()
            .windows(3)
            .any(|seq| seq.iter().collect::<String>() == bab_string)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input
        .lines()
        .map(IpV7::from)
        .filter(IpV7::supports_tls)
        .count()
}

fn solve_second_part(input: &str) -> usize {
    input
        .lines()
        .map(IpV7::from)
        .filter(IpV7::supports_ssl)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        const INPUT: &str = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
";
        let answer = 2;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        const INPUT: &str = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
";
        let answer = 3;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_supports_tls() {
        assert!(IpV7::from("abba[mnop]qrst").supports_tls());
        assert!(!IpV7::from("abcd[bddb]xyyx").supports_tls());
        assert!(!IpV7::from("aaaa[qwer]tyui").supports_tls());
        assert!(IpV7::from("ioxxoj[asdfgh]zxcvbn").supports_tls());
    }

    #[test]
    fn test_supports_ssl() {
        assert!(IpV7::from("aba[bab]xyz").supports_ssl());
        assert!(!IpV7::from("xyx[xyx]xyx").supports_ssl());
        assert!(IpV7::from("aaa[kek]eke").supports_ssl());
        assert!(IpV7::from("zazbz[bzb]cdb").supports_ssl());
    }

    check_answers!(118, 260);
}

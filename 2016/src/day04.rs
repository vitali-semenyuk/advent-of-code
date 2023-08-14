use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl From<&str> for Room {
    fn from(value: &str) -> Self {
        let (encrypted_name, rest) = value.rsplit_once('-').unwrap();
        let (sector_id, checksum) = rest.split_once('[').unwrap();
        let checksum = checksum.strip_suffix(']').unwrap();

        Self {
            encrypted_name: encrypted_name.to_string(),
            sector_id: sector_id.parse().unwrap(),
            checksum: checksum.to_string(),
        }
    }
}

impl Room {
    fn is_real(&self) -> bool {
        let chars = self.encrypted_name.chars().filter(|c| *c != '-').fold(
            HashMap::new(),
            |mut acc, char| {
                let count = acc.get(&char).unwrap_or(&0);
                acc.insert(char, count + 1);
                acc
            },
        );

        let mut chars_occurence = chars.iter().collect::<Vec<_>>();
        chars_occurence.sort_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)));
        chars_occurence.reverse();
        let top_chars: String = chars_occurence.iter().map(|t| t.0).take(5).collect();

        self.checksum == top_chars
    }

    fn name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|ch| cycle_char(ch, self.sector_id))
            .collect()
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    input
        .lines()
        .map(Room::from)
        .filter(Room::is_real)
        .map(|r| r.sector_id)
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
    input
        .lines()
        .map(Room::from)
        .find(|room| room.name() == "northpole object storage")
        .unwrap()
        .sector_id
}

fn cycle_char(ch: char, n: u32) -> char {
    if ch == '-' {
        return ' ';
    }

    let ord = ch as u32 - 'a' as u32;
    let ord = (ord + n) % 26 + 'a' as u32;

    (ord as u8) as char
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]
northpole-object-storage-26[fake]
";

    #[test]
    fn test_first_part() {
        let answer = 1514;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 26;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_room_is_real() {
        assert!(Room::from("aaaaa-bbb-z-y-x-123[abxyz]").is_real());
        assert!(Room::from("a-b-c-d-e-f-g-h-987[abcde]").is_real());
        assert!(Room::from("not-a-real-room-404[oarel]").is_real());
        assert!(!Room::from("totally-real-room-200[decoy]").is_real());
    }

    #[test]
    fn test_room_name() {
        assert_eq!(
            "very encrypted name",
            Room::from("qzmt-zixmtkozy-ivhz-343[]").name()
        )
    }

    check_answers!(361724, 482);
}

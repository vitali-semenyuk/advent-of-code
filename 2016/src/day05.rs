use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let door_id = input.strip_suffix("\n").unwrap();
    generate_passord_legacy(door_id, 8)
}

fn solve_second_part(input: &str) -> String {
    let door_id = input.strip_suffix("\n").unwrap();
    generate_passord(door_id, 8)
}

fn generate_passord_legacy(id: &str, len: usize) -> String {
    let mut password = String::new();
    let mut nonce = 0;

    for _ in 0..len {
        let char = mine(id, 5, &mut nonce).chars().nth(5).unwrap();
        password.push(char);
    }

    password
}

fn generate_passord(id: &str, len: usize) -> String {
    let mut password = vec![None; len];
    let mut nonce = 0;

    loop {
        let hash = mine(id, 5, &mut nonce);
        let position = hash.chars().nth(5).unwrap();
        let char = hash.chars().nth(6).unwrap();

        if let Some(position) = position.to_digit(10) {
            let position = position as usize;
            if (0..len).contains(&position) && password[position].is_none() {
                password[position] = Some(char);
            }
        }

        if password.iter().all(Option::is_some) {
            break;
        }
    }

    let password: Option<String> = password.into_iter().collect();
    password.unwrap()
}

fn mine(key: &str, padding: usize, nonce: &mut u32) -> String {
    let whole_bytes = padding / 2;
    let has_leftover = padding % 2 != 0;
    let mask = [0x00].repeat(whole_bytes);

    loop {
        let hash = md5::compute(format!("{key}{nonce}"));
        *nonce += 1;

        if !hash.starts_with(&mask) {
            continue;
        }

        if has_leftover && hash[whole_bytes] & 0xF0 != 0 {
            continue;
        }

        return format!("{:x}", hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc
";

    #[ignore]
    #[test]
    fn test_first_part() {
        let answer = "18f47a30";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "05ace8e3";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!("801b56a7", "424a0197");
}

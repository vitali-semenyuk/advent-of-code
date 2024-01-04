use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let initial_state = input.trim_end();
    fill_disk(272, initial_state)
}

fn solve_second_part(input: &str) -> String {
    let initial_state = input.trim_end();
    fill_disk(35651584, initial_state)
}

fn fill_disk(size: usize, initial_state: &str) -> String {
    let mut data = initial_state.to_string();

    while data.len() < size {
        data = generate_random(&data);
    }

    data = data.chars().take(size).collect();

    calculate_checksum(&data)
}

fn generate_random(initial_state: &str) -> String {
    let a = initial_state;
    let b = initial_state
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect::<String>();

    format!("{a}0{b}")
}

fn calculate_checksum(data: &str) -> String {
    let string = data
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
        .collect::<String>();

    if string.len() % 2 == 0 {
        return calculate_checksum(&string);
    }

    string
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "10000
";

    #[test]
    fn test_first_part() {
        let answer = "11010011110011010";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "10111110011110111";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_fill_disk() {
        assert_eq!("01100", fill_disk(20, "10000"));
    }

    #[test]
    fn test_generate_random() {
        assert_eq!("100", generate_random("1"));
        assert_eq!("001", generate_random("0"));
        assert_eq!("11111000000", generate_random("11111"));
        assert_eq!("1111000010100101011110000", generate_random("111100001010"));
    }

    #[test]
    fn test_calculate_checksum() {
        assert_eq!("100", calculate_checksum("110010110100"));
    }

    check_answers!("10010101010011101", "01100111101101111");
}

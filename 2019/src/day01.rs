use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    input
        .lines()
        .map(|l| calculate_required_fuel(l.parse::<u32>().unwrap()))
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
    input
        .lines()
        .map(|l| calculate_full_fuel(l.parse::<u32>().unwrap()))
        .sum()
}

fn calculate_required_fuel(mass: u32) -> u32 {
    let fuel = mass as i32 / 3 - 2;
    fuel.max(0) as u32
}

fn calculate_full_fuel(mass: u32) -> u32 {
    let mut total = calculate_required_fuel(mass);

    if total > 0 {
        total += calculate_full_fuel(total)
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "12
14
1969
100756
";

    #[test]
    fn test_first_part() {
        let answer = 34241;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 51331;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_calculate_required_fuel() {
        assert_eq!(calculate_required_fuel(12), 2);
        assert_eq!(calculate_required_fuel(14), 2);
        assert_eq!(calculate_required_fuel(1969), 654);
        assert_eq!(calculate_required_fuel(100756), 33583);
        assert_eq!(calculate_required_fuel(1), 0);
    }

    #[test]
    fn test_calculate_full_fuel() {
        assert_eq!(calculate_full_fuel(14), 2);
        assert_eq!(calculate_full_fuel(1969), 966);
        assert_eq!(calculate_full_fuel(100756), 50346);
        assert_eq!(calculate_full_fuel(1), 0);
    }

    check_answers!(3465245, 5194970);
}

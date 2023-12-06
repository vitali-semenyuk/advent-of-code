use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();
    let (_, times) = times.split_once(':').unwrap();
    let (_, distances) = distances.split_once(':').unwrap();
    let times = times
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap());
    let distances = distances
        .split_ascii_whitespace()
        .map(|v| v.parse::<u64>().unwrap());

    times
        .zip(distances)
        .map(|(time, distance)| solution(time, distance))
        .product()
}

fn solve_second_part(input: &str) -> u32 {
    let mut lines = input.lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let (_, time) = time.split_once(':').unwrap();
    let (_, distance) = distance.split_once(':').unwrap();
    let time = time
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distance
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    solution(time, distance)
}

fn solution(time: u64, distance: u64) -> u32 {
    let (from, to) = solve_quadratic(1.0, -(time as f64), distance as f64);

    let mut count = to.floor() as u32 - from.ceil() as u32 + 1;

    if from == from.floor() {
        count -= 1;
    }
    if to == to.floor() {
        count -= 1;
    }

    count
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = b.powf(2.0) - 4.0 * a * c;
    let sqrt_d = d.sqrt();

    let x1 = (-b - sqrt_d) / (2.0 * a);
    let x2 = (-b + sqrt_d) / (2.0 * a);

    (x1, x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_first_part() {
        let answer = 288;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 71503;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(2269432, 35865985);
}

use std::fmt::Display;

#[derive(Debug, Clone)]
struct Disc {
    positions: u32,
    current_position: u32,
}

impl Disc {
    fn new(positions: u32, current_position: u32) -> Self {
        Self {
            positions,
            current_position,
        }
    }

    fn tick(&mut self) {
        self.advance(1)
    }

    fn advance(&mut self, time: u32) {
        self.current_position += time;
        self.current_position %= self.positions;
    }
}

impl From<&str> for Disc {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        let positions = parts.nth(3).unwrap().parse().unwrap();
        let current_position = parts
            .nth(7)
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .parse()
            .unwrap();

        Self {
            positions,
            current_position,
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let discs = input.lines().map(Disc::from).collect::<Vec<_>>();

    simulate_all(&discs)
}

fn solve_second_part(input: &str) -> u32 {
    let mut discs = input.lines().map(Disc::from).collect::<Vec<_>>();

    discs.push(Disc::new(11, 0));

    simulate_all(&discs)
}

fn simulate_all(discs: &[Disc]) -> u32 {
    let mut dt = 0;

    loop {
        if simulate(&discs, dt) {
            break;
        }
        dt += 1;
    }

    dt
}

fn simulate(discs: &[Disc], initial_time: u32) -> bool {
    let mut discs = discs.to_vec();

    for disc in discs.iter_mut() {
        disc.advance(initial_time)
    }

    for i in 0..discs.len() {
        for disc in discs.iter_mut() {
            disc.tick()
        }
        let current_disc = &discs[i];
        if current_disc.current_position != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.
";

    #[test]
    fn test_first_part() {
        let answer = 5;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 85;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(16824, 3543984);
}

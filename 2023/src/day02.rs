use std::fmt::Display;

#[derive(Debug, Clone)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Set>,
}

impl Game {
    fn is_valid(&self, cubes: &Set) -> bool {
        self.rounds.iter().all(|round| {
            round.red <= cubes.red && round.green <= cubes.green && round.blue <= cubes.blue
        })
    }

    fn get_all_cubes(&self) -> Set {
        self.rounds.iter().fold(Set::new(0, 0, 0), |acc, cubes| {
            let red = acc.red.max(cubes.red);
            let green = acc.green.max(cubes.green);
            let blue = acc.blue.max(cubes.blue);

            Set::new(red, green, blue)
        })
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, sets) = value.split_once(": ").unwrap();
        let id = game.split_once(' ').unwrap().1.parse().unwrap();

        let rounds = sets
            .split("; ")
            .map(|set_str| {
                let mut set = Set::new(0, 0, 0);
                for cubes in set_str.split(", ") {
                    let (count, color) = cubes.split_once(' ').unwrap();
                    let count = count.parse().unwrap();

                    match color {
                        "red" => set.red = count,
                        "green" => set.green = count,
                        "blue" => set.blue = count,
                        _ => panic!("Invalid color value"),
                    }
                }
                set
            })
            .collect();

        Self { id, rounds }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let cubes = Set::new(12, 13, 14);

    input
        .lines()
        .map(Game::from)
        .filter(|game| game.is_valid(&cubes))
        .map(|game| game.id)
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
    input
        .lines()
        .map(Game::from)
        .map(|game| game.get_all_cubes())
        .map(|set| set.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_first_part() {
        let answer = 8;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2286;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(2076, 70950);
}

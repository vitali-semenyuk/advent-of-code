use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug)]
enum Destination {
    Bot(u32),
    Output(u32),
}

#[derive(Debug)]
enum Instruction {
    InitialValue {
        bot_id: u32,
        value: u32,
    },
    BotRule {
        bot_id: u32,
        low: Destination,
        high: Destination,
    },
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        match parts.next().unwrap() {
            "value" => {
                let value = parts.next().unwrap().parse().unwrap();
                let bot_id = parts.nth(3).unwrap().parse().unwrap();
                Self::InitialValue { bot_id, value }
            }
            "bot" => {
                let bot_id = parts.next().unwrap().parse().unwrap();
                let destination_class = match parts.nth(3).unwrap() {
                    "bot" => Destination::Bot,
                    "output" => Destination::Output,
                    _ => panic!("Unexpected value"),
                };
                let destination = parts.next().unwrap().parse().unwrap();
                let low = destination_class(destination);
                let destination_class = match parts.nth(3).unwrap() {
                    "bot" => Destination::Bot,
                    "output" => Destination::Output,
                    _ => panic!("Unexpected value"),
                };
                let destination = parts.next().unwrap().parse().unwrap();
                let high = destination_class(destination);

                Self::BotRule { bot_id, low, high }
            }
            _ => panic!("Unexpected value"),
        }
    }
}

#[derive(Debug)]
struct Bot {
    inputs: HashSet<u32>,
}

impl Bot {
    fn new() -> Self {
        Self {
            inputs: HashSet::new(),
        }
    }

    fn is_ready(&self) -> bool {
        self.inputs.len() == 2
    }

    fn get_high(&self) -> u32 {
        *self.inputs.iter().max().unwrap()
    }

    fn get_low(&self) -> u32 {
        *self.inputs.iter().min().unwrap()
    }

    fn add_input(&mut self, value: u32) {
        self.inputs.insert(value);
    }
}

#[derive(Debug)]
struct Factory {
    bots: HashMap<u32, Bot>,
    outputs: HashMap<u32, u32>,
}

impl Factory {
    fn new() -> Self {
        Self {
            bots: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn initialize(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            if let Instruction::InitialValue { bot_id, value } = *instruction {
                self.set_value(bot_id, value)
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.bots.values().all(Bot::is_ready)
    }

    fn update(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            if let Instruction::BotRule { bot_id, low, high } = instruction {
                if let Some(source_bot) = self.bots.get(bot_id) {
                    if !source_bot.is_ready() {
                        continue;
                    }

                    let low_value = source_bot.get_low();
                    let high_value = source_bot.get_high();

                    match low {
                        Destination::Bot(low_bot_id) => self.set_value(*low_bot_id, low_value),
                        Destination::Output(out) => {
                            self.outputs.insert(*out, low_value);
                        }
                    };

                    match high {
                        Destination::Bot(high_bot_id) => self.set_value(*high_bot_id, high_value),
                        Destination::Output(out) => {
                            self.outputs.insert(*out, high_value);
                        }
                    }
                }
            }
        }
    }

    fn find_bot(&self, a: u32, b: u32) -> Option<u32> {
        self.bots
            .iter()
            .find(|(_, bot)| bot.inputs.contains(&a) && bot.inputs.contains(&b))
            .map(|(id, _)| *id)
    }

    fn product(&self) -> u32 {
        self.outputs.get(&0).unwrap()
            * self.outputs.get(&1).unwrap()
            * self.outputs.get(&2).unwrap()
    }

    fn set_value(&mut self, bot_id: u32, value: u32) {
        if let Entry::Vacant(e) = self.bots.entry(bot_id) {
            let mut bot = Bot::new();
            bot.add_input(value);
            e.insert(bot);
        } else {
            let bot = self.bots.get_mut(&bot_id).unwrap();
            bot.add_input(value);
        };
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let factory = setup_factory(input);

    factory.find_bot(61, 17).unwrap_or(0)
}

fn solve_second_part(input: &str) -> u32 {
    let factory = setup_factory(input);

    factory.product()
}

fn setup_factory(input: &str) -> Factory {
    let instructions = input.lines().map(Instruction::from).collect::<Vec<_>>();

    let mut factory = Factory::new();
    factory.initialize(&instructions);

    loop {
        factory.update(&instructions);

        if factory.is_finished() {
            break;
        }
    }

    factory
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
";

    #[test]
    fn test_first_part() {
        let answer = 0;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 30;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_find_bot() {
        let factory = setup_factory(INPUT);

        assert_eq!(Some(2), factory.find_bot(5, 2))
    }

    check_answers!(113, 12803);
}

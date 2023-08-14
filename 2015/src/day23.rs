use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            r => Err(ParseError(format!("Unsupported register name: '{r}'"))),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpEven(Register, i32),
    JumpOne(Register, i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, arguments) = s
            .split_once(' ')
            .ok_or(ParseError(format!("Expected instruction, got '{s}'")))?;
        let mut arguments = arguments.split(", ");

        let result = match instruction {
            "hlf" => {
                let register = arguments
                    .next()
                    .ok_or(ParseError("Expected register name".to_string()))?;
                Self::Half(Register::from_str(register)?)
            }
            "tpl" => {
                let register = arguments
                    .next()
                    .ok_or(ParseError("Expected register name".to_string()))?;
                Self::Triple(Register::from_str(register)?)
            }
            "inc" => {
                let register = arguments
                    .next()
                    .ok_or(ParseError("Expected register name".to_string()))?;
                Self::Increment(Register::from_str(register)?)
            }
            "jmp" => {
                let offset = arguments
                    .next()
                    .ok_or(ParseError("Expected offset".to_string()))?;
                let offset = offset
                    .parse()
                    .or(Err(ParseError(format!("Invalid offset: {offset}"))))?;
                Self::Jump(offset)
            }
            "jie" => {
                let register = arguments
                    .next()
                    .ok_or(ParseError("Expected register name".to_string()))?;
                let offset = arguments
                    .next()
                    .ok_or(ParseError("Expected offset".to_string()))?;
                let offset = offset
                    .parse()
                    .or(Err(ParseError(format!("Invalid offset: {offset}"))))?;
                Self::JumpEven(Register::from_str(register)?, offset)
            }
            "jio" => {
                let register = arguments
                    .next()
                    .ok_or(ParseError("Expected register name".to_string()))?;
                let offset = arguments
                    .next()
                    .ok_or(ParseError("Expected offset".to_string()))?;
                let offset = offset
                    .parse()
                    .or(Err(ParseError(format!("Invalid offset: {offset}"))))?;
                Self::JumpOne(Register::from_str(register)?, offset)
            }
            _ => panic!("Unknown instruction: {instruction}"),
        };

        Ok(result)
    }
}

#[derive(Debug)]
struct Computer {
    a: u32,
    b: u32,
    ip: usize,
}

impl Computer {
    fn new(a: u32, b: u32) -> Self {
        Self { a, b, ip: 0 }
    }

    fn evaluate(&mut self, instructions: &[Instruction]) {
        loop {
            let instruction = instructions.get(self.ip);
            if instruction.is_none() {
                break;
            }

            match instruction.unwrap() {
                Instruction::Half(r) => self.set_register(r, self.get_register(r) / 2),
                Instruction::Triple(r) => self.set_register(r, self.get_register(r) * 3),
                Instruction::Increment(r) => self.set_register(r, self.get_register(r) + 1),
                Instruction::Jump(n) => {
                    self.jump(*n);
                    continue;
                }
                Instruction::JumpEven(r, n) => {
                    if self.get_register(r) % 2 == 0 {
                        self.jump(*n);
                        continue;
                    }
                }
                Instruction::JumpOne(r, n) => {
                    if self.get_register(r) == 1 {
                        self.jump(*n);
                        continue;
                    }
                }
            };

            self.ip += 1;
        }
    }

    fn get_register(&self, register: &Register) -> u32 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn set_register(&mut self, register: &Register, value: u32) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }

    fn jump(&mut self, offset: i32) {
        let ip = self.ip as i32 + offset;
        if ip < 0 {
            self.ip = usize::MAX
        } else {
            self.ip = ip as usize
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
    match evaluate(input, 0) {
        Ok((_, b)) => b,
        Err(error) => panic!("{error}"),
    }
}

fn solve_second_part(input: &str) -> u32 {
    match evaluate(input, 1) {
        Ok((_, b)) => b,
        Err(error) => panic!("{error}"),
    }
}

fn evaluate(source_code: &str, register_a: u32) -> Result<(u32, u32), ParseError> {
    let instructions = parse(source_code)?;

    let mut computer = Computer::new(register_a, 0);
    computer.evaluate(&instructions);

    Ok((computer.a, computer.b))
}

fn parse(source_code: &str) -> Result<Vec<Instruction>, ParseError> {
    source_code.lines().map(Instruction::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "inc b
jio b, +2
tpl b
inc b
";

    #[test]
    fn test_first_part() {
        let answer = 2;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 2;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(170, 247);
}

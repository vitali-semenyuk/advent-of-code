use std::{collections::HashMap, fmt::Display, str::FromStr};

type Integer = i32;

#[derive(Debug)]
struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            r => Err(ParseError(format!("Unsupported register name: '{r}'"))),
        }
    }
}

#[derive(Debug)]
enum Value {
    Register(Register),
    Literal(Integer),
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let register = Register::from_str(s);
        if let Ok(register) = register {
            Ok(Self::Register(register))
        } else {
            let literal = s
                .parse()
                .or(Err(ParseError(format!("Invalid value: {s}"))))?;
            Ok(Self::Literal(literal))
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    Jump(Value, Integer),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        match parts
            .next()
            .ok_or(ParseError("Expected instruction".to_string()))?
        {
            "cpy" => {
                let x = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;
                let y = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;

                Ok(Self::Copy(Value::from_str(x)?, Register::from_str(y)?))
            }
            "inc" => {
                let x = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;

                Ok(Self::Increment(Register::from_str(x)?))
            }
            "dec" => {
                let x = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;

                Ok(Self::Decrement(Register::from_str(x)?))
            }
            "jnz" => {
                let x = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;
                let y = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;
                let y = y
                    .parse()
                    .or(Err(ParseError(format!("Invalid numeric literal: '{y}'"))))?;

                Ok(Self::Jump(Value::from_str(x)?, y))
            }
            i => Err(ParseError(format!("Unknown instruction: '{i}'"))),
        }
    }
}

#[derive(Debug)]
struct Computer {
    registers: HashMap<Register, Integer>,
    ip: usize,
}

impl Computer {
    fn new(c: Integer) -> Self {
        let registers = HashMap::from([
            (Register::A, 0),
            (Register::B, 0),
            (Register::C, c),
            (Register::D, 0),
        ]);

        Self { registers, ip: 0 }
    }

    fn evaluate(&mut self, instructions: &[Instruction]) {
        loop {
            let instruction = instructions.get(self.ip);
            if instruction.is_none() {
                break;
            }

            match instruction.unwrap() {
                Instruction::Copy(x, y) => self.set_register(y, self.get_value(x)),
                Instruction::Increment(x) => self.set_register(x, self.get_register(x) + 1),
                Instruction::Decrement(x) => self.set_register(x, self.get_register(x) - 1),
                Instruction::Jump(x, y) => {
                    if self.get_value(x) != 0 {
                        self.jump(*y);
                        continue;
                    }
                }
            }

            self.ip += 1;
        }
    }

    fn get_register(&self, register: &Register) -> Integer {
        *self
            .registers
            .get(register)
            .expect("Uninitialized register")
    }

    fn set_register(&mut self, register: &Register, value: Integer) {
        let register = self
            .registers
            .get_mut(register)
            .expect("Uninitialized register");

        *register = value;
    }

    fn get_value(&self, value: &Value) -> Integer {
        match value {
            Value::Register(r) => self.get_register(r),
            Value::Literal(n) => *n,
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

fn solve_first_part(input: &str) -> i32 {
    match evaluate(input, 0) {
        Ok(a) => a,
        Err(error) => panic!("{error}"),
    }
}

fn solve_second_part(input: &str) -> i32 {
    match evaluate(input, 1) {
        Ok(a) => a,
        Err(error) => panic!("{error}"),
    }
}

fn evaluate(source_code: &str, c: Integer) -> Result<Integer, ParseError> {
    let instructions = parse(source_code)?;

    let mut computer = Computer::new(c);
    computer.evaluate(&instructions);

    Ok(computer.get_register(&Register::A))
}

fn parse(source_code: &str) -> Result<Vec<Instruction>, ParseError> {
    source_code.lines().map(Instruction::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
";

    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(318077, 9227731);
}

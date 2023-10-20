use std::{collections::HashMap, fmt::Display, str::FromStr};

pub type Integer = i32;

#[derive(Debug)]
pub struct ParseError(String);

pub type ExecutionResult = Result<Integer, ParseError>;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Register {
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

#[derive(Debug, Clone)]
pub enum Value {
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

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy(Value, Value),
    Increment(Register),
    Decrement(Register),
    Jump(Value, Value),
    Toggle(Register),
    Multiply(Register, Value),
    Noop,
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

                Ok(Self::Copy(Value::from_str(x)?, Value::from_str(y)?))
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

                Ok(Self::Jump(Value::from_str(x)?, Value::from_str(y)?))
            }
            "tgl" => {
                let x = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;
                Ok(Self::Toggle(Register::from_str(x)?))
            }
            "mul" => {
                let x = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;
                let y = parts
                    .next()
                    .ok_or(ParseError("Expected argument".to_string()))?;

                Ok(Self::Multiply(Register::from_str(x)?, Value::from_str(y)?))
            }
            "nop" => Ok(Self::Noop),

            i => Err(ParseError(format!("Unknown instruction: '{i}'"))),
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    registers: HashMap<Register, Integer>,
    ip: usize,
}

impl Computer {
    pub fn new() -> Self {
        let registers = HashMap::from([
            (Register::A, 0),
            (Register::B, 0),
            (Register::C, 0),
            (Register::D, 0),
        ]);

        Self { registers, ip: 0 }
    }

    pub fn evaluate(&mut self, instructions: &[Instruction]) {
        let mut instructions = instructions.to_vec();

        loop {
            let instruction = instructions.get(self.ip);
            if instruction.is_none() {
                break;
            }

            match instruction.unwrap() {
                Instruction::Copy(x, y) => match y {
                    Value::Register(r) => self.set_register(r, self.get_value(x)),
                    Value::Literal(_) => {}
                },
                Instruction::Increment(x) => self.set_register(x, self.get_register(x) + 1),
                Instruction::Decrement(x) => self.set_register(x, self.get_register(x) - 1),
                Instruction::Jump(x, y) => {
                    let offset = self.get_value(y);
                    if self.get_value(x) != 0 {
                        self.jump(offset);
                        continue;
                    }
                }
                Instruction::Toggle(x) => {
                    let offset = self.get_register(x);
                    let target_address = self.ip as i32 + offset;

                    if target_address >= 0 {
                        let target_address = target_address as usize;
                        let target_instruction = instructions.get(target_address);
                        if let Some(target_instruction) = target_instruction {
                            let new_instruction = match target_instruction {
                                Instruction::Copy(x, y) => Instruction::Jump(x.clone(), y.clone()),
                                Instruction::Increment(x) => Instruction::Decrement(x.clone()),
                                Instruction::Decrement(x) => Instruction::Increment(x.clone()),
                                Instruction::Jump(x, y) => Instruction::Copy(x.clone(), y.clone()),
                                Instruction::Toggle(x) => Instruction::Increment(x.clone()),
                                instr => instr.clone(),
                            };
                            instructions[target_address] = new_instruction;
                        }
                    }
                }
                Instruction::Multiply(x, y) => {
                    self.set_register(x, self.get_register(x) * self.get_value(y))
                }
                Instruction::Noop => {}
            }

            self.ip += 1;
        }
    }

    pub fn get_register(&self, register: &Register) -> Integer {
        *self
            .registers
            .get(register)
            .expect("Uninitialized register")
    }

    pub fn set_register(&mut self, register: &Register, value: Integer) {
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

pub fn parse_code(source_code: &str) -> Result<Vec<Instruction>, ParseError> {
    source_code.lines().map(Instruction::from_str).collect()
}

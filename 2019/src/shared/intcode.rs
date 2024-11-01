use std::{collections::VecDeque, fmt::Display};

const OPTCODE_ADD: i32 = 1;
const OPTCODE_MULTIPLY: i32 = 2;
const OPTCODE_INPUT: i32 = 3;
const OPTCODE_OUTPUT: i32 = 4;
const OPTCODE_JUMP_IF_TRUE: i32 = 5;
const OPTCODE_JUMP_IF_FALSE: i32 = 6;
const OPTCODE_LESS_THAN: i32 = 7;
const OPTCODE_EQUALS: i32 = 8;
const OPTCODE_HALT: i32 = 99;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidInstruction { ip: usize, optcode: i32 },
    ArgumentError { ip: usize, n: usize },
    InvalidAddress { ip: usize, address: usize },
    MissingInput { ip: usize },
    AbruptHalt,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           RuntimeError::InvalidInstruction { ip, optcode } => write!(
                f,
                "Invalid instruction: instruction #{ip} with unknown optcode {optcode}"
            ),
            RuntimeError::ArgumentError { ip, n } => write!(f, "ArgumentError: instruction #{ip} requires argument at position {n}"),
            RuntimeError::InvalidAddress { ip, address } => {
                write!(
                    f,
                    "Invalid memory address access: instruction #{ip} tried to access memory @ {address}"
                )
            }
            RuntimeError::MissingInput { ip } => write!(
                f,
                "Missing input: instruction #{ip} expected an input value"
            ),
            RuntimeError::AbruptHalt => write!(f, "Abrupt halt: execution reached the end of instructions set without 'halt' being called"),
        }
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Operation(i32);

impl Operation {
    fn optcode(&self) -> i32 {
        self.0 % 100
    }

    fn parameter_mode(&self, position: u32) -> Option<ParameterMode> {
        let flag = self.0 as u32 % 10_u32.pow(position + 2) / 10_u32.pow(position + 1);

        match flag {
            0 => Some(ParameterMode::Position),
            1 => Some(ParameterMode::Immediate),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intcode {
    memory: Vec<i32>,
    ip: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    debug: bool,
    halted: bool,
}

impl Intcode {
    fn new(memory: Vec<i32>) -> Self {
        Self {
            memory,
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            debug: false,
            halted: false,
        }
    }

    pub fn run(&mut self) -> Result<&Self, RuntimeError> {
        let mut ip = self.ip;

        while !self.halted {
            let optcode = self.get(ip).ok_or(RuntimeError::AbruptHalt)?;
            let operation = Operation(optcode);
            let optcode = operation.optcode();

            match optcode {
                OPTCODE_ADD => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;
                    let arg2 = self.get_argument(&operation, ip, 2)?;
                    let arg3 = self.get_plain_argument(ip, 3)?;

                    self.set(arg3 as usize, arg1 + arg2)
                        .ok_or(RuntimeError::InvalidAddress {
                            ip,
                            address: arg3 as usize,
                        })?;

                    if self.debug {
                        println!("> [#{ip}] Executing ADD operation (optcode {optcode}) with arguments {arg1}/{arg2}/{arg3}")
                    }

                    ip += 4;
                }
                OPTCODE_MULTIPLY => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;
                    let arg2 = self.get_argument(&operation, ip, 2)?;
                    let arg3 = self.get_plain_argument(ip, 3)?;

                    self.set(arg3 as usize, arg1 * arg2)
                        .ok_or(RuntimeError::InvalidAddress {
                            ip,
                            address: arg3 as usize,
                        })?;

                    if self.debug {
                        println!("> [#{ip}] Executing MULTIPLY operation (optcode {optcode}) with arguments {arg1}/{arg2}/{arg3}")
                    }

                    ip += 4;
                }
                OPTCODE_INPUT => {
                    let arg1 = self.get_plain_argument(ip, 1)?;

                    let input = self
                        .input
                        .pop_front()
                        .ok_or(RuntimeError::MissingInput { ip })?;

                    self.set(arg1 as usize, input)
                        .ok_or(RuntimeError::InvalidAddress {
                            ip,
                            address: arg1 as usize,
                        })?;

                    if self.debug {
                        println!("> [#{ip}] Executing INPUT operation (optcode {optcode}) with argument {arg1}")
                    }

                    ip += 2;
                }
                OPTCODE_OUTPUT => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;

                    self.output.push_back(arg1);

                    if self.debug {
                        println!("> [#{ip}] Executing OUTPUT operation (optcode {optcode}) with argument {arg1}")
                    }

                    ip += 2;
                }
                OPTCODE_JUMP_IF_TRUE => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;
                    let arg2 = self.get_argument(&operation, ip, 2)?;

                    if self.debug {
                        println!("> [#{ip}] Executing JUMP_IF_TRUE operation (optcode {optcode}) with arguments {arg1}/{arg2}")
                    }

                    if arg1 != 0 {
                        ip = arg2 as usize;
                    } else {
                        ip += 3;
                    }
                }
                OPTCODE_JUMP_IF_FALSE => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;
                    let arg2 = self.get_argument(&operation, ip, 2)?;

                    if self.debug {
                        println!("> [#{ip}] Executing JUMP_IF_FALSE operation (optcode {optcode}) with arguments {arg1}/{arg2}")
                    }

                    if arg1 == 0 {
                        ip = arg2 as usize;
                    } else {
                        ip += 3;
                    }
                }
                OPTCODE_LESS_THAN => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;
                    let arg2 = self.get_argument(&operation, ip, 2)?;
                    let arg3 = self.get_plain_argument(ip, 3)?;

                    let value = if arg1 < arg2 { 1 } else { 0 };

                    self.set(arg3 as usize, value)
                        .ok_or(RuntimeError::InvalidAddress {
                            ip,
                            address: arg3 as usize,
                        })?;

                    if self.debug {
                        println!("> [#{ip}] Executing LESS_THAN operation (optcode {optcode}) with arguments {arg1}/{arg2}/{arg3}")
                    }

                    ip += 4;
                }
                OPTCODE_EQUALS => {
                    let arg1 = self.get_argument(&operation, ip, 1)?;
                    let arg2 = self.get_argument(&operation, ip, 2)?;
                    let arg3 = self.get_plain_argument(ip, 3)?;

                    let value = if arg1 == arg2 { 1 } else { 0 };

                    self.set(arg3 as usize, value)
                        .ok_or(RuntimeError::InvalidAddress {
                            ip,
                            address: arg3 as usize,
                        })?;

                    if self.debug {
                        println!("> [#{ip}] Executing EQUALS operation (optcode {optcode}) with arguments {arg1}/{arg2}/{arg3}")
                    }

                    ip += 4;
                }
                OPTCODE_HALT => {
                    if self.debug {
                        println!("> [#{ip}] Executing HALT operation (optcode {optcode})\n")
                    }

                    self.halted = true;
                }
                _ => return Err(RuntimeError::InvalidInstruction { optcode, ip }),
            }

            self.ip = ip;
        }

        Ok(self)
    }

    pub fn get(&self, address: usize) -> Option<i32> {
        self.memory.get(address).copied()
    }

    pub fn set(&mut self, address: usize, value: i32) -> Option<()> {
        if let Some(address) = self.memory.get_mut(address) {
            *address = value;
            Some(())
        } else {
            None
        }
    }

    pub fn input(&mut self, value: i32) {
        self.input.push_back(value);
    }

    pub fn output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.ip = 0;
        self.halted = false;
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    #[allow(dead_code)]
    pub fn set_debug(&mut self, value: bool) {
        self.debug = value;
    }

    fn get_argument(
        &self,
        operation: &Operation,
        ip: usize,
        n: usize,
    ) -> Result<i32, RuntimeError> {
        let argument = self.get_plain_argument(ip, n)?;

        match operation
            .parameter_mode(n as u32)
            .ok_or(RuntimeError::ArgumentError { ip, n })?
        {
            ParameterMode::Position => {
                self.get(argument as usize)
                    .ok_or(RuntimeError::InvalidAddress {
                        ip,
                        address: argument as usize,
                    })
            }
            ParameterMode::Immediate => Ok(argument),
        }
    }

    fn get_plain_argument(&self, ip: usize, n: usize) -> Result<i32, RuntimeError> {
        self.get(ip + n)
            .ok_or(RuntimeError::ArgumentError { ip, n })
    }
}

impl From<&str> for Intcode {
    fn from(value: &str) -> Self {
        let memory = value
            .trim()
            .split(',')
            .map(|c| c.parse().expect("Invalid memory value"))
            .collect();

        Self::new(memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_run() {
        assert_eq!(
            Intcode::from("1,0,0,0,99").run().unwrap().memory,
            vec![2, 0, 0, 0, 99]
        );

        assert_eq!(
            Intcode::from("2,3,0,3,99").run().unwrap().memory,
            vec![2, 3, 0, 6, 99]
        );

        assert_eq!(
            Intcode::from("2,4,4,5,99,0").run().unwrap().memory,
            vec![2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            Intcode::from("1,1,1,4,99,5,6,0,99").run().unwrap().memory,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );

        assert_eq!(
            Intcode::from("1,9,10,3,2,3,11,0,99,30,40,50")
                .run()
                .unwrap()
                .memory,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );

        // IO
        let mut intcode = Intcode::from("3,0,4,0,99");
        intcode.input(1337);

        assert_eq!(intcode.run().unwrap().memory, vec![1337, 0, 4, 0, 99]);
        assert_eq!(intcode.output().unwrap(), 1337);

        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let mut intcode = Intcode::from("3,9,8,9,10,9,4,9,99,-1,8");

        intcode.input(7);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 0, 8]
        );
        assert_eq!(intcode.output().unwrap(), 0);

        intcode.reset();
        intcode.input(8);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, 1, 8]
        );
        assert_eq!(intcode.output().unwrap(), 1);

        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let mut intcode = Intcode::from("3,9,7,9,10,9,4,9,99,-1,8");

        intcode.input(7);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 1, 8]
        );
        assert_eq!(intcode.output().unwrap(), 1);

        intcode.reset();
        intcode.input(9);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8]
        );
        assert_eq!(intcode.output().unwrap(), 0);

        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let mut intcode = Intcode::from("3,3,1108,-1,8,3,4,3,99");

        intcode.input(7);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 3, 1108, 0, 8, 3, 4, 3, 99]
        );
        assert_eq!(intcode.output().unwrap(), 0);

        intcode.reset();
        intcode.input(8);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 3, 1108, 1, 8, 3, 4, 3, 99]
        );
        assert_eq!(intcode.output().unwrap(), 1);

        //Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let mut intcode = Intcode::from("3,3,1107,-1,8,3,4,3,99");

        intcode.input(7);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 3, 1107, 1, 8, 3, 4, 3, 99]
        );
        assert_eq!(intcode.output().unwrap(), 1);

        intcode.reset();
        intcode.input(9);
        assert_eq!(
            intcode.run().unwrap().memory,
            vec![3, 3, 1107, 0, 8, 3, 4, 3, 99]
        );
        assert_eq!(intcode.output().unwrap(), 0);
    }
}

use std::{collections::HashMap, fmt::Display};

type Wire = String;

#[derive(Debug, Clone)]
enum Value {
    Wire(Wire),
    Specific(u16),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        value
            .parse::<u16>()
            .map_or(Value::Wire(value.to_string()), |n| Value::Specific(n))
    }
}

#[derive(Debug, Clone)]
enum Gate {
    Pass(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Wire, u16),
    RShift(Wire, u16),
    Not(Wire),
}

#[derive(Debug)]
struct Instruction {
    input: Gate,
    output: Wire,
}

const ERROR_MESSAGE: &str = "Invalid instruction";

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (input, output) = value.split_once(" -> ").expect(ERROR_MESSAGE);
        let mut input_parts = input.split_ascii_whitespace();

        let input = match input_parts.clone().count() {
            1 => {
                let value = input_parts.next().unwrap();

                Gate::Pass(Value::from(value))
            }
            2 => {
                input_parts.next();
                Gate::Not(input_parts.next().expect(ERROR_MESSAGE).to_string())
            }
            3 => {
                let x = input_parts.next().expect(ERROR_MESSAGE);
                let operator = input_parts.next().expect(ERROR_MESSAGE);
                let y = input_parts.next().expect(ERROR_MESSAGE);

                match operator {
                    "AND" => Gate::And(Value::from(x), Value::from(y)),
                    "OR" => Gate::Or(Value::from(x), Value::from(y)),
                    "LSHIFT" => Gate::LShift(x.to_string(), y.parse().expect(ERROR_MESSAGE)),
                    "RSHIFT" => Gate::RShift(x.to_string(), y.parse().expect(ERROR_MESSAGE)),
                    _ => panic!("{ERROR_MESSAGE}"),
                }
            }
            _ => panic!("{ERROR_MESSAGE}"),
        };
        let output = output.to_string();

        Self { input, output }
    }
}

struct Circuit(HashMap<Wire, Gate>);

impl Circuit {
    fn new(instructions: &[Instruction]) -> Self {
        let mut hash_map = HashMap::new();

        for instruction in instructions {
            hash_map.insert(instruction.output.clone(), instruction.input.clone());
        }

        Self(hash_map)
    }

    fn evaluate(&self, output: &Wire, cache: &mut HashMap<Wire, u16>) -> Option<u16> {
        if let Some(&result) = cache.get(output) {
            return Some(result);
        }

        let circuit = &self.0;
        let gate = circuit.get(output)?;

        let mut get_value = |value: &Value| -> Option<u16> {
            match value {
                Value::Wire(x) => self.evaluate(&x, cache),
                Value::Specific(v) => Some(*v),
            }
        };

        let result = match gate {
            Gate::Pass(value) => match value {
                Value::Wire(x) => self.evaluate(x, cache)?,
                Value::Specific(v) => *v,
            },
            Gate::And(x, y) => {
                let x = get_value(x)?;
                let y = get_value(y)?;

                x & y
            }
            Gate::Or(x, y) => {
                let x = get_value(x)?;
                let y = get_value(y)?;

                x | y
            }
            Gate::LShift(x, n) => {
                let x = self.evaluate(x, cache)?;

                x << n
            }
            Gate::RShift(x, n) => {
                let x = self.evaluate(x, cache)?;

                x >> n
            }
            Gate::Not(x) => {
                let x = self.evaluate(x, cache)?;

                !x
            }
        };

        cache.insert(output.clone(), result);

        Some(result)
    }

    fn override_wire(&mut self, wire: &Wire, value: u16) {
        self.0
            .insert(wire.clone(), Gate::Pass(Value::Specific(value)));
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u16 {
    let circuit = create_circuit(input);
    evaluate_circuit(&circuit, "a").unwrap_or(0)
}

fn solve_second_part(input: &str) -> u16 {
    let mut circuit = create_circuit(input);
    let a = evaluate_circuit(&circuit, "a").unwrap_or(0);

    circuit.override_wire(&"b".to_string(), a);
    evaluate_circuit(&circuit, "a").unwrap_or(0)
}

fn create_circuit(input: &str) -> Circuit {
    let instructions = input.lines().map(Instruction::from).collect::<Vec<_>>();
    Circuit::new(&instructions)
}

fn evaluate_circuit(circuit: &Circuit, output: &str) -> Option<u16> {
    let mut cache = HashMap::new();
    circuit.evaluate(&output.to_string(), &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";

    #[test]
    fn test_first_part() {
        let answer = 0;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 0;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_evaluate_circuit() {
        let circuit = create_circuit(INPUT);

        assert_eq!(evaluate_circuit(&circuit, "d"), Some(72));
        assert_eq!(evaluate_circuit(&circuit, "e"), Some(507));
        assert_eq!(evaluate_circuit(&circuit, "f"), Some(492));
        assert_eq!(evaluate_circuit(&circuit, "g"), Some(114));
        assert_eq!(evaluate_circuit(&circuit, "h"), Some(65412));
        assert_eq!(evaluate_circuit(&circuit, "i"), Some(65079));
        assert_eq!(evaluate_circuit(&circuit, "x"), Some(123));
        assert_eq!(evaluate_circuit(&circuit, "y"), Some(456));
    }

    check_answers!(16076, 2797);
}

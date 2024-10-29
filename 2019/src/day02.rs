use std::fmt::Display;

#[derive(Debug, Clone)]
struct Intcode {
    memory: Vec<u32>,
}

impl Intcode {
    fn run(&mut self) -> Option<&Self> {
        let mut i = 0;

        loop {
            let optcode = self.memory[i];
            if optcode == 99 {
                break;
            }

            let op1 = self.memory[self.memory[i + 1] as usize];
            let op2 = self.memory[self.memory[i + 2] as usize];
            let address = self.memory[i + 3] as usize;

            match optcode {
                1 => self.memory[address] = op1 + op2,
                2 => self.memory[address] = op1 * op2,
                _ => return None,
            }

            i += 4;
        }

        Some(self)
    }

    fn get(&self, address: usize) -> u32 {
        self.memory[address]
    }

    fn set(&mut self, address: usize, value: u32) {
        self.memory[address] = value
    }
}

impl From<&str> for Intcode {
    fn from(value: &str) -> Self {
        let memory = value
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Self { memory }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let mut intcode = Intcode::from(input);

    intcode.set(1, 12);
    intcode.set(2, 2);

    intcode.run();

    intcode.get(0)
}

fn solve_second_part(input: &str) -> u32 {
    let intcode = Intcode::from(input);

    let (noun, verb) = brute_force_solution(&intcode, 19690720);

    noun * 100 + verb
}

fn brute_force_solution(intcode: &Intcode, target: u32) -> (u32, u32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = intcode.clone();

            intcode.set(1, noun);
            intcode.set(2, verb);

            intcode.run();

            if intcode.get(0) == target {
                return (noun, verb);
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,12,2,0,99,0,0,0,0,0,0,0,67
";

    #[test]
    fn test_first_part() {
        let answer = 69;

        assert_eq!(answer, solve_first_part(INPUT))
    }

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
    }

    check_answers!(3058646, 8976);
}

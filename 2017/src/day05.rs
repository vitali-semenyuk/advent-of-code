use std::fmt::Display;

struct Cpu {
    jumps: Vec<i32>,
    ip: usize,
}

impl Cpu {
    fn new(jumps: Vec<i32>) -> Self {
        Self { jumps, ip: 0 }
    }

    fn run(&mut self, sophisticated_jumps: bool) -> usize {
        let mut ticks = 0;
        let max_address = self.jumps.len() as i32 - 1;

        loop {
            ticks += 1;

            let offset = self.jumps[self.ip];
            let new_ip = self.ip as i32 + offset;
            if new_ip < 0 || new_ip > max_address {
                break;
            }

            self.jumps[self.ip] += if sophisticated_jumps && offset >= 3 {
                -1
            } else {
                1
            };
            self.ip = new_ip as usize;
        }

        ticks
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let jumps = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut cpu = Cpu::new(jumps);

    cpu.run(false)
}

fn solve_second_part(input: &str) -> usize {
    let jumps = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut cpu = Cpu::new(jumps);

    cpu.run(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0
3
0
1
-3
";

    #[test]
    fn test_first_part() {
        let answer = 5;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 10;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(374269, 27720699);
}

use std::fmt::Display;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        if string == "noop" {
            Instruction::Noop
        } else {
            let (_, number) = string.split_once(" ").unwrap();
            let number = number.parse().unwrap();
            Instruction::AddX(number)
        }
    }
}

impl Instruction {
    fn process(&self) -> i32 {
        match self {
            Self::Noop => 0,
            Self::AddX(number) => *number,
        }
    }
}

#[derive(Debug)]
struct CPU {
    instructions: Vec<Instruction>,
    register: i32,
    ip: usize,
}

impl Iterator for CPU {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.ip / 2);
        instruction.and_then(|instr| {
            let res = self.register;

            self.ip += match instr {
                Instruction::Noop => 2,
                Instruction::AddX(_) => 1,
            };
            if self.ip % 2 == 0 {
                self.register += instr.process()
            }

            Some(res)
        })
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let instructions = input.lines().map(Instruction::from).collect();
    let mut cpu = CPU {
        instructions,
        register: 1,
        ip: 0,
    };

    let mut i = 0;
    [20, 60, 100, 140, 180, 220]
        .map(|n| {
            let r = cpu.nth(n - i - 1).unwrap();
            i = n;
            r * n as i32
        })
        .iter()
        .sum()
}

fn solve_second_part(input: &str) -> String {
    let instructions = input.lines().map(Instruction::from).collect();
    let cpu = CPU {
        instructions,
        register: 1,
        ip: 0,
    };

    let mut result = String::new();

    let width = 40;
    for (cycle, register) in cpu.enumerate() {
        let col = (cycle % width) as i32;
        let sym = if col >= register - 1 && col <= register + 1 {
            '#'
        } else {
            '.'
        };
        result.push(sym);
        if col == width as i32 - 1 {
            result.push('\n')
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_first_part() {
        let answer = 13140;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(14560, "####.#..#.###..#..#.####.###..#..#.####.
#....#.#..#..#.#..#.#....#..#.#..#....#.
###..##...#..#.####.###..#..#.#..#...#..
#....#.#..###..#..#.#....###..#..#..#...
#....#.#..#.#..#..#.#....#....#..#.#....
####.#..#.#..#.#..#.####.#.....##..####.
");
}

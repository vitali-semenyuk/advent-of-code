use std::fmt::Display;

#[derive(Debug)]
enum Instruction {
    TurnOn(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        match parts.next().unwrap() {
            "rect" => {
                let (a, b) = parts.next().unwrap().split_once("x").unwrap();
                Instruction::TurnOn(a.parse().unwrap(), b.parse().unwrap())
            }
            "rotate" => {
                let class = match parts.next().unwrap() {
                    "row" => Instruction::RotateRow,
                    "column" => Instruction::RotateColumn,
                    _ => panic!("Unexpected value"),
                };
                let a = parts.next().unwrap();
                let (_, a) = a.split_once("=").unwrap();
                let b = parts.nth(1).unwrap();
                class(a.parse().unwrap(), b.parse().unwrap())
            }
            _ => panic!("Unexpected value"),
        }
    }
}

struct Screen {
    area: Vec<Vec<bool>>,
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.area {
            for cell in row {
                let char = if *cell { "#" } else { "." };
                write!(f, "{char}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        let area = vec![vec![false; width]; height];
        Screen { area }
    }

    fn lit_count(&self) -> usize {
        self.area
            .iter()
            .map(|row| row.iter().filter(|c| **c).count())
            .sum()
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(instruction)
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            &Instruction::TurnOn(a, b) => {
                for x in 0..a {
                    for y in 0..b {
                        self.area[y][x] = true
                    }
                }
            }
            &Instruction::RotateRow(a, b) => self.area[a].rotate_right(b),
            &Instruction::RotateColumn(a, b) => {
                let mut column = self.area.iter().map(|row| row[a]).collect::<Vec<_>>();
                column.rotate_right(b);

                for (index, row) in self.area.iter_mut().enumerate() {
                    row[a] = column[index]
                }
            }
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    setup_screen(input).lit_count()
}

fn solve_second_part(input: &str) -> String {
    let screen = setup_screen(input);
    println!("{screen}");
    "RURUCEOEIL".to_string()
}

fn setup_screen(input: &str) -> Screen {
    let instructions = input.lines().map(Instruction::from).collect::<Vec<_>>();
    let mut screen = Screen::new(50, 6);

    screen.execute(&instructions);

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
";

    #[test]
    fn test_first_part() {
        let answer = 6;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "RURUCEOEIL";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(121, "RURUCEOEIL");
}

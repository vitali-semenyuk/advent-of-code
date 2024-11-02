use std::fmt::Display;

use crate::shared::intcode::{Intcode, RuntimeError};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("Unexpected value"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => 'X',
            Tile::Paddle => '_',
            Tile::Ball => 'o',
        };

        write!(f, "{}", ch)
    }
}

struct Game {
    screen: Vec<Vec<Tile>>,
    score: u32,
    paddle_x: usize,
    ball_x: usize,
}

impl Game {
    fn create(intcode: &mut Intcode) -> Self {
        intcode.set(0, 2);

        if let Err(err) = intcode.run() {
            match err {
                RuntimeError::MissingInput { ip: _ } => (),
                _ => panic!("{}", err),
            }
        }

        let screen = create_screen(&intcode.buffered_output());

        Self {
            screen,
            score: 0,
            paddle_x: 0,
            ball_x: 0,
        }
    }

    fn act(&mut self, intcode: &mut Intcode) {
        if self.ball_x > self.paddle_x {
            intcode.input(1);
        } else if self.ball_x < self.paddle_x {
            intcode.input(-1);
        } else {
            intcode.input(0);
        }

        if let Err(err) = intcode.run() {
            match err {
                RuntimeError::MissingInput { ip: _ } => (),
                _ => panic!("{}", err),
            }
        }
    }

    fn update_screen(&mut self, data: &Vec<i64>) {
        let cells = data.chunks(3).collect::<Vec<_>>();

        for cell in cells {
            if cell[0] == -1 {
                self.score = cell[2] as u32;
                continue;
            }

            let x = cell[0] as usize;
            let y = cell[1] as usize;
            let block = Tile::from(cell[2]);

            match block {
                Tile::Paddle => self.paddle_x = x,
                Tile::Ball => self.ball_x = x,
                _ => (),
            }

            self.screen[y][x] = block;
        }
    }

    fn print_screen(&self) {
        let result = self
            .screen
            .iter()
            .map(|row| row.iter().map(|t| t.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        println!("Score: {}\n{}", self.score, result);
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let mut intcode = Intcode::from(input);

    intcode.run().expect("Runtime error");

    intcode
        .buffered_output()
        .into_iter()
        .enumerate()
        .filter(|(i, tile)| (i + 1) % 3 == 0 && *tile == 2)
        .count()
}

fn solve_second_part(input: &str) -> u32 {
    let mut intcode = Intcode::from(input);

    let mut game = Game::create(&mut intcode);

    game.print_screen();

    loop {
        if intcode.is_halted() {
            break;
        }

        game.act(&mut intcode);
        game.update_screen(&intcode.buffered_output());

        //thread::sleep(Duration::from_millis(100));
        //print!("{}[2J", 27 as char);
        //game.print_screen();
    }

    game.score
}

fn create_screen(data: &Vec<i64>) -> Vec<Vec<Tile>> {
    let cells = data.chunks(3).filter(|ch| ch[0] != -1).collect::<Vec<_>>();

    let width = cells.iter().max_by_key(|ch| ch[0]).unwrap()[0] as usize + 1;
    let height = cells.iter().max_by_key(|ch| ch[1]).unwrap()[1] as usize + 1;

    let mut screen = vec![vec![Tile::Empty; width]; height];

    for cell in cells {
        let x = cell[0] as usize;
        let y = cell[1] as usize;
        let block = Tile::from(cell[2]);

        screen[y][x] = block;
    }

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
";

    #[ignore]
    #[test]
    fn test_first_part() {
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(355, 18371);
}

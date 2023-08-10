use std::fmt::Display;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unexpected value"),
        }
    }
}

#[derive(Debug)]
struct Solver {
    x: usize,
    y: usize,
    matrix: Vec<Vec<Option<char>>>,
}

impl Solver {
    fn new(x: usize, y: usize, matrix: Vec<Vec<Option<char>>>) -> Self {
        Self { x, y, matrix }
    }

    fn simple() -> Self {
        let matrix = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let matrix = matrix
            .into_iter()
            .map(|row| row.into_iter().map(Some).collect())
            .collect();

        Self::new(1, 1, matrix)
    }

    fn complex() -> Self {
        let matrix = vec![
            vec![None, None, Some('1'), None, None],
            vec![None, Some('2'), Some('3'), Some('4'), None],
            vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            vec![None, Some('A'), Some('B'), Some('C'), None],
            vec![None, None, Some('D'), None, None],
        ];

        Self::new(0, 2, matrix)
    }

    fn solve(&mut self, directions: &Vec<Vec<Direction>>) -> String {
        let mut result = Vec::new();

        for row in directions {
            for direction in row {
                self.consume(direction)
            }

            result.push(self.get_value_at(self.x, self.y).unwrap())
        }

        result.iter().collect()
    }

    fn consume(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => {
                if self.y > 0 && self.get_value_at(self.x, self.y - 1).is_some() {
                    self.y -= 1;
                }
            }
            Direction::Down => {
                if self.get_value_at(self.x, self.y + 1).is_some() {
                    self.y += 1;
                }
            }
            Direction::Left => {
                if self.x > 0 && self.get_value_at(self.x - 1, self.y).is_some() {
                    self.x -= 1;
                }
            }
            Direction::Right => {
                if self.get_value_at(self.x + 1, self.y).is_some() {
                    self.x += 1;
                }
            }
        }
    }

    fn get_value_at(&self, x: usize, y: usize) -> Option<char> {
        *self.matrix.get(y)?.get(x)?
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let mut solver = Solver::simple();
    solver.solve(&get_directions(input))
}

fn solve_second_part(input: &str) -> String {
    let mut solver = Solver::complex();
    solver.solve(&get_directions(input))
}

fn get_directions(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| line.chars().map(Direction::from).collect())
        .collect::<Vec<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ULL
RRDDD
LURDL
UUUUD
";

    #[test]
    fn test_first_part() {
        let answer = "1985";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "5DB3";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!("98575", "CD8D4");
}

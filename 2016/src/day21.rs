use std::fmt::Display;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => panic!("Invalida value"),
        }
    }
}

#[derive(Debug)]
enum Operation {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    Rotate(Direction, usize),
    PositionalRotate(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        match parts.next().unwrap() {
            "swap" => match parts.next().unwrap() {
                "position" => {
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.nth(2).unwrap().parse().unwrap();
                    Operation::SwapPositions(x, y)
                }
                "letter" => {
                    let x = parts.next().unwrap().chars().next().unwrap();
                    let y = parts.nth(2).unwrap().chars().next().unwrap();
                    Operation::SwapLetters(x, y)
                }
                _ => panic!("Unknown operation"),
            },
            "rotate" => {
                let direction = parts.next().unwrap();

                if direction == "based" {
                    let x = parts.nth(4).unwrap().chars().next().unwrap();
                    Operation::PositionalRotate(x)
                } else {
                    let x = parts.next().unwrap().parse().unwrap();
                    Operation::Rotate(Direction::from(direction), x)
                }
            }
            "reverse" => {
                let x = parts.nth(1).unwrap().parse().unwrap();
                let y = parts.nth(1).unwrap().parse().unwrap();
                Operation::Reverse(x, y)
            }
            "move" => {
                let x = parts.nth(1).unwrap().parse().unwrap();
                let y = parts.nth(2).unwrap().parse().unwrap();
                Operation::Move(x, y)
            }
            _ => panic!("Unknown operation"),
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> String {
    let operations = input.lines().map(Operation::from).collect::<Vec<_>>();

    scramble("abcdefgh", &operations)
}

fn solve_second_part(input: &str) -> String {
    let operations = input.lines().map(Operation::from).collect::<Vec<_>>();

    unscramble("fbgdceah", &operations)
}

fn scramble(password: &str, operations: &[Operation]) -> String {
    let mut result = password.chars().collect::<Vec<_>>();

    for operation in operations {
        match operation {
            Operation::SwapPositions(x, y) => result.swap(*x, *y),
            Operation::SwapLetters(x, y) => {
                let ix = result.iter().position(|n| n == x).unwrap();
                let iy = result.iter().position(|n| n == y).unwrap();
                result.swap(ix, iy)
            }
            Operation::Rotate(dir, x) => match dir {
                Direction::Left => result.rotate_left(*x),
                Direction::Right => result.rotate_right(*x),
            },
            Operation::PositionalRotate(x) => {
                let ix = result.iter().position(|n| n == x).unwrap();
                let mut shift = ix + 1;
                if ix >= 4 {
                    shift += 1;
                }
                shift %= result.len();
                result.rotate_right(shift)
            }
            Operation::Reverse(x, y) => {
                let target = &mut result[*x..=*y];
                target.reverse();
            }
            Operation::Move(x, y) => {
                let char = result.remove(*x);
                result.insert(*y, char)
            }
        }
    }

    result.iter().collect()
}

fn unscramble(hash: &str, operations: &[Operation]) -> String {
    let mut result = hash.chars().collect::<Vec<_>>();

    for operation in operations.iter().rev() {
        match operation {
            Operation::SwapPositions(x, y) => result.swap(*x, *y),
            Operation::SwapLetters(x, y) => {
                let ix = result.iter().position(|n| n == x).unwrap();
                let iy = result.iter().position(|n| n == y).unwrap();
                result.swap(ix, iy)
            }
            Operation::Rotate(dir, x) => match dir {
                Direction::Left => result.rotate_right(*x),
                Direction::Right => result.rotate_left(*x),
            },
            Operation::PositionalRotate(x) => {
                let ix = result.iter().position(|n| n == x).unwrap();
                let offset = [1, 1, 6, 2, 7, 3, 0, 4][ix];
                result.rotate_left(offset);
            }
            Operation::Reverse(x, y) => {
                let target = &mut result[*x..=*y];
                target.reverse();
            }
            Operation::Move(x, y) => {
                let char = result.remove(*y);
                result.insert(*x, char)
            }
        }
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d
";

    #[test]
    fn test_first_part() {
        let answer = "fbdecgha";

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = "efghdabc";

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_scramble() {
        let operations = INPUT.lines().map(Operation::from).collect::<Vec<_>>();

        assert_eq!("decab", scramble("abcde", &operations))
    }

    #[test]
    fn test_unscramble() {
        let operations = INPUT.lines().map(Operation::from).collect::<Vec<_>>();

        assert_eq!("abcde", unscramble("decab", &operations));
        assert_eq!(
            "abcdefgh",
            unscramble(&scramble("abcdefgh", &operations), &operations)
        );
    }

    check_answers!("dbfgaehc", "aghfcdeb");
}

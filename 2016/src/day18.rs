use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn is_trap(&self) -> bool {
        *self == Self::Trap
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Safe,
            '^' => Self::Trap,
            _ => panic!("Invalid value"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            Tile::Safe => '.',
            Tile::Trap => '^',
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
struct Floor {
    tiles: Vec<Vec<Tile>>,
}

impl Floor {
    fn new(seed: &str) -> Self {
        let first_row = seed.chars().map(Tile::from).collect();
        let tiles = vec![first_row];
        Self { tiles }
    }

    fn fill_row(&mut self) {
        let current_row = self.tiles.last().unwrap();
        let mut next_row = Vec::new();

        for i in 0..current_row.len() {
            let left = if i > 0 {
                current_row.get(i - 1).unwrap()
            } else {
                &Tile::Safe
            }
            .is_trap();
            let right = current_row.get(i + 1).unwrap_or(&Tile::Safe).is_trap();

            let is_trap = !left && right || left && !right;
            let tile = if is_trap { Tile::Trap } else { Tile::Safe };

            next_row.push(tile)
        }

        self.tiles.push(next_row)
    }

    fn fill_rows(&mut self, n: u32) {
        for _ in 0..n {
            self.fill_row()
        }
    }

    fn get_safe_tiles_count(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|tile| **tile == Tile::Safe).count())
            .sum()
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    let seed = input.strip_suffix('\n').unwrap_or(input);
    let mut floor = Floor::new(seed);

    floor.fill_rows(40 - 1);

    floor.get_safe_tiles_count()
}

fn solve_second_part(input: &str) -> usize {
    let seed = input.strip_suffix('\n').unwrap_or(input);
    let mut floor = Floor::new(seed);

    floor.fill_rows(400000 - 1);

    floor.get_safe_tiles_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".^^.^.^^^^
";

    #[test]
    fn test_first_part() {
        let answer = 185;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 1935478;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_fill_rows() {
        let mut floor = Floor::new("..^^.");
        floor.fill_rows(2);
        let answer = "..^^.
.^^^^
^^..^
";
        assert_eq!(answer, floor.to_string());

        let mut floor = Floor::new(".^^.^.^^^^");
        floor.fill_rows(9);
        let answer = ".^^.^.^^^^
^^^...^..^
^.^^.^.^^.
..^^...^^^
.^^^^.^^.^
^^..^.^^..
^^^^..^^^.
^..^^^^.^^
.^^^..^.^^
^^.^^^..^^
";
        assert_eq!(answer, floor.to_string());
    }

    #[test]
    fn test_get_safe_tiles_count() {
        let mut floor = Floor::new(".^^.^.^^^^");
        floor.fill_rows(9);
        assert_eq!(38, floor.get_safe_tiles_count())
    }

    check_answers!(1956, 19995121);
}

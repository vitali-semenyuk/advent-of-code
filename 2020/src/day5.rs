pub fn solve(input: &str) -> (i64, i64) {
    (solve_first_part(input), solve_second_part(input))
}

fn solve_first_part(input: &str) -> i64 {
    input.lines().map(|seat| parse_seat(seat)).max().unwrap() as i64
}

fn solve_second_part(input: &str) -> i64 {
    let seats: Vec<_> = input.lines().map(|seat| parse_seat(seat)).collect();
    (0..128 * 8)
        .filter(|seat| {
            seat > &0
                && !seats.contains(seat)
                && seats.contains(&(seat - 1))
                && seats.contains(&(seat + 1))
        })
        .next()
        .unwrap() as i64
}

fn parse_seat(seat: &str) -> u16 {
    let mut row_low = 0;
    let mut row_high = 127;
    let mut col_low = 0;
    let mut col_high = 7;

    for dir in seat.chars() {
        let row_mid = (row_high - row_low) / 2 + 1;
        let col_mid = (col_high - col_low) / 2 + 1;

        match dir {
            'F' => {
                row_high -= row_mid;
            }
            'B' => {
                row_low += row_mid;
            }
            'L' => {
                col_high -= col_mid;
            }
            'R' => {
                col_low += col_mid;
            }
            _ => panic!("Unexpected char"),
        };
    }

    row_low * 8 + col_low
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_first_part() {
        let answer = 820;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_parse_seat() {
        assert_eq!(357, parse_seat("FBFBBFFRLR"));
        assert_eq!(567, parse_seat("BFFFBBFRRR"));
        assert_eq!(119, parse_seat("FFFBBBFRRR"));
        assert_eq!(820, parse_seat("BBFFBBFRLL"));
    }

    #[test]
    fn first_part_solution() {
        let input = fs::read_to_string("./tasks/day5.txt").unwrap();
        let answer = 998;

        assert_eq!(answer, solve_first_part(&input));
    }

    #[test]
    fn second_part_solution() {
        let input = fs::read_to_string("./tasks/day5.txt").unwrap();
        let answer = 676;

        assert_eq!(answer, solve_second_part(&input));
    }
}

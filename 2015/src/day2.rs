use std::fmt::Display;

#[derive(Debug)]
struct GiftBox {
    length: u32,
    width: u32,
    height: u32,
}

impl GiftBox {
    fn wrapping_area(&self) -> u32 {
        let side_ares = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let area: u32 = side_ares.iter().map(|side| side * 2).sum();
        let min_side_area = side_ares.iter().min().unwrap();

        area + min_side_area
    }

    fn ribbon_length(&self) -> u32 {
        let side_perimeters = [
            self.length + self.width,
            self.width + self.height,
            self.height + self.length,
        ]
        .map(|v| v * 2);
        let min_perimeter = side_perimeters.iter().min().unwrap();
        let volume = self.length * self.width * self.height;

        min_perimeter + volume
    }
}

impl From<&str> for GiftBox {
    fn from(value: &str) -> Self {
        let mut splitted = value.splitn(3, 'x');
        let length: u32 = splitted.next().unwrap().parse().unwrap();
        let width: u32 = splitted.next().unwrap().parse().unwrap();
        let height: u32 = splitted.next().unwrap().parse().unwrap();

        Self {
            length,
            width,
            height,
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    input
        .lines()
        .map(GiftBox::from)
        .map(|gift| gift.wrapping_area())
        .sum()
}

fn solve_second_part(input: &str) -> u32 {
     input
        .lines()
        .map(GiftBox::from)
        .map(|gift| gift.ribbon_length())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(58, solve_first_part("2x3x4"));
        assert_eq!(43, solve_first_part("1x1x10"));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(34, solve_second_part("2x3x4"));
        assert_eq!(14, solve_second_part("1x1x10"));
    }

    check_answers!(1606483, 3842356);
}

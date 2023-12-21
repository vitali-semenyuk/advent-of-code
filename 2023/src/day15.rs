use std::fmt::Display;

#[derive(Clone, Debug)]
struct Lens {
    focal_length: u8,
    label: String,
}

#[derive(Debug)]
enum Instruction {
    Add(Lens),
    Remove(String),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some((label, focal_length)) = value.split_once('=') {
            Self::Add(Lens {
                focal_length: focal_length.parse().unwrap(),
                label: label.to_string(),
            })
        } else {
            Self::Remove(value.strip_suffix('-').unwrap().to_string())
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
        .trim_end()
        .split(',')
        .map(|string| hash(string) as u32)
        .sum()
}

fn solve_second_part(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .map(Instruction::from)
        .fold(
            [(); 256].map(|_| Vec::new()),
            |mut acc: [Vec<Lens>; 256], instruction| {
                match instruction {
                    Instruction::Add(lens) => {
                        let index = hash(&lens.label) as usize;
                        let current_box = &mut acc[index];
                        if let Some(i) = current_box.iter().position(|l| l.label == lens.label) {
                            current_box[i].focal_length = lens.focal_length
                        } else {
                            current_box.push(lens.clone())
                        }
                    }
                    Instruction::Remove(label) => {
                        let index = hash(&label) as usize;
                        let current_box = &mut acc[index];
                        if let Some(i) = current_box.iter().position(|l| l.label == label) {
                            current_box.remove(i);
                        }
                    }
                }

                acc
            },
        )
        .into_iter()
        .enumerate()
        .flat_map(|(index, current_box)| {
            current_box
                .into_iter()
                .enumerate()
                .map(move |(i, l)| (index + 1) * (i + 1) * l.focal_length as usize)
        })
        .sum()
}

fn hash(string: &str) -> u8 {
    let mut value = 0;

    for char in string.bytes() {
        value = ((value as u16 + char as u16) * 17) as u8;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

    #[test]
    fn test_first_part() {
        let answer = 1320;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 145;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    check_answers!(516469, 221627);
}

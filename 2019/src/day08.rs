use std::{collections::HashMap, fmt::Display};

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const IMAGE_SIZE: usize = IMAGE_WIDTH * IMAGE_HEIGHT;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let layer = input
        .trim()
        .as_bytes()
        .chunks(IMAGE_SIZE)
        .map(|chunk| {
            let mut counts = HashMap::new();
            for digit in chunk {
                *counts.entry(*digit as char).or_insert(0) += 1;
            }
            counts
        })
        .min_by_key(|counter| *counter.get(&'0').unwrap_or(&0))
        .unwrap();

    layer[&'1'] * layer[&'2']
}

fn solve_second_part(input: &str) -> String {
    let data = input.trim().chars().collect::<Vec<_>>();
    let layers = data.len() / IMAGE_SIZE;

    let decoded = (0..IMAGE_HEIGHT)
        .map(|y| {
            (0..IMAGE_WIDTH)
                .map(|x| {
                    let i = y * IMAGE_WIDTH + x;

                    for layer in 0..layers {
                        let offset = IMAGE_SIZE * layer + i;
                        match data[offset] {
                            '0' => return '0',
                            '1' => return ' ',
                            '2' => continue,
                            _ => panic!("Unexpected digit"),
                        };
                    }

                    unreachable!("Corrupt image");
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", decoded);

    "PHPEU".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    check_answers!(1452, "PHPEU");
}

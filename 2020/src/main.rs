use std::{env, fs};

#[macro_use]
mod macros;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

const DAYS: [fn(&str) -> (i64, i64); 5] = [
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
];

fn main() {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Please provide a day");
    let day: u8 = day.parse().expect("Day should be a number");

    let input = fs::read_to_string(format!("./tasks/day{day}.txt")).expect("No input file");
    let solver_fn = DAYS.get((day - 1) as usize).unwrap();

    let (first_answer, second_answer) = solver_fn(&input);

    println!("First task: {}", first_answer);
    println!("Second task: {}", second_answer);
}

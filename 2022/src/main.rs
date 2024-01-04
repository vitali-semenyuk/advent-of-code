use std::{env, fmt::Display, fs};

#[macro_use]
mod macros;

mod day1;
mod day10;
mod day11;
mod day12;
mod day14;
mod day15;
mod day16;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;
mod day9;

type SolverFunction = fn(&str) -> (Box<dyn Display>, Box<dyn Display>);

const DAYS: [SolverFunction; 18] = [
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day1::solve,
    day8::solve,
    day9::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day1::solve,
    day14::solve,
    day15::solve,
    day16::solve,
    day1::solve,
    day18::solve,
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

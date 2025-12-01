use std::{env, fmt::Display, fs};

#[macro_use]
mod macros;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

type SolverFunction = fn(&str) -> (Box<dyn Display>, Box<dyn Display>);

const DAYS: [SolverFunction; 12] = [
    day01::solve,
    day02::solve,
    day03::solve,
    day04::solve,
    day05::solve,
    day06::solve,
    day07::solve,
    day08::solve,
    day09::solve,
    day10::solve,
    day11::solve,
    day12::solve,
];

fn main() {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Please provide a day");
    let day: u8 = day.parse().expect("Day should be a number");

    let input =
        fs::read_to_string(format!("../inputs/2025/day{day:0>2}.txt")).expect("No input file");
    let solver_fn = DAYS.get((day - 1) as usize).expect("No solver function");

    let (first_answer, second_answer) = solver_fn(&input);

    println!("First task: {}", first_answer);
    println!("Second task: {}", second_answer);
}

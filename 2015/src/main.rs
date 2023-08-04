use std::{env, fmt::Display, fs};

#[macro_use]
mod macros;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const DAYS: [fn(&str) -> (Box<dyn Display>, Box<dyn Display>); 25] = [
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
    day9::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
    day14::solve,
    day15::solve,
    day16::solve,
    day17::solve,
    day18::solve,
    day19::solve,
    day20::solve,
    day21::solve,
    day22::solve,
    day23::solve,
    day24::solve,
    day25::solve,
];

fn main() {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Please provide a day");
    let day: u8 = day.parse().expect("Day should be a number");

    let input = fs::read_to_string(format!("./tasks/day{day}.txt")).expect("No input file");
    let solver_fn = DAYS.get((day - 1) as usize).expect("No solver function");

    let (first_answer, second_answer) = solver_fn(&input);

    println!("First task: {}", first_answer);
    println!("Second task: {}", second_answer);
}

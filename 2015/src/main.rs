use std::{env, fmt::Display, fs};

#[macro_use]
mod macros;

mod day1;

const DAYS: [fn(&str) -> (Box<dyn Display>, Box<dyn Display>); 1] = [day1::solve];

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

use std::{env, fs, io::Error};

mod day1;
mod day2;
mod day3;

const DAYS: [fn(&str) -> (i64, i64); 3] = [day1::solve, day2::solve, day3::solve];

fn main() -> Result<(), Error> {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Please provide a day");
    let day: u8 = day.parse().expect("Day should be a number");

    let input = fs::read_to_string(format!("./tasks/day{day}.txt"))?;
    let solver_fn = DAYS.get((day - 1) as usize).unwrap();

    let (first_answer, second_answer) = solver_fn(&input);

    println!("First task: {}", first_answer);
    println!("Second task: {}", second_answer);

    Ok(())
}

use std::{fs, io::Error};

mod day1;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./tasks/day1.txt")?;
    let first_answer = day1::solve_first_part(&input);
    let second_answer = day1::solve_second_part(&input);

    println!("First task: {}", first_answer);
    println!("Second task: {}", second_answer);

    Ok(())
}

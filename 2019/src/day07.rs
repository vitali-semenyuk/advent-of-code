use std::fmt::Display;

use crate::shared::intcode::{Intcode, RuntimeError};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i64 {
    generate_combinations(&vec![0, 1, 2, 3, 4], 5)
        .into_iter()
        .map(|phase_settings| run_program(input, &phase_settings))
        .max()
        .unwrap()
}

fn solve_second_part(input: &str) -> i64 {
    generate_combinations(&vec![5, 6, 7, 8, 9], 5)
        .into_iter()
        .map(|phase_settings| run_loop(input, &phase_settings))
        .max()
        .unwrap()
}

fn run_program(code: &str, phase_settings: &[i64]) -> i64 {
    let mut amps = (0..5).map(|_| Intcode::from(code)).collect::<Vec<_>>();
    let mut signal = 0;

    for (i, amp) in amps.iter_mut().enumerate() {
        amp.input(phase_settings[i]);
        amp.input(signal);

        if let Err(err) = amp.run() {
            panic!("Error! {err}")
        }

        signal = amp.output().expect("No output");
    }

    signal
}

fn run_loop(code: &str, phase_settings: &[i64]) -> i64 {
    let mut amps = (0..5).map(|_| Intcode::from(code)).collect::<Vec<_>>();

    for (i, amp) in amps.iter_mut().enumerate() {
        amp.input(phase_settings[i]);
    }

    let mut signal = 0;

    loop {
        let mut halted = 0;

        for amp in amps.iter_mut() {
            if amp.is_halted() {
                halted += 1;
                continue;
            }

            amp.input(signal);

            if let Err(err) = amp.run() {
                match err {
                    RuntimeError::MissingInput { ip: _ } => (),
                    _ => panic!("{}", err),
                }
            }

            signal = amp.output().expect("No output");
        }

        if halted == 5 {
            break;
        }
    }

    signal
}

fn generate_combinations(digits: &Vec<i64>, length: usize) -> Vec<Vec<i64>> {
    generate_combinations_recursive(digits, length, &vec![vec![]])
}

fn generate_combinations_recursive(
    digits: &Vec<i64>,
    length: usize,
    generated: &Vec<Vec<i64>>,
) -> Vec<Vec<i64>> {
    if length == 0 {
        return generated.clone();
    }

    let mut result = Vec::new();

    for set in generated {
        for digit in digits {
            if set.contains(digit) {
                continue;
            }

            let mut new_set = set.clone();
            new_set.push(*digit);
            result.push(new_set);
        }
    }

    generate_combinations_recursive(digits, length - 1, &result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(
            solve_first_part("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );

        assert_eq!(
            solve_first_part(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            ),
            54321
        );

        assert_eq!(
            solve_first_part("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
            65210
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(
            solve_second_part("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            139629729
        );

        assert_eq!(
            solve_second_part("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            18216
        );
    }

    check_answers!(262086, 5371621);
}

use std::fmt::Display;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    get_distance(input.trim_end().parse().unwrap())
}

fn solve_second_part(_input: &str) -> i32 {
    // let first_ring = [
    //     vec![1],
    //     vec![1, 2],
    //     vec![1, 2, 3],
    //     vec![1, 4],
    //     vec![1, 4, 5],
    //     vec![1, 6],
    //     vec![1, 2, 6, 7],
    //     vec![1, 2, 8],
    // ];

    // // Fill first ring
    // let mut values = vec![1];
    // for indicies in first_ring {
    //     let value = indicies.into_iter().map(|j| values[j - 1]).sum();

    //     values.push(value);
    // }

    // let mut inner = 2;
    // let mut previos = 9;
    // let mut count = 8;

    // for ring in 2..3 {
    //     let mut period = count / 4;
    //     let mut half_period = period / 2;

    //     for i in 0..count {
    //         let indicies = vec![previos];
    //         let value = indicies.into_iter().map(|j| values[j - 1]).sum();

    //         values.push(value);

    //         inner += 1;
    //         previos += 1;
    //     }

    //     count += 8;
    // }

    // dbg!(values);

    0
}

// fn _solve_second_part(_input: &str) -> i32 {
//     let lookups = [
//         vec![1],
//         vec![1, 2],
//         vec![1, 2, 3],
//         vec![1, 4],
//         vec![1, 4, 5],
//         vec![1, 6],
//         vec![1, 2, 6, 7],
//         vec![1, 2, 8],
//         vec![2, 9],
//         vec![2, 3, 9, 10],
//         vec![2, 3, 11],
//         vec![3, 12],
//         vec![3, 4, 12, 13],
//         vec![3, 4, 5, 14],
//         vec![4, 5, 15],
//         vec![5, 16],
//         vec![5, 6, 16, 17],
//         vec![5, 6, 7, 18],
//         vec![6, 7, 19],
//         vec![7, 20],
//         vec![7, 8, 20, 21],
//         vec![7, 8, 9, 22],
//         // [8, 9, 23]
//         // [9, 10, 24]
//     ];

//     let mut values = vec![1];

//     for i in 0..22 {
//         let value = lookups[i].iter().map(|j| values[j - 1]).sum::<u32>();

//         values.push(value);
//     }

//     dbg!(values);

//     0
// }

fn get_distance(n: u32) -> u32 {
    if n < 2 {
        return 0;
    }

    let sqrt = (n as f32).sqrt();
    let d = sqrt.ceil() as u32;
    let d = if d % 2 == 0 { d + 1 } else { d };
    let r = d / 2;

    let circle_min = (d - 2).pow(2) + 1;
    let circle_max = d.pow(2);

    let count = circle_max - circle_min + 1;
    let period = count / 4;
    let half_period = period / 2;
    let ord = circle_max - n;

    let tmp1 = ord % period;
    let tmp2 = ord % half_period;

    if tmp1 > tmp2 {
        r + tmp2
    } else {
        d - 1 - tmp1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1024
";

    #[test]
    fn test_first_part() {
        let answer = 31;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore = "work in progress"]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_get_distance() {
        assert_eq!(get_distance(1), 0);
        assert_eq!(get_distance(12), 3);
        assert_eq!(get_distance(23), 2);
        assert_eq!(get_distance(1024), 31);
    }

    // check_answers!(475, 42);
}

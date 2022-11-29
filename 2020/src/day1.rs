pub fn solve_first_part(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut result = 0;

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                result = numbers[i] * numbers[j];
                break;
            }
        }
    }

    result
}

pub fn solve_second_part(input: &str) -> i32 {
    let numbers: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut result = 0;

    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    result = numbers[i] * numbers[j] * numbers[k];
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "1721
979
366
299
675
1456";
        let answer = 514579;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part() {
        let input = "1721
979
366
299
675
1456";
        let answer = 241861950;

        assert_eq!(answer, solve_second_part(input))
    }
}

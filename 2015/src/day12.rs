use std::fmt::Display;

use serde_json::Value;

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i64 {
    let json: Value = serde_json::from_str(input).unwrap();
    sum_all_numbers(&json)
}

fn solve_second_part(input: &str) -> i64 {
    let json: Value = serde_json::from_str(input).unwrap();
    sum_numbers(&json)
}

fn sum_all_numbers(json: &Value) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(array) => array.iter().map(sum_all_numbers).sum(),
        Value::Object(object) => object.values().map(sum_all_numbers).sum(),
        _ => 0,
    }
}

fn sum_numbers(json: &Value) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(array) => array.iter().map(sum_numbers).sum(),
        Value::Object(object) => {
            if object.values().any(|value| value == "red") {
                return 0;
            }

            object.values().map(sum_numbers).sum()
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        assert_eq!(6, solve_first_part("[1,2,3]"));
        assert_eq!(6, solve_first_part(r#"{"a":2,"b":4}"#));
        assert_eq!(3, solve_first_part("[[[3]]]"));
        assert_eq!(3, solve_first_part(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(0, solve_first_part(r#"{"a":[-1,1]}"#));
        assert_eq!(0, solve_first_part(r#"[-1,{"a":1}]"#));
        assert_eq!(0, solve_first_part("[]"));
        assert_eq!(0, solve_first_part("{}"));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(6, solve_second_part("[1,2,3]"));
        assert_eq!(4, solve_second_part(r#"[1,{"c":"red","b":2},3]"#));
        assert_eq!(0, solve_second_part(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
        assert_eq!(6, solve_second_part(r#"[1,"red",5]"#));
    }

    check_answers!(111754, 65402);
}

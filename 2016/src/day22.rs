use std::fmt::Display;

#[derive(Debug, PartialEq)]
struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    available: u32,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        let mut name = parts.next().unwrap().split('-');
        let x = name
            .nth(1)
            .unwrap()
            .strip_prefix('x')
            .unwrap()
            .parse()
            .unwrap();
        let y = name
            .next()
            .unwrap()
            .strip_prefix('y')
            .unwrap()
            .parse()
            .unwrap();
        let size = parts
            .next()
            .unwrap()
            .strip_suffix('T')
            .unwrap()
            .parse()
            .unwrap();
        let used = parts
            .next()
            .unwrap()
            .strip_suffix('T')
            .unwrap()
            .parse()
            .unwrap();
        let available = parts
            .next()
            .unwrap()
            .strip_suffix('T')
            .unwrap()
            .parse()
            .unwrap();

        Self {
            x,
            y,
            size,
            used,
            available,
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let nodes = input.lines().skip(2).map(Node::from).collect::<Vec<_>>();

    let mut counter = 0;
    for a in &nodes {
        for b in &nodes {
            if a != b && a.used > 0 && b.available >= a.used {
                counter += 1
            }
        }
    }

    counter
}

fn solve_second_part(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%
";

    #[test]
    fn test_first_part() {
        let answer = 7;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(934, 42);
}

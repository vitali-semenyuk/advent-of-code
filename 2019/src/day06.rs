use std::{collections::HashMap, fmt::Display};

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (center, sattelite) = line.split_once(')').unwrap();
        map.insert(sattelite, center);
    }

    map.keys().map(|planet| count_orbits(&map, planet)).sum()
}

fn solve_second_part(input: &str) -> usize {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (center, sattelite) = line.split_once(')').unwrap();
        map.insert(sattelite, center);
    }

    let my_chain = get_chain(&map, "YOU");
    let santa_chain = get_chain(&map, "SAN");

    let mut d = 0;
    for i in 1..1_000_000 {
        if my_chain[i] == santa_chain[i] {
            d += 1;
        } else {
            break;
        }
    }

    my_chain.len() + santa_chain.len() - d * 2 - 4
}

fn count_orbits(map: &HashMap<&str, &str>, planet: &str) -> u32 {
    map.get(planet)
        .map(|p| count_orbits(map, p) + 1)
        .unwrap_or(0)
}

fn get_chain<'a>(map: &'a HashMap<&str, &str>, planet: &'a str) -> Vec<&'a str> {
    map.get(planet)
        .map(|p| {
            let mut chain = get_chain(map, p);
            chain.push(planet);
            chain
        })
        .unwrap_or(vec![planet])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
        let answer = 42;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
        let answer = 4;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(154386, 346);
}

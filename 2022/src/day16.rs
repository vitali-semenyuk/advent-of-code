use std::{collections::{HashMap, hash_map::DefaultHasher}, fmt::Display};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

impl From<&str> for Valve {
    fn from(string: &str) -> Self {
        let (valve, tunnels) = string.split_once(";").unwrap();
        let mut parts = valve.split_whitespace();
        parts.next();
        let name = parts.next().unwrap().to_string();
        let rate = parts.skip(2).next().unwrap();
        let rate = rate.split("=").skip(1).next().unwrap().parse().unwrap();
        let tunnels: Vec<_> = tunnels
            .split_whitespace()
            .skip(4)
            .map(|s| s.trim_matches(',').to_string())
            .collect();

        Valve {
            name,
            rate,
            tunnels,
        }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    // let valves: HashMap<_, _> = input
    //     .lines()
    //     .map(|l| {
    //         let valve = Valve::from(l);
    //         (valve.name.clone(), valve)
    //     })
    //     .collect();

    // dbg!(&valves);

    // dfs(&valves, valves.get("AA").unwrap(), 30, 0, Vec::new())
    0
}

fn solve_second_part(input: &str) -> i32 {
    42
}

// fn dfs(
//     graph: &HashMap<String, Valve>,
//     v: &Valve,
//     depth: u32,
//     rate: u32,
//     enabled: Vec<String>,
// ) -> u32 {
//     let cache = HashMap::new();

//     let inner = |graph, v: &Valve, depth, rate, enabled: Vec<String>| {
//         // let key =
//         // if depth == 0 {
//         //     return 0;
//         // }

//         let mut results: Vec<u32> = v
//             .tunnels
//             .iter()
//             .map(|t| {
//                 dfs(
//                     graph,
//                     graph.get(t).unwrap(),
//                     depth - 1,
//                     rate,
//                     enabled.clone(),
//                 ) + rate
//             })
//             .collect();
//         if enabled.contains(&v.name) && v.rate > 0 {
//             let mut enabled = enabled.clone();
//             enabled.push(v.name.clone());
//             results.push(dfs(graph, v, depth - 1, rate + v.rate, enabled))
//         }

//         *results.iter().max().unwrap()
//     };

//     inner(graph, v, depth, rate, enabled)
// }

// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[ignore]
    #[test]
    fn test_first_part() {
        let answer = 1651;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[ignore]
    #[test]
    fn test_second_part() {
        let answer = 1707;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(42, 42);
}

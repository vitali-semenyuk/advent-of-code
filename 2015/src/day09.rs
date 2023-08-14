use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Route {
    origin: String,
    destination: String,
    distance: u32,
}

impl From<&str> for Route {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();

        let origin = parts.next().unwrap().to_string();
        parts.next();
        let destination = parts.next().unwrap().to_string();
        parts.next();
        let distance = parts.next().unwrap().to_string().parse().unwrap();

        Self {
            origin,
            destination,
            distance,
        }
    }
}

#[derive(Debug)]
struct Graph(HashMap<String, Vec<(String, u32)>>);

impl Graph {
    fn new(routes: &[Route]) -> Self {
        let mut hash_map = HashMap::new();

        for route in routes {
            if hash_map.contains_key(&route.destination) {
                let destinations: &mut Vec<_> = hash_map.get_mut(&route.destination).unwrap();
                destinations.push((route.origin.clone(), route.distance))
            } else {
                hash_map.insert(
                    route.destination.clone(),
                    vec![(route.origin.clone(), route.distance)],
                );
            }

            if hash_map.contains_key(&route.origin) {
                let destinations: &mut Vec<_> = hash_map.get_mut(&route.origin).unwrap();
                destinations.push((route.destination.clone(), route.distance))
            } else {
                hash_map.insert(
                    route.origin.clone(),
                    vec![(route.destination.clone(), route.distance)],
                );
            }
        }

        Self(hash_map)
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let routes = input.lines().map(Route::from).collect::<Vec<_>>();
    let graph = Graph::new(&routes);

    graph
        .0
        .keys()
        .flat_map(|start| traverse(&graph, start, &Vec::new()))
        .min()
        .unwrap()
}

fn solve_second_part(input: &str) -> u32 {
    let routes = input.lines().map(Route::from).collect::<Vec<_>>();
    let graph = Graph::new(&routes);

    graph
        .0
        .keys()
        .flat_map(|start| traverse(&graph, start, &Vec::new()))
        .max()
        .unwrap()
}

fn traverse(graph: &Graph, node: &String, visited: &[String]) -> Vec<u32> {
    let destinations = graph.0.get(node).expect("Node not found");
    let mut distances = vec![];

    for (destination, distance) in destinations {
        if visited.contains(destination) {
            continue;
        }

        let mut visited = visited.to_owned();
        visited.push(node.clone());

        let mut ds = traverse(graph, destination, &visited)
            .iter()
            .map(|d| d + distance)
            .collect::<Vec<_>>();

        distances.append(&mut ds);
    }

    if distances.is_empty() {
        vec![0]
    } else {
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";

    #[test]
    fn test_first_part() {
        let answer = 605;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 982;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(117, 909);
}

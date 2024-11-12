use std::{
    cmp::{max, min, Ordering},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Planet {
    position: Vector3,
    velocity: Vector3,
}

impl Planet {
    fn get_kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn get_potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }
}

impl From<&str> for Planet {
    fn from(value: &str) -> Self {
        let value = value.trim_matches(['<', '>']);
        let mut parts = value.split(", ");
        let (_, x) = parts.next().unwrap().split_once('=').unwrap();
        let (_, y) = parts.next().unwrap().split_once('=').unwrap();
        let (_, z) = parts.next().unwrap().split_once('=').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        let z = z.parse().unwrap();

        let position = Vector3 { x, y, z };
        let velocity = Vector3 { x: 0, y: 0, z: 0 };

        Planet { position, velocity }
    }
}

impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<{}, {}, {}> (<{}, {}, {}>)",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z,
        )
    }
}

struct Simulation {
    planets: Vec<Planet>,
    states: Vec<Vec<Vector3>>,
}

impl Simulation {
    fn new(planets: Vec<Planet>) -> Self {
        let states = planets.iter().map(|p| vec![p.position]).collect();
        Self { planets, states }
    }

    fn step(&mut self) {
        let pairs = get_pairs(self.planets.len());

        for (i, j) in &pairs {
            let (a, b) = get_pair_mut(&mut self.planets, *i, *j).unwrap();
            apply_gravity(a, b);
        }

        for planet in &mut self.planets {
            update_position(planet);
        }

        for (i, planet) in self.planets.iter().enumerate() {
            self.states[i].push(planet.position);
        }
    }

    fn get_state(&mut self, planet: usize, n: usize) -> Vector3 {
        let missing_steps = n as i32 - self.states[0].len() as i32 + 1;

        if missing_steps > 0 {
            for _ in 0..missing_steps {
                self.step();
            }
        }

        self.states[planet][n]
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let mut planets = input.lines().map(Planet::from).collect::<Vec<_>>();

    get_energy(&mut planets, 1000)
}

fn solve_second_part(input: &str) -> usize {
    let planets = input.lines().map(Planet::from).collect::<Vec<_>>();

    let mut simulation = Simulation::new(planets.clone());

    planets.into_iter().enumerate().fold(1, |period, (i, _)| {
        lcm(period, get_period_for_planet(&mut simulation, i))
    })
}

fn get_energy(planets: &mut [Planet], iterations: usize) -> i32 {
    let pairs = get_pairs(planets.len());

    for _ in 0..iterations {
        for (i, j) in &pairs {
            let (a, b) = get_pair_mut(planets, *i, *j).unwrap();
            apply_gravity(a, b);
        }

        for planet in &mut *planets {
            update_position(planet);
        }
    }

    planets
        .iter()
        .map(|p| p.get_kinetic_energy() * p.get_potential_energy())
        .sum()
}

fn get_period_for_planet(simulation: &mut Simulation, planet: usize) -> usize {
    let period_x = get_period(|n| simulation.get_state(planet, n).x);
    let period_y = get_period(|n| simulation.get_state(planet, n).y);
    let period_z = get_period(|n| simulation.get_state(planet, n).z);
    let period = lcm(period_x, period_y);
    lcm(period, period_z)
}

fn get_period<F>(mut f: F) -> usize
where
    F: FnMut(usize) -> i32,
{
    for period in 2..1_000_000 {
        let mut is_ok = true;
        for i in 0..period {
            if f(i) != f(i + period) {
                is_ok = false;
                break;
            }
        }

        if is_ok {
            return period;
        }
    }

    panic!("Unable to find period")
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn apply_gravity(a: &mut Planet, b: &mut Planet) {
    (a.velocity.x, b.velocity.x) =
        apply_gravity_value(a.position.x, b.position.x, a.velocity.x, b.velocity.x);
    (a.velocity.y, b.velocity.y) =
        apply_gravity_value(a.position.y, b.position.y, a.velocity.y, b.velocity.y);
    (a.velocity.z, b.velocity.z) =
        apply_gravity_value(a.position.z, b.position.z, a.velocity.z, b.velocity.z);
}

fn update_position(planet: &mut Planet) {
    planet.position.x += planet.velocity.x;
    planet.position.y += planet.velocity.y;
    planet.position.z += planet.velocity.z;
}

fn apply_gravity_value(pos_a: i32, pos_b: i32, vel_a: i32, vel_b: i32) -> (i32, i32) {
    match pos_a.cmp(&pos_b) {
        Ordering::Less => (vel_a + 1, vel_b - 1),
        Ordering::Equal => (vel_a, vel_b),
        Ordering::Greater => (vel_a - 1, vel_b + 1),
    }
}

fn get_pairs(length: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for i in 0..length {
        for j in (i + 1)..length {
            result.push((i, j));
        }
    }

    result
}

fn get_pair_mut<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    let (first, second) = (min(i, j), max(i, j));

    if i == j || second >= slice.len() {
        return None;
    }

    let (_, tmp) = slice.split_at_mut(first);
    let (x, rest) = tmp.split_at_mut(1);
    let (_, y) = rest.split_at_mut(second - first - 1);
    let pair = if i < j {
        (&mut x[0], &mut y[0])
    } else {
        (&mut y[0], &mut x[0])
    };

    Some(pair)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

    const INPUT2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn test_first_part() {
        let mut planets = INPUT1.lines().map(Planet::from).collect::<Vec<_>>();
        assert_eq!(179, get_energy(&mut planets, 10));

        let mut planets = INPUT2.lines().map(Planet::from).collect::<Vec<_>>();
        assert_eq!(1940, get_energy(&mut planets, 100));
    }

    #[test]
    fn test_second_part() {
        assert_eq!(2772, solve_second_part(INPUT1));
        assert_eq!(4686774924, solve_second_part(INPUT2));
    }

    check_answers!(10198, 271442326847376);
}

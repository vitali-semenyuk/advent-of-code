use std::{
    cmp::{max, min},
    fmt::Display,
    ops::Range,
};

#[derive(Debug, Clone, PartialEq)]
struct Seeds(Range<i64>);

impl Seeds {
    fn new(start: i64, size: i64) -> Self {
        let range = start..start + size;
        Self(range)
    }
}

#[derive(Debug)]
struct MapRange {
    source: i64,
    destination: i64,
    size: i64,
}

impl MapRange {
    fn map(&self, seeds: &Seeds) -> Vec<(Seeds, bool)> {
        let diff = self.destination - self.source;

        let intersection = self.intersection(&seeds.0);

        if intersection.is_empty() {
            vec![(seeds.clone(), false)]
        } else {
            let mut v = Vec::new();

            let size = intersection.start - seeds.0.start;
            if size > 0 {
                v.push((Seeds::new(seeds.0.start, size), false));
            }

            let size = intersection.end - intersection.start;
            if size > 0 {
                v.push((Seeds::new(intersection.start + diff, size), true));
            }

            let size = seeds.0.end - intersection.end;
            if size > 0 {
                v.push((Seeds::new(intersection.end, size), false));
            }

            v
        }
    }

    fn intersection(&self, range: &Range<i64>) -> Range<i64> {
        let r = self.source..self.source + self.size;
        max(range.start, r.start)..min(range.end, r.end)
    }
}

impl From<&str> for MapRange {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        let destination = parts.next().unwrap().parse().unwrap();
        let source = parts.next().unwrap().parse().unwrap();
        let size = parts.next().unwrap().parse().unwrap();

        Self {
            source,
            destination,
            size,
        }
    }
}

#[derive(Debug)]
struct Map(Vec<MapRange>);

impl Map {
    fn map(&self, seeds: &Seeds) -> Vec<Seeds> {
        self.0
            .iter()
            .fold(vec![(seeds.clone(), false)], |acc, cur| {
                let processed = acc.clone().into_iter().filter(|(_, p)| *p);
                let new = acc
                    .iter()
                    .filter(|(_, p)| !p)
                    .map(|(s, _)| s)
                    .flat_map(|s| cur.map(s));

                processed.chain(new).collect()
            })
            .into_iter()
            .map(|(s, _)| s)
            .collect()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let ranges = value.lines().skip(1).map(MapRange::from);
        Self(ranges.collect())
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    range_maps: Vec<Map>,
}

impl Almanac {
    fn get_locations_simple(&self) -> Vec<Seeds> {
        let seeds = self.get_seeds();

        self.get_locations(seeds)
    }

    fn get_locations_ranges(&self) -> Vec<Seeds> {
        let seeds = self.get_seeds_ranges();

        self.get_locations(seeds)
    }

    fn get_locations(&self, seeds: Vec<Seeds>) -> Vec<Seeds> {
        seeds
            .into_iter()
            .flat_map(|seed| {
                self.range_maps.iter().fold(vec![seed], |acc, rmap| {
                    acc.iter().flat_map(|s| rmap.map(s)).collect()
                })
            })
            .collect()
    }

    fn get_seeds(&self) -> Vec<Seeds> {
        self.seeds.iter().map(|s| Seeds::new(*s, 1)).collect()
    }

    fn get_seeds_ranges(&self) -> Vec<Seeds> {
        self.seeds
            .chunks(2)
            .map(|a| Seeds::new(a[0], a[1]))
            .collect()
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut parts = value.split("\n\n");
        let (_, seeds) = parts.next().unwrap().split_once(':').unwrap();
        let seeds = seeds
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        let range_maps = parts.map(Map::from).collect();

        Self { seeds, range_maps }
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i64 {
    let almanac = Almanac::from(input);

    almanac
        .get_locations_simple()
        .iter()
        .map(|s| s.0.start)
        .min()
        .unwrap()
}

fn solve_second_part(input: &str) -> i64 {
    let almanac = Almanac::from(input);

    almanac
        .get_locations_ranges()
        .iter()
        .map(|s| s.0.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_first_part() {
        let answer = 35;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 46;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    #[test]
    fn test_get_locations_ranges() {
        let input = "seeds: 1 99

map:
50 98 2
52 50 48
";

        let ranges = Almanac::from(input).get_locations_ranges();

        // 98-100 -> 50-52
        // 50-98 -> 52-100
        // 1-50 -> 1-50
        assert_eq!(ranges.len(), 3);
        assert!(ranges.contains(&Seeds::new(1, 49)));
        assert!(ranges.contains(&Seeds::new(50, 2)));
        assert!(ranges.contains(&Seeds::new(52, 48)));
    }

    check_answers!(462648396, 2520479);
}

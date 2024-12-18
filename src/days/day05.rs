use crate::{Solution, SolutionPair};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day5.txt").unwrap();
    let sol1: usize = part1(&input);
    let sol2: usize = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rules_lines = parts.next().unwrap();
    let updates = parts.next().unwrap();
    let adj_map = build_adjacency_map(rules_lines);

    updates
        .lines()
        .filter_map(|line| {
            let values = line
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            for w in values.windows(2) {
                if !adj_map.contains_key(&w[0]) || !adj_map.get(&w[0]).unwrap().contains(&w[1]) {
                    return None;
                }
            }

            Some(values[values.len() >> 1])
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rules_lines = parts.next().unwrap();
    let updates = parts.next().unwrap();
    let adj_map = build_adjacency_map(rules_lines);

    updates
        .lines()
        .filter_map(|line| {
            let mut values = line
                .split(",")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut ordered: bool = true;

            for w in values.windows(2) {
                if !adj_map.contains_key(&w[0]) || !adj_map.get(&w[0]).unwrap().contains(&w[1]) {
                    ordered = false;
                }
            }

            if !ordered {
                values.sort_by(|a, b| {
                    if adj_map.contains_key(&a) && adj_map.get(&a).unwrap().contains(&b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                Some(values[values.len() >> 1])
            } else {
                None
            }
        })
        .sum()
}

fn build_adjacency_map(rules_lines: &str) -> HashMap<usize, HashSet<usize>> {
    rules_lines
        .lines()
        .map(|line| {
            let mut parts = line.trim().split("|");
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut adj_map, (first, second)| {
            adj_map
                .entry(first)
                .or_insert_with(HashSet::new)
                .insert(second);

            adj_map
        })
}

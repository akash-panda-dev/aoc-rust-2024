use crate::{Solution, SolutionPair};
use std::{collections::HashMap, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

// I need to create pairs of sorted left and right lists.

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day1.txt").unwrap();

    // parse the two lists
    let mut right_frequencies: HashMap<u32, u32> = HashMap::new();

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let l = parts.next().unwrap().parse::<u32>().unwrap();
            let r = parts.next().unwrap().parse::<u32>().unwrap();
            *right_frequencies.entry(r).or_default() += 1;

            (l, r)
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    // Part 1
    let diff_sum: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    // Part 2
    let similarity_score: u32 = left
        .iter()
        .map(|l| l * right_frequencies.get(&l).unwrap_or(&0))
        .sum();

    (Solution::from(diff_sum), Solution::from(similarity_score))
}

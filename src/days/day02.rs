use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day2.txt").unwrap();
    let sol1 = part1(&input);
    let sol2 = part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

fn is_sequence_safe(levels: &[isize]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let increasing = levels[1] > levels[0];
    levels.windows(2).all(|window| {
        let diff = window[1] - window[0];
        match increasing {
            true => diff > 0 && diff <= 3,
            false => diff < 0 && diff >= -3,
        }
    })
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|&report| {
            let levels: Vec<isize> = report
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();
            is_sequence_safe(&levels)
        })
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|report| {
            let levels: Vec<isize> = report
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();

            if is_sequence_safe(&levels) {
                return true;
            }
            for i in 0..levels.len() {
                let mut modified = levels.clone();
                modified.remove(i);
                if is_sequence_safe(&modified) {
                    return true;
                }
            }
            false
        })
        .count()
}

use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day6.txt").unwrap();
    let sol1: u64 = part1(&input);
    let sol2: u64 = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position(i64, i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_position(&self, pos: Position) -> Position {
        match self {
            Direction::Up => Position(pos.0 - 1, pos.1),
            Direction::Right => Position(pos.0, pos.1 + 1),
            Direction::Down => Position(pos.0 + 1, pos.1),
            Direction::Left => Position(pos.0, pos.1 - 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Grid {
    obstacles: HashSet<Position>,
    bounds: (i64, i64),
}

impl Grid {
    fn is_outside(&self, pos: Position) -> bool {
        pos.0 < 0 || pos.1 < 0 || pos.0 > self.bounds.0 || pos.1 > self.bounds.1
    }
}

fn parse(input: &str) -> (Grid, Position) {
    let lines: Vec<&str> = input.lines().collect();
    let bounds = (lines.len() as i64 - 1, lines[0].len() as i64 - 1);
    let mut obstacles = HashSet::new();
    let mut start_pos = Position(0, 0);

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let pos = Position(row as i64, col as i64);
            match ch {
                '#' => {
                    obstacles.insert(pos);
                }
                '^' => start_pos = pos,
                _ => {}
            }
        }
    }

    (Grid { obstacles, bounds }, start_pos)
}

fn part1(input: &str) -> u64 {
    let (grid, mut pos) = parse(input);
    let mut visited = HashSet::new();
    let mut dir = Direction::Up;

    visited.insert(pos);

    loop {
        let next_pos = dir.next_position(pos);

        // Exit if we go outside the grid
        if grid.is_outside(next_pos) {
            break;
        }

        // Turn right if we hit an obstacle, otherwise move forward
        if grid.obstacles.contains(&next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }

    visited.len() as u64
}

fn part2(input: &str) -> u64 {
    let (grid, mut pos) = parse(input);
    let mut visited = HashSet::new();
    let mut dir = Direction::Up;
    let mut possible_loopy_pos = HashSet::new();

    visited.insert(pos);

    loop {
        let next_pos = dir.next_position(pos);

        // Exit if we go outside the grid
        if grid.is_outside(next_pos) {
            break;
        }

        if is_cycle_present(pos, grid.clone(), dir, next_pos) {
            possible_loopy_pos.insert(next_pos);
        }

        // Turn right if we hit an obstacle, otherwise move forward
        if grid.obstacles.contains(&next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
            visited.insert(pos);
        }
    }

    possible_loopy_pos.len() as u64
}

fn is_cycle_present(
    start_pos: Position,
    grid: Grid,
    start_dir: Direction,
    possible_obstacle: Position,
) -> bool {
    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut grid = grid;
    grid.obstacles.insert(possible_obstacle);

    // Store both position and direction in visited to detect cycles
    visited.insert((pos, dir));

    loop {
        let next_pos = dir.next_position(pos);

        // Exit if we go outside the grid
        if grid.is_outside(next_pos) {
            return false;
        }

        // Turn right if we hit an obstacle, otherwise move forward
        if grid.obstacles.contains(&next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }

        if !visited.insert((pos, dir)) {
            return true;
        }
    }
}

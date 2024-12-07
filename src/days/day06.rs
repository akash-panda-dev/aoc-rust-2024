use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

// Use u8 for compact direction representation
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

// Use packed coordinates for Position
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    row: i16,
    col: i16,
}

impl Direction {
    #[inline(always)]
    fn next_position(&self, pos: Position) -> Position {
        match self {
            Direction::Up => Position {
                row: pos.row - 1,
                col: pos.col,
            },
            Direction::Right => Position {
                row: pos.row,
                col: pos.col + 1,
            },
            Direction::Down => Position {
                row: pos.row + 1,
                col: pos.col,
            },
            Direction::Left => Position {
                row: pos.row,
                col: pos.col - 1,
            },
        }
    }

    #[inline(always)]
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

// Use a more efficient grid representation
#[derive(Clone)]
struct Grid {
    // Use a Vec<bool> instead of HashSet for obstacle lookup
    obstacles: Vec<Vec<bool>>,
    rows: i16,
    cols: i16,
}

impl Grid {
    #[inline(always)]
    fn is_outside(&self, pos: Position) -> bool {
        pos.row < 0 || pos.col < 0 || pos.row >= self.rows || pos.col >= self.cols
    }

    #[inline(always)]
    fn has_obstacle(&self, pos: &Position) -> bool {
        self.obstacles[pos.row as usize][pos.col as usize]
    }

    fn add_obstacle(&mut self, pos: Position) {
        self.obstacles[pos.row as usize][pos.col as usize] = true;
    }
}

fn parse(input: &str) -> (Grid, Position) {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len() as i16;
    let cols = lines[0].len() as i16;

    // Pre-allocate with capacity
    let mut obstacles = vec![vec![false; cols as usize]; rows as usize];
    let mut start_pos = Position { row: 0, col: 0 };

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '#' => obstacles[row][col] = true,
                '^' => {
                    start_pos = Position {
                        row: row as i16,
                        col: col as i16,
                    }
                }
                _ => {}
            }
        }
    }

    (
        Grid {
            obstacles,
            rows,
            cols,
        },
        start_pos,
    )
}

fn trace_path(grid: &Grid, start_pos: Position) -> Vec<Position> {
    let mut visited = Vec::new();
    let mut pos = start_pos;
    let mut dir = Direction::Up;

    visited.push(pos);

    loop {
        let next_pos = dir.next_position(pos);
        if grid.is_outside(next_pos) {
            break;
        }

        if grid.has_obstacle(&next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
            visited.push(pos);
        }
    }

    visited
}

fn will_it_loop(start_pos: Position, mut grid: Grid, possible_obstacle: Position) -> bool {
    use std::collections::HashSet;

    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut dir = Direction::Up;

    grid.add_obstacle(possible_obstacle);
    visited.insert((pos, dir));

    loop {
        let next_pos = dir.next_position(pos);
        if grid.is_outside(next_pos) {
            return false;
        }

        if grid.has_obstacle(&next_pos) {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }

        if !visited.insert((pos, dir)) {
            return true;
        }
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day6.txt").unwrap();
    let sol1 = part1(&input);
    let sol2 = part2(&input);
    (Solution::from(sol1), Solution::from(sol2))
}

fn part1(input: &str) -> u64 {
    let (grid, start_pos) = parse(input);
    trace_path(&grid, start_pos).len() as u64
}

fn part2(input: &str) -> u64 {
    let (grid, start_pos) = parse(input);
    let visited = trace_path(&grid, start_pos);

    visited
        .iter()
        .filter(|&&pos| will_it_loop(start_pos, grid.clone(), pos))
        .count() as u64
}



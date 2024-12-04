use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("./input/day4.txt").unwrap();

    let sol1: usize = part1(&input);
    let sol2: usize = part2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX
//
//Parse:
//left -> right
//top -> bottom
//
//zig zag
//try to match both XMAS and SAMX at the same time.
fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut xmas_count: usize = 0;
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];

    // Horizontal check (row by row)
    for row in &grid {
        xmas_count += row.windows(4).filter(|w| w == &XMAS || w == &SAMX).count();
    }

    // Vertical check (column by column)
    for col_idx in 0..num_cols {
        let mut row_idx = 0;
        while row_idx <= num_rows - 4 {
            let sequence = [
                grid[row_idx][col_idx],
                grid[row_idx + 1][col_idx],
                grid[row_idx + 2][col_idx],
                grid[row_idx + 3][col_idx],
            ];
            if XMAS == sequence || SAMX == sequence {
                xmas_count += 1;
            }
            row_idx += 1;
        }
    }

    // Zig Zag check
    for row in 0..num_rows - 3 {
        for col in 0..num_cols - 3 {
            let sequence = [
                grid[row][col],
                grid[row + 1][col + 1],
                grid[row + 2][col + 2],
                grid[row + 3][col + 3],
            ];
            if XMAS == sequence || SAMX == sequence {
                xmas_count += 1;
            }
        }
    }

    for row in 0..num_rows - 3 {
        for col in 0..num_cols - 3 {
            let sequence = [
                grid[row][col + 3],
                grid[row + 1][col + 2],
                grid[row + 2][col + 1],
                grid[row + 3][col],
            ];
            if XMAS == sequence || SAMX == sequence {
                xmas_count += 1;
            }
        }
    }

    xmas_count
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut xmas_count: usize = 0;
    const MAS: [char; 3] = ['M', 'A', 'S'];
    const SAM: [char; 3] = ['S', 'A', 'M'];

    // Zig Zag check
    for row in 0..num_rows - 2 {
        for col in 0..num_cols - 2 {
            let sequence1 = [
                grid[row][col],
                grid[row + 1][col + 1],
                grid[row + 2][col + 2],
            ];

            let sequence2 = [
                grid[row][col + 2],
                grid[row + 1][col + 1],
                grid[row + 2][col],
            ];

            if SAM == sequence1 || MAS == sequence1 {
                if SAM == sequence2 || MAS == sequence2 {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count
}

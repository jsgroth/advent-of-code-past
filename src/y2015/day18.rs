//! Day 18: Like a GIF For Your Yard
//! https://adventofcode.com/2015/day/18

use std::error::Error;
use crate::SimpleError;

fn solve_part(input: &str, steps: usize, locked_corners: bool) -> Result<usize, SimpleError> {
    let mut grid = parse_input(input);
    if grid.is_empty() {
        return Err(SimpleError::new(String::from("input grid is empty")));
    }

    if locked_corners {
        turn_on_corners(&mut grid);
    }

    for _ in 0..steps {
        grid = simulate_turn(&grid, locked_corners);
    }

    let final_count = grid.into_iter().map(|row| {
        row.into_iter().filter(|&b| b).count()
    })
        .sum();

    Ok(final_count)
}

fn simulate_turn(grid: &Vec<Vec<bool>>, locked_corners: bool) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; grid[0].len()]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, &on) in row.iter().enumerate() {
            let on_neighbors = count_neighbors(grid, i, j);
            if on {
                new_grid[i][j] = on_neighbors == 2 || on_neighbors == 3;
            } else {
                new_grid[i][j] = on_neighbors == 3;
            }
        }
    }

    if locked_corners {
        turn_on_corners(&mut new_grid);
    }

    new_grid
}

fn count_neighbors(grid: &Vec<Vec<bool>>, i: usize, j: usize) -> usize {
    let mut on_neighbors = 0;
    for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        let ii = (i as i32) + dy;
        let jj = (j as i32) + dx;

        if ii < 0 || jj < 0 || ii >= grid.len() as i32 || jj >= grid[0].len() as i32 {
            continue;
        }

        if grid[ii as usize][jj as usize] {
            on_neighbors += 1;
        }
    }
    on_neighbors
}

fn turn_on_corners(grid: &mut Vec<Vec<bool>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    grid[0][0] = true;
    grid[0][cols - 1] = true;
    grid[rows - 1][0] = true;
    grid[rows - 1][cols - 1] = true;
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| {
        line.chars().map(|c| c == '#').collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, 100, false)?;
    let solution2 = solve_part(input, 100, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample18.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(4), solve_part(SAMPLE_INPUT, 4, false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(17), solve_part(SAMPLE_INPUT, 5, true));
    }
}
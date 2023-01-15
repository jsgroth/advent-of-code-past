//! Day 24: Planet of Discord
//! https://adventofcode.com/2019/day/24

use std::collections::HashSet;
use std::error::Error;
use std::iter;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let mut grid = parse_input(input)?;

    let mut previous_states: HashSet<_> = iter::once(grid.clone()).collect();

    loop {
        grid = simulate_iteration(&grid);

        if !previous_states.insert(grid.clone()) {
            return Ok(biodiversity_rating(&grid));
        }
    }
}

fn solve_part_2(input: &str, minutes: usize) -> Result<usize, SimpleError> {
    let initial_grid = parse_input(input)?;

    let mut grids = Vec::new();
    grids.push(vec![vec![false; initial_grid[0].len()]; initial_grid.len()]);
    grids.push(initial_grid.clone());
    grids.push(vec![vec![false; initial_grid[0].len()]; initial_grid.len()]);

    for _ in 0..minutes {
        grids = simulate_iteration_recursive(&grids);
    }

    let bugs = grids.iter().map(|grid| {
        grid.iter().map(|row| {
            row.iter().filter(|&&b| b).count()
        })
            .sum::<usize>()
    })
        .sum();

    Ok(bugs)
}

fn biodiversity_rating(grid: &Vec<Vec<bool>>) -> u64 {
    let mut total_rating = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, &bug) in row.iter().enumerate() {
            if bug {
                total_rating += 2_u64.pow((i * grid[0].len() + j) as u32);
            }
        }
    }

    total_rating
}

fn simulate_iteration(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; grid[0].len()]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, &bug) in row.iter().enumerate() {
            let mut neighbor_count = 0;
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if i == 0 && di == -1 || j == 0 && dj == -1 {
                    continue;
                }

                let adj_i = (i as i32 + di) as usize;
                let adj_j = (j as i32 + dj) as usize;
                if adj_i >= grid.len() || adj_j >= grid[0].len() {
                    continue;
                }

                if grid[adj_i][adj_j] {
                    neighbor_count += 1;
                }
            }

            new_grid[i][j] = neighbor_count == 1 || (!bug && neighbor_count == 2);
        }
    }

    new_grid
}

fn simulate_iteration_recursive(grids: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    let mut new_grids = Vec::new();

    for (grid_index, grid) in grids.iter().enumerate() {
        let mut new_grid = vec![vec![false; grid[0].len()]; grid.len()];
        for (i, row) in grid.iter().enumerate() {
            for (j, &bug) in row.iter().enumerate() {
                if i == 2 && j == 2 {
                    continue;
                }

                let mut neighbor_count = 0;

                for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let adj_i = i as i32 + di;
                    let adj_j = j as i32 + dj;

                    if adj_i == -1 {
                        if grid_index > 0 && grids[grid_index - 1][1][2] {
                            neighbor_count += 1;
                        }
                    } else if adj_i == 5 {
                        if grid_index > 0 && grids[grid_index - 1][3][2] {
                            neighbor_count += 1;
                        }
                    } else if adj_j == -1 {
                        if grid_index > 0 && grids[grid_index - 1][2][1] {
                            neighbor_count += 1;
                        }
                    } else if adj_j == 5 {
                        if grid_index > 0 && grids[grid_index - 1][2][3] {
                            neighbor_count += 1;
                        }
                    } else if adj_i == 2 && adj_j == 2 {
                        if grid_index < grids.len() - 1 {
                            neighbor_count += count_from_direction(&grids[grid_index + 1], di, dj);
                        }
                    } else {
                        if grid[adj_i as usize][adj_j as usize] {
                            neighbor_count += 1;
                        }
                    }
                }

                new_grid[i][j] = neighbor_count == 1 || (!bug && neighbor_count == 2);
            }
        }
        new_grids.push(new_grid);
    }

    if should_prepend(&new_grids[0]) {
        new_grids.insert(0, vec![vec![false; 5]; 5]);
    }

    if should_append(&new_grids[new_grids.len() - 1]) {
        new_grids.push(vec![vec![false; 5]; 5]);
    }

    new_grids
}

fn count_from_direction(grid: &Vec<Vec<bool>>, di: i32, dj: i32) -> usize {
    match (di, dj) {
        (-1, 0) => {
            grid[4].iter().filter(|&&b| b).count()
        }
        (1, 0) => {
            grid[0].iter().filter(|&&b| b).count()
        }
        (0, -1) => {
            (0..5).filter(|&i| grid[i][4]).count()
        }
        (0, 1) => {
            (0..5).filter(|&i| grid[i][0]).count()
        }
        _ => panic!("unexpected di/dj: di={di}, dj={dj}")
    }
}

fn should_prepend(first_grid: &Vec<Vec<bool>>) -> bool {
    if first_grid[0].iter().any(|&b| b) {
        return true;
    }

    if first_grid[4].iter().any(|&b| b) {
        return true;
    }

    (1..4).any(|i| first_grid[i][0] || first_grid[i][4])
}

fn should_append(last_grid: &Vec<Vec<bool>>) -> bool {
    for i in 1..4 {
        for j in 1..4 {
            if last_grid[i][j] {
                return true;
            }
        }
    }

    false
}

fn parse_input(input: &str) -> Result<Vec<Vec<bool>>, SimpleError> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                '#' => Ok(true),
                '.' => Ok(false),
                _ => Err(SimpleError::new(format!("unexpected char: {c}")))
            }
        })
            .collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(u64, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input, 200)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample24.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(2129920), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(99), solve_part_2(SAMPLE_INPUT, 10));
    }
}
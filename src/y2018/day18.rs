//! Day 18: Settlers of The North Pole
//! https://adventofcode.com/2018/day/18

use std::collections::HashMap;
use std::error::Error;
use std::iter;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Space {
    Open,
    Tree,
    Lumberyard,
}

impl Space {
    fn ordinal(&self) -> usize {
        match self {
            Self::Open => 0,
            Self::Tree => 1,
            Self::Lumberyard => 2,
        }
    }
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let mut collection_area = parse_input(input)?;

    for _ in 0..10 {
        collection_area = simulate_iteration(&collection_area);
    }

    Ok(compute_score(&collection_area))
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let mut collection_area = parse_input(input)?;

    let mut past_collection_areas: HashMap<_, _> = iter::once((collection_area.clone(), 0)).collect();
    for i in 1.. {
        collection_area = simulate_iteration(&collection_area);

        if let Some(&previous_iteration) = past_collection_areas.get(&collection_area) {
            let rem = (1_000_000_000 - i) % (i - previous_iteration);
            for _ in 0..rem {
                collection_area = simulate_iteration(&collection_area);
            }

            return Ok(compute_score(&collection_area));
        }

        past_collection_areas.insert(collection_area.clone(), i);
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn compute_score(collection_area: &Vec<Vec<Space>>) -> usize {
    let mut tree_count = 0;
    let mut lumberyard_count = 0;
    for row in collection_area {
        for &space in row {
            match space {
                Space::Tree => tree_count += 1,
                Space::Lumberyard => lumberyard_count += 1,
                Space::Open => {},
            }
        }
    }

    tree_count * lumberyard_count
}

fn simulate_iteration(collection_area: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let mut next_area = vec![vec![Space::Open; collection_area[0].len()]; collection_area.len()];

    for (i, row) in collection_area.iter().enumerate() {
        for (j, &space) in row.iter().enumerate() {
            let neighbor_counts = count_neighbors(&collection_area, i, j);
            next_area[i][j] = match space {
                Space::Open => {
                    if neighbor_counts[Space::Tree.ordinal()] >= 3 {
                        Space::Tree
                    } else {
                        Space::Open
                    }
                }
                Space::Tree => {
                    if neighbor_counts[Space::Lumberyard.ordinal()] >= 3 {
                        Space::Lumberyard
                    } else {
                        Space::Tree
                    }
                }
                Space::Lumberyard => {
                    let adjacent_trees = neighbor_counts[Space::Tree.ordinal()];
                    let adjacent_lumberyards = neighbor_counts[Space::Lumberyard.ordinal()];
                    if adjacent_trees >= 1 && adjacent_lumberyards >= 1 {
                        Space::Lumberyard
                    } else {
                        Space::Open
                    }
                }
            }
        }
    }

    next_area
}

fn count_neighbors(collection_area: &Vec<Vec<Space>>, i: usize , j: usize) -> [u32; 3] {
    let mut neighbor_counts = [0; 3];

    for (di, dj) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        if i == 0 && di == -1 || j == 0 && dj == -1 {
            continue;
        }

        let ii = ((i as i32) + di) as usize;
        let jj = ((j as i32) + dj) as usize;
        if ii >= collection_area.len() || jj >= collection_area[0].len() {
            continue;
        }

        neighbor_counts[collection_area[ii][jj].ordinal()] += 1;
    }

    neighbor_counts
}

fn parse_input(input: &str) -> Result<Vec<Vec<Space>>, SimpleError> {
    let lines: Vec<_> = input.lines().collect();
    if lines.is_empty() {
        return Err(SimpleError::new(String::from("input has no lines")));
    }

    let mut collection_area = vec![vec![Space::Open; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            collection_area[i][j] = match c {
                '.' => Space::Open,
                '|' => Space::Tree,
                '#' => Space::Lumberyard,
                _ => return Err(SimpleError::new(format!("invalid char at ({i}, {j}): {c}")))
            }
        }
    }

    Ok(collection_area)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample18.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(1147), solve_part_1(SAMPLE_INPUT));
    }
}
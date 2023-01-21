//! Day 11: Seating System
//! https://adventofcode.com/2020/day/11

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let mut map = parse_input(input)?;

    loop {
        let next_map = simulate_iteration(
            &map,
            count_neighbors_adjacent,
            4,
        );

        if map == next_map {
            return Ok(count_occupied(&map));
        }

        map = next_map;
    }
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let mut map = parse_input(input)?;

    loop {
        let next_map = simulate_iteration(
            &map,
            count_neighbors_line_of_sight,
            5,
        );

        if map == next_map {
            return Ok(count_occupied(&map));
        }

        map = next_map;
    }
}

fn count_occupied(map: &[Vec<Space>]) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|&&space| space == Space::OccupiedSeat).count())
        .sum()
}

fn simulate_iteration(
    map: &Vec<Vec<Space>>,
    neighbor_count_fn: impl Fn(&Vec<Vec<Space>>, usize, usize) -> usize,
    occupied_neighbor_threshold: usize,
) -> Vec<Vec<Space>> {
    let mut new_map = vec![vec![Space::Floor; map[0].len()]; map.len()];

    for (i, row) in map.iter().enumerate() {
        for (j, &space) in row.iter().enumerate() {
            let neighbors = neighbor_count_fn(map, i, j);

            let new_space = match space {
                Space::Floor => Space::Floor,
                Space::EmptySeat => {
                    if neighbors == 0 {
                        Space::OccupiedSeat
                    } else {
                        Space::EmptySeat
                    }
                }
                Space::OccupiedSeat => {
                    if neighbors >= occupied_neighbor_threshold {
                        Space::EmptySeat
                    } else {
                        Space::OccupiedSeat
                    }
                }
            };

            new_map[i][j] = new_space;
        }
    }

    new_map
}

const DIRECTIONS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn count_neighbors_adjacent(map: &Vec<Vec<Space>>, i: usize, j: usize) -> usize {
    let mut neighbors = 0;

    for &(di, dj) in &DIRECTIONS {
        if i == 0 && di == -1 || j == 0 && dj == -1 {
            continue;
        }

        let adj_i = (i as i32 + di) as usize;
        let adj_j = (j as i32 + dj) as usize;
        if adj_i >= map.len() || adj_j >= map[0].len() {
            continue;
        }

        if let Space::OccupiedSeat = map[adj_i][adj_j] {
            neighbors += 1;
        }
    }

    neighbors
}

fn count_neighbors_line_of_sight(map: &Vec<Vec<Space>>, i: usize, j: usize) -> usize {
    let mut neighbors = 0;

    for &(di, dj) in &DIRECTIONS {
        let mut i = i;
        let mut j = j;
        loop {
            if i == 0 && di == -1 || j == 0 && dj == -1 {
                break;
            }

            i = (i as i32 + di) as usize;
            j = (j as i32 + dj) as usize;
            if i >= map.len() || j >= map[0].len() {
                break;
            }

            match map[i][j] {
                Space::OccupiedSeat => {
                    neighbors += 1;
                    break;
                }
                Space::EmptySeat => {
                    break;
                }
                Space::Floor => {}
            }
        }
    }

    neighbors
}

fn parse_input(input: &str) -> Result<Vec<Vec<Space>>, SimpleError> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match c {
                'L' => Ok(Space::EmptySeat),
                '.' => Ok(Space::Floor),
                _ => Err(SimpleError::new(format!("unexpected char: {c}")))
            }
        })
            .collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample11.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(37), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(26), solve_part_2(SAMPLE_INPUT));
    }
}
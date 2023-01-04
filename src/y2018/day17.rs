//! Day 17: Reservoir Research
//! https://adventofcode.com/2018/day/17

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Clay,
    RestingWater,
    RunningWater,
}

impl Space {
    fn is_water(&self) -> bool {
        match self {
            Self::RestingWater | Self::RunningWater => true,
            _ => false
        }
    }

    fn is_solid(&self) -> bool {
        match self {
            Self::Clay | Self::RestingWater => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ClayVein {
    Vertical { x: u32, y_min: u32, y_max: u32 },
    Horizontal { x_min: u32, x_max: u32, y: u32 },
}

impl ClayVein {
    fn new_vertical(x: u32, y_min: u32, y_max: u32) -> Self {
        Self::Vertical { x, y_min, y_max }
    }

    fn new_horizontal(x_min: u32, x_max: u32, y: u32) -> Self {
        Self::Horizontal { x_min, x_max, y }
    }
}

fn solve_both_parts(input: &str) -> Result<(usize, usize), SimpleError> {
    let clay_veins = parse_input(input)?;

    let mut map = build_map(&clay_veins);
    flood(&mut map, 500, 0);

    let min_y = clay_veins.iter()
        .map(|&clay_vein| match clay_vein {
            ClayVein::Vertical { y_min, .. } => y_min as usize,
            ClayVein::Horizontal { y, .. } => y as usize,
        })
        .min()
        .unwrap();

    let mut water_count = 0;
    let mut resting_water_count = 0;
    for y in min_y..map.len() {
        let row = &map[y];
        water_count += row.iter().filter(|&&space| space.is_water()).count();
        resting_water_count += row.iter().filter(|&&space| space == Space::RestingWater).count();
    }

    Ok((water_count, resting_water_count))
}

fn flood(map: &mut Vec<Vec<Space>>, x: usize, y: usize) {
    // Descend downwards as far as possible
    let mut current_y = y;
    while current_y < map.len() - 1 && map[current_y + 1][x] == Space::Empty {
        current_y += 1;
    }

    if current_y == map.len() - 1 {
        // Reached the bottom of the map, this is just a downwards stream of running water
        for running_y in y..=current_y {
            map[running_y][x] = Space::RunningWater;
        }
        return;
    }

    while current_y >= y {
        // Determine how far left and right the water can go from here
        let left_x = {
            let mut current_x = x;
            while map[current_y + 1][current_x].is_solid() && map[current_y][current_x - 1] == Space::Empty {
                current_x -= 1;
            }
            current_x
        };

        let right_x = {
            let mut current_x = x;
            while map[current_y + 1][current_x].is_solid() && map[current_y][current_x + 1] == Space::Empty {
                current_x += 1;
            }
            current_x
        };

        // If either the left or right edge is above an empty space, recursively flood downwards
        let mut ran_off_edge = false;
        for end_x in [left_x, right_x] {
            if map[current_y + 1][end_x] == Space::Empty {
                flood(map, end_x, current_y + 1);
                ran_off_edge = true;
            }
        }

        // If the water ran off either edge, we need to process this row again because the row
        // below could now contain resting water that allows this row to extend farther out
        if ran_off_edge {
            continue;
        }

        // If this row ends in a space above running water, this entire row and the remainder of
        // the stream upwards are running water
        if map[current_y + 1][left_x] == Space::RunningWater || map[current_y + 1][right_x] == Space::RunningWater {
            for running_x in left_x..=right_x {
                map[current_y][running_x] = Space::RunningWater;
            }
            for running_y in y..current_y {
                map[running_y][x] = Space::RunningWater;
            }
            return;
        }

        // Water did not run off the edges and this row is not above running water, therefore
        // this row is full of water at rest - fill it and move up to the previous row
        for resting_x in left_x..=right_x {
            map[current_y][resting_x] = Space::RestingWater;
        }
        current_y -= 1;
    }
}

fn build_map(clay_veins: &[ClayVein]) -> Vec<Vec<Space>> {
    let max_x = clay_veins.iter()
        .map(|&clay_vein| match clay_vein {
            ClayVein::Vertical { x, .. } => x,
            ClayVein::Horizontal { x_max, .. } => x_max,
        })
        .max()
        .unwrap();

    let max_y = clay_veins.iter()
        .map(|&clay_vein| match clay_vein {
            ClayVein::Vertical { y_max, .. } => y_max,
            ClayVein::Horizontal { y, .. } => y,
        })
        .max()
        .unwrap();

    // Leave extra space at the x end to allow water to run off to the right of the rightmost clay
    let mut map = vec![vec![Space::Empty; max_x as usize + 2]; max_y as usize + 1];
    for &clay_vein in clay_veins {
        match clay_vein {
            ClayVein::Vertical { x, y_min, y_max } => {
                for y in y_min..=y_max {
                    map[y as usize][x as usize] = Space::Clay;
                }
            }
            ClayVein::Horizontal { x_min, x_max, y } => {
                for x in x_min..=x_max {
                    map[y as usize][x as usize] = Space::Clay;
                }
            }
        }
    }

    map
}

fn parse_input(input: &str) -> Result<Vec<ClayVein>, SimpleError> {
    input.lines().map(|line| {
        let (l, r) = line.split_once(", ").ok_or(
            SimpleError::new(format!("invalid line format: {line}"))
        )?;

        match &line[..2] {
            "x=" => {
                let x = &l[2..];

                let (y_min, y_max) = r[2..].split_once("..").ok_or(
                    SimpleError::new(format!("invalid y range format in line: {line}"))
                )?;

                Ok(ClayVein::new_vertical(x.parse()?, y_min.parse()?, y_max.parse()?))
            }
            "y=" => {
                let y = &l[2..];

                let (x_min, x_max) = r[2..].split_once("..").ok_or(
                    SimpleError::new(format!("invalid x range format in line: {line}"))
                )?;

                Ok(ClayVein::new_horizontal(x_min.parse()?, x_max.parse()?, y.parse()?))
            }
            _ => Err(SimpleError::new(format!("expected line to start with 'x=' or 'y=': {line}")))
        }
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample17.txt");

    #[test]
    fn test_sample_input() {
        assert_eq!(Ok((57, 29)), solve_both_parts(SAMPLE_INPUT));
    }
}
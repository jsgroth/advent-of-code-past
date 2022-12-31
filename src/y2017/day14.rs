//! Day 14: Disk Defragmentation
//! https://adventofcode.com/2017/day/14

use std::error::Error;
use crate::SimpleError;
use crate::y2017::knothash;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let key_string = crate::read_single_line(input)?;

    let total_used = (0..128).map(|i| {
        knothash::compute_knot_hash(format!("{key_string}-{i}").as_str())
            .into_iter()
            .map(|byte| byte.count_ones())
            .sum::<u32>()
    })
        .sum();

    Ok(total_used)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let key_string = crate::read_single_line(input)?;

    let mut used_squares = vec![vec![false; 128]; 128];

    for i in 0..128 {
        let hash = knothash::compute_knot_hash(format!("{key_string}-{i}").as_str());
        for j in 0..16 {
            for k in 0..8 {
                let is_used = (hash[j] & (0x01 << (7 - k))) != 0;
                used_squares[i][8 * j + k] = is_used;
            }
        }
    }

    let mut region_markers = vec![vec![0; 128]; 128];
    let mut region_count = 0;
    for i in 0..128 {
        for j in 0..128 {
            if used_squares[i][j] && region_markers[i][j] == 0 {
                fill_region(&used_squares, i, j, &mut region_markers, region_count + 1);
                region_count += 1;
            }
        }
    }

    Ok(region_count)
}

fn fill_region(used_squares: &Vec<Vec<bool>>, i: usize, j: usize, region_markers: &mut Vec<Vec<u32>>, region: u32) {
    region_markers[i][j] = region;

    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if i == 0 && di == -1 || j == 0 && dj == -1 {
            continue;
        }

        let new_i = ((i as i32) + di) as usize;
        let new_j = ((j as i32) + dj) as usize;
        if new_i >= used_squares.len() || new_j >= used_squares[0].len() {
            continue;
        }

        if used_squares[new_i][new_j] && region_markers[new_i][new_j] == 0 {
            fill_region(used_squares, new_i, new_j, region_markers, region);
        }
    }
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(8108), solve_part_1("flqrgnkx"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(1242), solve_part_2("flqrgnkx"));
    }
}
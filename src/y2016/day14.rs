//! Day 14: One-Time Pad
//!
//! <https://adventofcode.com/2016/day/14>

use crate::SimpleError;
use std::collections::VecDeque;
use std::error::Error;

fn solve_part(input: &str, stretch_hashes: bool) -> Result<usize, SimpleError> {
    let salt = crate::read_single_line(input)?;

    let mut keys_found: Vec<(usize, char)> = Vec::new();
    let mut possible_keys: VecDeque<(usize, char)> = VecDeque::new();

    for i in 0.. {
        while i >= 1000
            && !possible_keys.is_empty()
            && possible_keys.front().copied().unwrap().0 < i - 1000
        {
            possible_keys.pop_front();
        }

        if keys_found.len() >= 64 && possible_keys.is_empty() {
            return Ok(keys_found[63].0);
        }

        let mut digest = md5::compute(format!("{salt}{i}").as_bytes());
        if stretch_hashes {
            for _ in 0..2016 {
                digest = md5::compute(format!("{digest:x}").as_bytes());
            }
        }

        let hex_digest = format!("{digest:x}");
        let chars: Vec<_> = hex_digest.chars().collect();

        let mut iteration_keys_found = Vec::new();
        for &(key_index, key_char) in &possible_keys {
            for window in chars.windows(5) {
                if window.iter().all(|&c| c == key_char) {
                    iteration_keys_found.push((key_index, key_char));
                    break;
                }
            }
        }

        keys_found.extend(iteration_keys_found.clone());
        keys_found.sort_by_key(|&(key_index, _)| key_index);
        possible_keys.retain(|possible_key| !iteration_keys_found.contains(possible_key));

        if keys_found.len() >= 64 {
            continue;
        }

        for window in chars.windows(3) {
            if window[0] == window[1] && window[0] == window[2] && window[1] == window[2] {
                possible_keys.push_back((i, window[0]));
                break;
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Takes too long
    fn test_sample_input_part_1() {
        assert_eq!(Ok(22728), solve_part("abc", false));
    }

    #[test]
    #[ignore] // Takes WAY too long
    fn test_sample_input_part_2() {
        assert_eq!(Ok(22551), solve_part("abc", true));
    }
}

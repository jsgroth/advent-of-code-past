//! Day 16: Dragon Checksum
//! https://adventofcode.com/2016/day/16

use std::error::Error;
use crate::SimpleError;

fn solve_part(input: &str, disk_size: usize) -> Result<String, SimpleError> {
    let initial_state = crate::read_single_line(input)?;

    let mut current_state = String::from(initial_state);
    while current_state.len() < disk_size {
        let current_state_flipped: String = current_state.chars().rev()
            .map(|c| if c == '1' { '0' } else { '1' })
            .collect();

        current_state.push('0');
        current_state.push_str(&current_state_flipped);
    }

    current_state.truncate(disk_size);

    Ok(checksum(&current_state))
}

fn checksum(s: &str) -> String {
    let mut result = String::from(s);
    while result.len() % 2 == 0 {
        let chars: Vec<_> = result.chars().collect();
        result = chars.chunks(2)
            .map(|window| {
                if window[0] == window[1] { '1' } else { '0' }
            })
            .collect();
    }
    result
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part(input, 272)?;
    let solution2 = solve_part(input, 35651584)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("01100")), solve_part("10000", 20));
    }
}
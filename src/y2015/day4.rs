//! Day 4: The Ideal Stocking Stuffer
//! https://adventofcode.com/2015/day/4

use crate::SimpleError;
use std::error::Error;

fn solve_part(input: &str, target_prefix: &str) -> Result<usize, SimpleError> {
    let line = crate::read_single_line(input)?;

    for i in 0.. {
        let s = format!("{line}{i}");
        let digest = md5::compute(s.as_bytes());
        let digest = format!("{digest:x}");

        if digest.starts_with(target_prefix) {
            return Ok(i);
        }
    }

    Err(SimpleError::new(String::from(
        "no solution found for part 1",
    )))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, "00000")?;
    let solution2 = solve_part(input, "000000")?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Takes several seconds to run
    fn test_sample_input_part_1() {
        assert_eq!(Ok(609043), solve_part("abcdef", "00000"));
        assert_eq!(Ok(1048970), solve_part("pqrstuv", "00000"));
    }

    #[test]
    fn test_invalid_input() {
        assert!(solve_part("", "000").is_err());
    }
}

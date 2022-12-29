//! Day 5: How About a Nice Game of Chess?
//! https://adventofcode.com/2016/day/5

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<String, SimpleError> {
    let door_id = crate::read_single_line(input)?;

    let mut password = String::new();
    for i in 0.. {
        let digest = md5::compute(format!("{door_id}{i}").as_bytes());
        let hex = format!("{digest:x}");
        if hex.starts_with("00000") {
            password.push(hex.chars().skip(5).next().unwrap());
            if password.len() == 8 {
                return Ok(password);
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let door_id = crate::read_single_line(input)?;

    let mut password = vec![0xFF_u8; 8];
    for i in 0.. {
        let digest = md5::compute(format!("{door_id}{i}").as_bytes());
        let hex = format!("{digest:x}");
        if hex.starts_with("00000") {
            let hex_bytes = hex.as_bytes();
            if ('0'..='7').contains(&(hex_bytes[5] as char)) {
                let index = (hex_bytes[5] - ('0' as u8)) as usize;
                if password[index] == 0xFF {
                    password[index as usize] = hex_bytes[6];

                    if password.iter().all(|&b| b != 0xFF) {
                        return Ok(String::from_utf8(password)?);
                    }
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Takes too long
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("18f47a30")), solve_part_1("abc"));
    }

    #[test]
    #[ignore] // Takes too long
    fn test_sample_input_part_2() {
        assert_eq!(Ok(String::from("05ace8e3")), solve_part_2("abc"));
    }
}
//! Day 5: Binary Boarding
//! https://adventofcode.com/2020/day/5

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let seat_ids: Vec<_> = input.lines()
        .map(|line| seat_id(line))
        .collect::<Result<_, _>>()?;

    Ok(seat_ids.into_iter().max().unwrap())
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let mut seat_ids: Vec<_> = input.lines()
        .map(|line| seat_id(line))
        .collect::<Result<_, _>>()?;

    seat_ids.sort();

    for i in 1..seat_ids.len() - 1 {
        if seat_ids[i + 1] != seat_ids[i] + 1 {
            return Ok(seat_ids[i] + 1);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn seat_id(seat: &str) -> Result<u32, SimpleError> {
    if seat.len() != 10 {
        return Err(SimpleError::new(format!("input string should be exactly 10 characters: {seat}")));
    }

    let mut row_start = 0;
    let mut row_end = 128;
    for row_direction in seat.chars().take(7) {
        match row_direction {
            'F' => {
                row_end = (row_end + row_start) / 2;
            }
            'B' => {
                row_start = (row_end + row_start) / 2;
            }
            _ => return Err(SimpleError::new(format!("invalid row direction: {row_direction}")))
        }
    }

    let mut col_start = 0;
    let mut col_end = 8;
    for col_direction in seat.chars().skip(7) {
        match col_direction {
            'L' => {
                col_end = (col_end + col_start) / 2;
            }
            'R' => {
                col_start = (col_end + col_start) / 2;
            }
            _ => return Err(SimpleError::new(format!("invalid column direction: {col_direction}")))
        }
    }

    Ok(8 * row_start + col_start)
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
        assert_eq!(Ok(357), seat_id("FBFBBFFRLR"));
        assert_eq!(Ok(567), seat_id("BFFFBBFRRR"));
        assert_eq!(Ok(119), seat_id("FFFBBBFRRR"));
        assert_eq!(Ok(820), seat_id("BBFFBBFRLL"));
    }
}
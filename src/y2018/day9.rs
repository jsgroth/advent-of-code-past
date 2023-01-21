//! Day 9: Marble Mania
//!
//! <https://adventofcode.com/2018/day/9>

use crate::SimpleError;
use std::collections::VecDeque;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let (players, last_marble) = parse_input(input)?;

    Ok(compute_max_player_score(players, last_marble))
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let (players, last_marble) = parse_input(input)?;

    Ok(compute_max_player_score(players, last_marble * 100))
}

fn compute_max_player_score(players: u64, last_marble: u64) -> u64 {
    let mut circle = VecDeque::new();
    circle.push_front(0);
    circle.push_front(1);

    let mut player_scores = vec![0; players as usize];
    for marble in 2..=last_marble {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            let removed = circle.pop_front().unwrap();

            let player = (marble % players) as usize;
            player_scores[player] += marble + removed;
        } else {
            circle.rotate_left(2);
            circle.push_front(marble);
        }
    }

    player_scores.into_iter().max().unwrap()
}

fn parse_input(input: &str) -> Result<(u64, u64), SimpleError> {
    let line = crate::read_single_line(input)?;
    let split: Vec<_> = line.split(' ').collect();
    if split.len() != 8 {
        return Err(SimpleError::new(format!(
            "expected 8 words in line: {line}"
        )));
    }

    let players: u64 = split[0].parse()?;
    let last_marble: u64 = split[6].parse()?;
    Ok((players, last_marble))
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_string(players: usize, last_marble: usize) -> String {
        format!("{players} players; last marble is worth {last_marble} points")
    }

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(32), solve_part_1(test_string(9, 25).as_str()));
        assert_eq!(Ok(8317), solve_part_1(test_string(10, 1618).as_str()));
        assert_eq!(Ok(146373), solve_part_1(test_string(13, 7999).as_str()));
        assert_eq!(Ok(2764), solve_part_1(test_string(17, 1104).as_str()));
        assert_eq!(Ok(54718), solve_part_1(test_string(21, 6111).as_str()));
        assert_eq!(Ok(37305), solve_part_1(test_string(30, 5807).as_str()));
    }
}

//! Day 17: Two Steps Forward
//! https://adventofcode.com/2016/day/17

use std::collections::VecDeque;
use std::error::Error;
use std::ops::RangeInclusive;
use crate::SimpleError;

#[derive(Debug)]
struct PathEntry {
    steps: usize,
    path: String,
    position: (usize, usize),
}

impl PathEntry {
    fn new(steps: usize, path: String, position: (usize, usize)) -> Self {
        Self { steps, path, position }
    }
}

const CAN_MOVE: RangeInclusive<char> = 'b'..='f';

fn solve_part(input: &str, find_longest_path: bool) -> Result<String, SimpleError> {
    let passcode = crate::read_single_line(input)?;

    let mut queue = VecDeque::new();
    queue.push_back(PathEntry::new(0, String::new(), (0, 0)));

    let mut longest_path_so_far: Option<String> = None;
    while !queue.is_empty() {
        let PathEntry { steps, path, position: (i, j) } = queue.pop_front().unwrap();

        let digest = md5::compute(format!("{passcode}{path}").as_bytes());
        let hex_digest_prefix: Vec<_> = format!("{digest:x}").chars().take(4).collect();

        let mut new_entries = Vec::new();
        if i > 0 && CAN_MOVE.contains(&hex_digest_prefix[0]) {
            new_entries.push(PathEntry::new(steps + 1, format!("{path}U"), (i - 1, j)));
        }
        if i < 3 && CAN_MOVE.contains(&hex_digest_prefix[1]) {
            new_entries.push(PathEntry::new(steps + 1, format!("{path}D"), (i + 1, j)));
        }
        if j > 0 && CAN_MOVE.contains(&hex_digest_prefix[2]) {
            new_entries.push(PathEntry::new(steps + 1, format!("{path}L"), (i, j - 1)));
        }
        if j < 3 && CAN_MOVE.contains(&hex_digest_prefix[3]) {
            new_entries.push(PathEntry::new(steps + 1, format!("{path}R"), (i, j + 1)));
        }

        if let Some(entry) = new_entries.iter().find(|entry| entry.position == (3, 3)) {
            if find_longest_path {
                longest_path_so_far = Some(entry.path.clone());
            } else {
                return Ok(entry.path.clone());
            }
        }

        new_entries.retain(|entry| entry.position != (3, 3));

        queue.extend(new_entries);
    }

    if find_longest_path {
        longest_path_so_far.ok_or_else(
            || SimpleError::new(String::from("no solution found"))
        )
    } else {
        Err(SimpleError::new(String::from("no solution found")))
    }
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    Ok(solve_part(input, true)?.len())
}

pub fn solve(input: &str) -> Result<(String, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("DDRRRD")), solve_part("ihgpwlah", false));
        assert_eq!(Ok(String::from("DDUDRLRRUDRD")), solve_part("kglvqrro", false));
        assert_eq!(Ok(String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR")), solve_part("ulqzkmiv", false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(370), solve_part_2("ihgpwlah"));
        assert_eq!(Ok(492), solve_part_2("kglvqrro"));
        assert_eq!(Ok(830), solve_part_2("ulqzkmiv"));
    }
}
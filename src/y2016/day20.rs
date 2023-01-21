//! Day 20: Firewall Rules
//! https://adventofcode.com/2016/day/20

use std::cmp;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct IpRange {
    start: u32,
    end: u32,
}

impl IpRange {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn combine(&self, other: &Self) -> Option<Self> {
        if self.start > other.start {
            return other.combine(self);
        }

        if other.start > self.end.saturating_add(1) {
            return None;
        }

        Some(Self { start: self.start, end: cmp::max(self.end, other.end) })
    }
}

fn solve_part_1(input: &str, max_valid_value: u32) -> Result<u32, SimpleError> {
    let ranges = parse_input(input)?;
    let combined_ranges = combine_ranges(ranges);

    if combined_ranges[0] == IpRange::new(0, max_valid_value) {
        return Err(SimpleError::new(String::from("no possible solution")));
    }

    let min_allowed_value = if combined_ranges[0].start == 0 {
        combined_ranges[0].end + 1
    } else {
        0
    };

    Ok(min_allowed_value)
}

fn solve_part_2(input: &str, max_valid_value: u32) -> Result<u32, SimpleError> {
    let ranges = parse_input(input)?;
    let combined_ranges = combine_ranges(ranges);

    if combined_ranges[0] == IpRange::new(0, max_valid_value) {
        return Ok(0);
    }

    let mut total_allowed = combined_ranges[0].start;
    total_allowed += max_valid_value - combined_ranges.last().unwrap().end;

    for window in combined_ranges.windows(2) {
        total_allowed += window[1].start - window[0].end - 1;
    }

    Ok(total_allowed)
}

fn combine_ranges(mut ranges: Vec<IpRange>) -> Vec<IpRange> {
    ranges.sort_by_key(|range| range.start);

    let mut combined_ranges = Vec::new();
    let mut prev_range = ranges[0];
    for &range in ranges.iter().skip(1) {
        match prev_range.combine(&range) {
            Some(combined) => {
                prev_range = combined;
            },
            None => {
                combined_ranges.push(prev_range);
                prev_range = range;
            }
        }
    }
    combined_ranges.push(prev_range);

    combined_ranges
}

fn parse_input(input: &str) -> Result<Vec<IpRange>, SimpleError> {
    let ranges: Result<Vec<_>, _> = input.lines().map(|line| {
        let (l, r) = line.split_once('-').ok_or_else(
            || SimpleError::new(format!("invalid line format: {line}"))
        )?;

        Ok(IpRange::new(l.parse()?, r.parse()?))
    })
        .collect();

    if let Ok(ranges) = &ranges {
        if ranges.is_empty() {
            return Err(SimpleError::new(String::from("input has no lines")));
        }
    }

    ranges
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input, u32::MAX)?;
    let solution2 = solve_part_2(input, u32::MAX)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "5-8\n0-2\n4-7";

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(3), solve_part_1(SAMPLE_INPUT, 9));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(2), solve_part_2(SAMPLE_INPUT, 9));
    }
}
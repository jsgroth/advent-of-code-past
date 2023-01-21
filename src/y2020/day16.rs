//! Day 16: Ticket Translation
//! https://adventofcode.com/2020/day/16

use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, value: u32) -> bool {
        value >= self.start && value <= self.end
    }
}

impl FromStr for Range {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or_else(
            || SimpleError::new(format!("range string has no '-': {s}"))
        )?;

        let start = start.parse()?;
        let end = end.parse()?;

        Ok(Self { start, end })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TicketField {
    name: String,
    range_a: Range,
    range_b: Range,
}

impl TicketField {
    fn can_contain(&self, value: u32) -> bool {
        self.range_a.contains(value) || self.range_b.contains(value)
    }
}

impl FromStr for TicketField {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, ranges) = s.split_once(": ").ok_or_else(
            || SimpleError::new(format!("no ': ' in ticket field string: {s}"))
        )?;

        let name = String::from(name);

        let (range_a, range_b) = ranges.split_once(" or ").ok_or_else(
            || SimpleError::new(format!("no ' or ' in ticket field string: {s}"))
        )?;

        let range_a = range_a.parse()?;
        let range_b = range_b.parse()?;

        Ok(Self { name, range_a, range_b })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    ticket_fields: Vec<TicketField>,
    your_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let Input { ticket_fields, nearby_tickets, .. } = parse_input(input)?;

    let invalid_value_sum = nearby_tickets.iter()
        .flat_map(|ticket| {
            ticket.iter().copied()
                .filter(|&value| {
                    !ticket_fields.iter().any(|field| field.can_contain(value))
                })
                .collect::<Vec<_>>()
        })
        .sum();

    Ok(invalid_value_sum)
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let Input { ticket_fields, your_ticket, nearby_tickets } = parse_input(input)?;

    let valid_nearby_tickets: Vec<_> = nearby_tickets.into_iter()
        .filter(|ticket| {
            ticket.iter().all(|&value| {
                ticket_fields.iter().any(|field| field.can_contain(value))
            })
        })
        .collect();

    let mut name_to_index = HashMap::new();
    let mut matched_to_field = vec![false; ticket_fields.len()];
    while name_to_index.len() < ticket_fields.len() {
        let remaining_fields: Vec<_> = ticket_fields.iter()
            .filter(|&field| !name_to_index.contains_key(&field.name))
            .cloned()
            .collect();

        let mut possible_fields = vec![remaining_fields.clone(); ticket_fields.len()];
        for i in 0..ticket_fields.len() {
            if matched_to_field[i] {
                continue;
            }

            for nearby_ticket in &valid_nearby_tickets {
                possible_fields[i].retain(|field| field.can_contain(nearby_ticket[i]));
            }

            if possible_fields[i].len() == 1 {
                name_to_index.insert(possible_fields[i][0].name.clone(), i);
                matched_to_field[i] = true;
                break;
            }
        }
    }

    let departure_value_product = name_to_index.into_iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, index)| your_ticket[index] as u64)
        .product();

    Ok(departure_value_product)
}

fn parse_input(input: &str) -> Result<Input, SimpleError> {
    let lines: Vec<_> = input.lines().collect();

    let split: Vec<_> = lines.split(|s| s.is_empty()).collect();
    if split.len() != 3 {
        return Err(SimpleError::new(String::from("there should be 2 blank lines in input")));
    }

    let ticket_fields: Vec<_> = split[0].iter()
        .map(|&line| TicketField::from_str(line))
        .collect::<Result<_, _>>()?;

    if split[1].len() != 2 {
        return Err(SimpleError::new(String::from("'your ticket' section should have 2 lines")));
    }

    let your_ticket = parse_ticket(split[1][1])?;

    let nearby_tickets: Vec<_> = split[2][1..].iter()
        .map(|&line| parse_ticket(line))
        .collect::<Result<_, _>>()?;

    Ok(Input { ticket_fields, your_ticket, nearby_tickets })
}

fn parse_ticket(line: &str) -> Result<Vec<u32>, ParseIntError> {
    line.split(',')
        .map(|s| s.parse::<u32>())
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample16.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(71), solve_part_1(SAMPLE_INPUT));
    }
}
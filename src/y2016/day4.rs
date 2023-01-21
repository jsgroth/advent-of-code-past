//! Day 4: Security Through Obscurity
//! https://adventofcode.com/2016/day/4

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug)]
struct RoomData {
    name: Vec<Vec<char>>,
    sector_id: u32,
    checksum: Vec<char>,
}

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let rooms = parse_input(input)?;

    let valid = rooms.iter().filter_map(|room| {
        let mut char_counts: HashMap<char, usize> = HashMap::new();

        for c in room.name.iter().flatten().copied() {
            if let Some(count) = char_counts.get_mut(&c) {
                *count += 1;
            } else {
                char_counts.insert(c, 1);
            }
        }

        let mut char_counts: Vec<_> = char_counts.into_iter().collect();
        char_counts.sort_by(|&(a_ch, a_count), &(b_ch, b_count)| {
            a_count.cmp(&b_count).reverse()
                .then(a_ch.cmp(&b_ch))
        });

        let computed_checksum: Vec<_> = char_counts[..5].iter().map(|&(c, _)| c).collect();
        if room.checksum == computed_checksum {
            Some(room.sector_id)
        } else {
            None
        }
    })
        .sum();

    Ok(valid)
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let rooms = parse_input(input)?;

    for room in &rooms {
        let decrypted_name: Vec<String> = room.name.iter().map(|word| {
            word.iter().copied().map(|c| {
                let ord = (((c as u32) - ('a' as u32) + room.sector_id) % 26) + ('a' as u32);
                char::from_u32(ord).unwrap()
            })
                .collect()
        })
            .collect();

        let decrypted_name = decrypted_name.join(" ");
        if decrypted_name.contains("north") && decrypted_name.contains("pole") {
            return Ok(format!("{} - {decrypted_name}", room.sector_id));
        }
    }

    Err(SimpleError::new(String::from("no rooms found containing 'north' and 'pole'")))
}

fn parse_input(input: &str) -> Result<Vec<RoomData>, SimpleError> {
    input.lines().map(|line| {
        let split: Vec<_> = line.split('-').collect();
        if split.is_empty() {
            return Err(SimpleError::new(String::from("empty line in input")));
        }

        let name: Vec<Vec<_>> = split[..split.len() - 1].iter()
            .map(|word| word.chars().collect())
            .collect();

        let last_word = split.last().unwrap();
        let bracket_index = last_word.chars().position(|c| c == '[').ok_or_else(
            || SimpleError::new(format!("line has no opening bracket: {line}"))
        )?;
        let sector_id: u32 = last_word[..bracket_index].parse()?;

        let checksum: Vec<_> = last_word[bracket_index + 1..last_word.len() - 1].chars().collect();

        Ok(RoomData { name, sector_id, checksum })
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
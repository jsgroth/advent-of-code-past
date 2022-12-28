//! Day 11: Corporate Policy
//! https://adventofcode.com/2015/day/11

use std::collections::HashSet;
use std::error::Error;
use crate::SimpleError;

fn solve_part(input: &str) -> Result<String, SimpleError> {
    let line = match input.lines().next() {
        Some(line) => line,
        None => return Err(SimpleError::new(String::from("input is empty")))
    };

    if line.len() != 8 {
        return Err(SimpleError::new(format!("invalid line format: {line}")));
    }

    let mut s: Vec<_> = line.as_bytes().iter().copied().collect();
    loop {
        *s.last_mut().unwrap() += 1;

        let mut i = s.len() - 1;
        while s[i] == ('z' as u8) + 1 {
            s[i] = 'a' as u8;

            i -= 1;
            s[i] += 1;

            if s[i] == ('i' as u8) || s[i] == ('o' as u8) || s[i] == ('l' as u8) {
                s[i] += 1;
            }
        }

        if is_valid(&s) {
            break;
        }
    }

    Ok(s.into_iter().map(|c| c as char).collect())
}

fn is_valid(s: &[u8]) -> bool {
    if !s.windows(3).any(|window| {
        window[1] == window[0] + 1 && window[2] == window[1] + 1
    }) {
        return false;
    }

    if s.iter().any(|&c| c == ('i' as u8) || c == ('o' as u8) || c == ('l' as u8)) {
        return false;
    }

    s.windows(2)
        .filter(|window| window[0] == window[1])
        .collect::<HashSet<_>>()
        .len() >= 2
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part(input)?;
    let solution2 = solve_part(&solution1)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(Ok(String::from("abcdffaa")), solve_part("abcdefgh"));
        assert_eq!(Ok(String::from("ghjaabcc")), solve_part("ghijklmn"));
    }
}
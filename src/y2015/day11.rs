//! Day 11: Corporate Policy
//! https://adventofcode.com/2015/day/11

use crate::SimpleError;
use std::collections::HashSet;
use std::error::Error;

fn solve_part(input: &str) -> Result<String, SimpleError> {
    let line = crate::read_single_line(input)?;

    if line.len() != 8 {
        return Err(SimpleError::new(format!("invalid line format: {line}")));
    }

    let mut s = line.as_bytes().to_vec();
    loop {
        *s.last_mut().unwrap() += 1;

        let mut i = s.len() - 1;
        while s[i] == b'z' + 1 {
            s[i] = b'a';

            i -= 1;
            s[i] += 1;

            if s[i] == b'i' || s[i] == b'o' || s[i] == b'l' {
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
    if !s
        .windows(3)
        .any(|window| window[1] == window[0] + 1 && window[2] == window[1] + 1)
    {
        return false;
    }

    if s.iter().any(|&c| c == b'i' || c == b'o' || c == b'l') {
        return false;
    }

    s.windows(2)
        .filter(|window| window[0] == window[1])
        .collect::<HashSet<_>>()
        .len()
        >= 2
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

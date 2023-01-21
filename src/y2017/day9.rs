//! Day 9: Stream Processing
//!
//! <https://adventofcode.com/2017/day/9>

use crate::SimpleError;
use std::error::Error;
use std::iter::Peekable;

fn solve_both_parts(input: &str) -> Result<(usize, usize), SimpleError> {
    let line = crate::read_single_line(input)?;

    Ok(parse_group(&mut line.chars().peekable(), 1))
}

fn parse_group<I>(iter: &mut Peekable<I>, depth: usize) -> (usize, usize)
where
    I: Iterator<Item = char>,
{
    iter.next();

    let mut total_group_score = depth;
    let mut total_garbage_score = 0;
    while let Some(&c) = iter.peek() {
        match c {
            '{' => {
                let (group_score, garbage_score) = parse_group(iter, depth + 1);
                total_group_score += group_score;
                total_garbage_score += garbage_score;
            }
            '<' => {
                total_garbage_score += parse_garbage(iter);
            }
            '}' => {
                iter.next();
                break;
            }
            _ => {
                iter.next();
            }
        }
    }

    (total_group_score, total_garbage_score)
}

fn parse_garbage<I>(iter: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = char>,
{
    iter.next();

    let mut garbage_chars = 0;
    while let Some(c) = iter.next() {
        match c {
            '>' => {
                break;
            }
            '!' => {
                iter.next();
            }
            _ => {
                garbage_chars += 1;
            }
        }
    }

    garbage_chars
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let (solution1, solution2) = solve_both_parts(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(Ok((16, 0)), solve_both_parts("{{{},{},{{}}}}"));
        assert_eq!(
            Ok((9, 6)),
            solve_both_parts("{{<ab>},{<ab>},{<!b>},{<ab>}}")
        );
        assert_eq!(
            Ok((9, 0)),
            solve_both_parts("{{<!!>},{<!>>},{<!!>},{<!>>}}")
        )
    }
}

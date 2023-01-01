//! Day 4: Repose Record
//! https://adventofcode.com/2018/day/4

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum LogEvent {
    GuardBeginsShift(usize),
    GuardFallsAsleep,
    GuardWakesUp,
}

#[derive(Debug, Clone, Copy)]
struct LogLine {
    minute: usize,
    event: LogEvent,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let logs = parse_input(input)?;

    let minutes_asleep = compute_guard_id_to_minutes_asleep(&logs);

    let (laziest_guard_id, guard_minutes_asleep) = minutes_asleep.into_iter()
        .max_by_key(|(_, guard_minutes_asleep)| {
            guard_minutes_asleep.iter().copied().sum::<usize>()
        })
        .unwrap();

    let laziest_minute = guard_minutes_asleep.into_iter().enumerate()
        .max_by_key(|&(_, minutes_slept)| minutes_slept)
        .map(|(minute, _)| minute)
        .unwrap();

    Ok(laziest_guard_id * laziest_minute)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let logs = parse_input(input)?;

    let minutes_asleep = compute_guard_id_to_minutes_asleep(&logs);

    let (laziest_guard_id, laziest_minute) = minutes_asleep.into_iter()
        .flat_map(|(guard_id, guard_minutes_asleep)| {
            guard_minutes_asleep.into_iter().enumerate()
                .map(|(minute, count)| (guard_id, minute, count))
                .collect::<Vec<_>>()
        })
        .max_by_key(|&(_, _, count)| count)
        .map(|(guard_id, minute, _)| (guard_id, minute))
        .unwrap();

    Ok(laziest_guard_id * laziest_minute)
}

fn compute_guard_id_to_minutes_asleep(logs: &Vec<LogLine>) -> HashMap<usize, Vec<usize>> {
    let mut minutes_asleep = HashMap::new();

    let mut current_guard_id = 0;
    let mut fell_asleep_minute = 0;
    for &log in logs {
        match log.event {
            LogEvent::GuardBeginsShift(guard_id) => {
                current_guard_id = guard_id;
                if !minutes_asleep.contains_key(&guard_id) {
                    minutes_asleep.insert(guard_id, vec![0; 60]);
                }
            }
            LogEvent::GuardFallsAsleep => {
                fell_asleep_minute = log.minute;
            }
            LogEvent::GuardWakesUp => {
                let guard_minutes_asleep = minutes_asleep.get_mut(&current_guard_id).unwrap();
                for minute in fell_asleep_minute..log.minute {
                    guard_minutes_asleep[minute] += 1;
                }
            }
        }
    }

    minutes_asleep
}

fn parse_input(input: &str) -> Result<Vec<LogLine>, SimpleError> {
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort();
    lines.into_iter().map(|line| {
        if line.len() < "[1518-00-00 00:00] ".len() {
            return Err(SimpleError::new(format!("invalid log line, not long enough: {line}")));
        }

        let minute: usize = line[15..17].parse()?;

        let rest_of_line = &line["[1518-00-00 00:00] ".len()..];
        let split: Vec<_> = rest_of_line.split(' ').collect();
        let event = match split.as_slice() {
            ["Guard", id, "begins", "shift"] => {
                LogEvent::GuardBeginsShift(id[1..].parse()?)
            }
            ["falls", "asleep"] => LogEvent::GuardFallsAsleep,
            ["wakes", "up"] => LogEvent::GuardWakesUp,
            _ => return Err(SimpleError::new(format!("unknown event in line: {line}")))
        };

        Ok(LogLine { minute, event })
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample4.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(240), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(4455), solve_part_2(SAMPLE_INPUT));
    }
}
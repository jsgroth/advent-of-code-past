mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;

use std::error::Error;
use crate::SimpleError;

pub fn run_day(day: usize) -> Result<(), Box<dyn Error>> {
    match day {
        1 => crate::run_solution(day1::solve),
        2 => crate::run_solution(day2::solve),
        3 => crate::run_solution(day3::solve),
        4 => crate::run_solution(day4::solve),
        _ => Err(Box::new(SimpleError::new(format!("unexpected day: {day}"))))
    }
}
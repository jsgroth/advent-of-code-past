mod day1;

use std::error::Error;
use crate::SimpleError;

pub fn run_day(day: usize) -> Result<(), Box<dyn Error>> {
    match day {
        1 => crate::run_solution(day1::solve),
        _ => Err(Box::new(SimpleError::new(format!("unexpected day: {day}"))))
    }
}
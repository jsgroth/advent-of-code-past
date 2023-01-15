mod intcode;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::error::Error;
use crate::SimpleError;

pub fn run_day(day: usize) -> Result<(), Box<dyn Error>> {
    match day {
        1 => crate::run_solution(day1::solve),
        2 => crate::run_solution(day2::solve),
        3 => crate::run_solution(day3::solve),
        4 => crate::run_solution(day4::solve),
        5 => crate::run_solution(day5::solve),
        6 => crate::run_solution(day6::solve),
        7 => crate::run_solution(day7::solve),
        8 => crate::run_solution(day8::solve),
        9 => crate::run_solution(day9::solve),
        10 => crate::run_solution(day10::solve),
        11 => crate::run_solution(day11::solve),
        12 => crate::run_solution(day12::solve),
        13 => crate::run_solution(day13::solve),
        14 => crate::run_solution(day14::solve),
        15 => crate::run_solution(day15::solve),
        16 => crate::run_solution(day16::solve),
        17 => crate::run_solution(day17::solve),
        18 => crate::run_solution(day18::solve),
        19 => crate::run_solution(day19::solve),
        20 => crate::run_solution(day20::solve),
        21 => crate::run_solution(day21::solve),
        22 => crate::run_solution(day22::solve),
        23 => crate::run_solution(day23::solve),
        24 => crate::run_solution(day24::solve),
        25 => day25::run(),
        _ => Err(Box::new(SimpleError::new(format!("unexpected day: {day}"))))
    }
}
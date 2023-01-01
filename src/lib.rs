#![forbid(unsafe_code)]

pub mod simpleerror;

pub mod y2015;
pub mod y2016;
pub mod y2017;
pub mod y2018;

use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Read;

pub use simpleerror::SimpleError;

fn run_solution<T1, T2, F>(solution: F) -> Result<(), Box<dyn Error>>
where
    T1: Display,
    T2: Display,
    F: Fn(&str) -> Result<(T1, T2), Box<dyn Error>>,
{
    let input = read_input()?;

    let (solution1, solution2) = solution(&input)?;
    println!("{solution1}");
    println!("{solution2}");

    Ok(())
}

// Read input from stdin
fn read_input() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    Ok(s)
}

fn read_single_line(input: &str) -> Result<&str, SimpleError> {
    match input.lines().next() {
        Some(line) => Ok(line),
        None => Err(SimpleError::new(String::from("input is empty, expected a single line"))),
    }
}
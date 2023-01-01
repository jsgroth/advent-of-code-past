use std::env;
use std::error::Error;
use advent_of_code_past::{y2015, y2016, y2017, y2018};

const USAGE: &str = "ARGS: year day\ninput should be passed via stdin";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let year = args.next().expect(USAGE);
    let day = args.next().expect(USAGE);
    let day: usize = day.parse().expect("day should be a non-negative integer");

    match year.as_str() {
        "2015" => y2015::run_day(day),
        "2016" => y2016::run_day(day),
        "2017" => y2017::run_day(day),
        "2018" => y2018::run_day(day),
        _ => panic!("unexpected year: {year}")
    }
}
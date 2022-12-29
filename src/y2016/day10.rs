//! Day 10: Balance Bots
//! https://adventofcode.com/2016/day/10

use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug, Clone, Copy)]
enum BotTarget {
    Bot(usize),
    Output(usize),
}

impl BotTarget {
    fn from_strs(type_str: &str, target: &str) -> Result<Self, SimpleError> {
        let target: usize = target.parse()?;
        match type_str {
            "bot" => Ok(Self::Bot(target)),
            "output" => Ok(Self::Output(target)),
            _ => Err(SimpleError::new(format!("invalid target string: {type_str} {target}")))
        }
    }
}

#[derive(Debug)]
struct Bot {
    number: usize,
    low_target: BotTarget,
    high_target: BotTarget,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let (bots, starting_values) = parse_input(input)?;

    let (values, _) = simulate_bots(bots, starting_values);
    values.into_iter()
        .find_map(|(bot_number, bot_values)| {
            if bot_values == vec![17, 61] || bot_values == vec![61, 17] {
                Some(bot_number)
            } else {
                None
            }
        })
        .ok_or(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let (bots, starting_values) = parse_input(input)?;

    let (_, outputs) = simulate_bots(bots, starting_values);

    for output_number in [0, 1, 2] {
        if !outputs.contains_key(&output_number) {
            return Err(SimpleError::new(format!("output is missing number {output_number}")));
        }
    }

    Ok(outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap())
}

fn simulate_bots(bots: Vec<Bot>, starting_values: Vec<(u32, usize)>) -> (HashMap<usize, Vec<u32>>, HashMap<usize, u32>) {
    let number_to_bot: HashMap<_, _> = bots.iter()
        .map(|bot| (bot.number, bot))
        .collect();

    let mut number_to_values: HashMap<usize, Vec<u32>> = HashMap::new();
    for &number in number_to_bot.keys() {
        number_to_values.insert(number, Vec::new());
    }

    let mut bots_to_check: Vec<usize> = Vec::new();
    for &(value, bot_number) in &starting_values {
        let bot_values = number_to_values.get_mut(&bot_number).unwrap();
        bot_values.push(value);
        if bot_values.len() == 2 {
            bots_to_check.push(bot_number);
        }
    }

    let mut outputs: HashMap<usize, u32> = HashMap::new();
    for &bot_number in &bots_to_check {
        let &bot = number_to_bot.get(&bot_number).unwrap();
        check_bot(bot, &number_to_bot, &mut number_to_values, &mut outputs);
    }

    (number_to_values, outputs)
}

fn check_bot(bot: &Bot, bots: &HashMap<usize, &Bot>, values: &mut HashMap<usize, Vec<u32>>, outputs: &mut HashMap<usize, u32>) {
    let bot_values = values.get_mut(&bot.number).unwrap();

    if bot_values.len() != 2 {
        return;
    }

    let low_value = cmp::min(bot_values[0], bot_values[1]);
    let high_value = cmp::max(bot_values[0], bot_values[1]);

    match bot.low_target {
        BotTarget::Bot(number) => {
            values.get_mut(&number).unwrap().push(low_value);
        }
        BotTarget::Output(number) => {
            outputs.insert(number, low_value);
        }
    }

    match bot.high_target {
        BotTarget::Bot(number) => {
            values.get_mut(&number).unwrap().push(high_value);
        }
        BotTarget::Output(number) => {
            outputs.insert(number, high_value);
        }
    }

    if let BotTarget::Bot(number) = bot.low_target {
        let &low_bot = bots.get(&number).unwrap();
        check_bot(low_bot, bots, values, outputs);
    }

    if let BotTarget::Bot(number) = bot.high_target {
        let high_bot = bots.get(&number).unwrap();
        check_bot(high_bot, bots, values, outputs);
    }
}

fn parse_input(input: &str) -> Result<(Vec<Bot>, Vec<(u32, usize)>), SimpleError> {
    let mut bots: Vec<Bot> = Vec::new();
    let mut starting_values: Vec<(u32, usize)> = Vec::new();

    for line in input.lines() {
        let split: Vec<_> = line.split(' ').collect();
        match split.as_slice() {
            ["value", n, "goes", "to", "bot", bot] => {
                let n: u32 = n.parse()?;
                let bot: usize = bot.parse()?;
                starting_values.push((n, bot));
            }
            ["bot", bot, "gives", "low", "to", low_type, low_target, "and", "high", "to", high_type, high_target] => {
                let number: usize = bot.parse()?;
                let low_target = BotTarget::from_strs(*low_type, *low_target)?;
                let high_target = BotTarget::from_strs(*high_type, *high_target)?;
                bots.push(Bot { number, low_target, high_target });
            }
            _ => return Err(SimpleError::new(format!("invalid line format: {line}")))
        }
    }

    Ok((bots, starting_values))
}

pub fn solve(input: &str) -> Result<(usize, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
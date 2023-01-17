//! Day 18: Operation Order
//! https://adventofcode.com/2020/day/18

use std::error::Error;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn evaluate(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let expression_sum = input.lines()
        .map(|line| evaluate_no_precedence(line))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(expression_sum)
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let expression_sum = input.lines()
        .map(|line| evaluate_add_first(line))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(expression_sum)
}

fn evaluate_no_precedence(expression: &str) -> Result<u64, SimpleError> {
    // Insert spaces to make parsing much easier
    let expression = expression.replace("(", "( ").replace(")", " )");

    let mut current_value = 0;
    let mut last_operator = Operator::Add;

    let mut levels = Vec::new();

    for token in expression.split(' ') {
        match token {
            "+" => {
                last_operator = Operator::Add;
            }
            "*" => {
                last_operator = Operator::Multiply;
            }
            "(" => {
                levels.push((current_value, last_operator));

                current_value = 0;
                last_operator = Operator::Add;
            }
            ")" => {
                let (prev_level_value, prev_level_operator) = levels.pop().ok_or(
                    SimpleError::new(format!("expression has unbalanced parentheses: {expression}"))
                )?;

                current_value = prev_level_operator.evaluate(prev_level_value, current_value);
                last_operator = prev_level_operator;
            }
            _ => {
                let n = token.parse()?;
                current_value = last_operator.evaluate(current_value, n);
            }
        }
    }

    if !levels.is_empty() {
        return Err(SimpleError::new(format!("expression has unbalanced parentheses: {expression}")));
    }

    Ok(current_value)
}

fn evaluate_add_first(expression: &str) -> Result<u64, SimpleError> {
    // Insert spaces to make parsing much easier
    let expression = expression.replace("(", "( ").replace(")", " )");

    let mut operands = Vec::new();
    let mut last_operator = Operator::Multiply;

    let mut levels = Vec::new();

    for token in expression.split(' ') {
        match token {
            "+" => {
                last_operator = Operator::Add;
            }
            "*" => {
                last_operator = Operator::Multiply;
            }
            "(" => {
                levels.push((operands, last_operator));

                operands = Vec::new();
                last_operator = Operator::Multiply;
            }
            ")" => {
                let level_product = operands.into_iter().product();

                let (prev_level_operands, prev_level_operator) = levels.pop().ok_or(
                    SimpleError::new(format!("expression has unbalanced parentheses: {expression}"))
                )?;

                operands = prev_level_operands;
                last_operator = prev_level_operator;

                match prev_level_operator {
                    Operator::Add => {
                        if operands.is_empty() {
                            return Err(SimpleError::new(format!("no value/expression before '+': {expression}")));
                        }

                        let value = operands.pop().unwrap() + level_product;
                        operands.push(value);
                    }
                    Operator::Multiply => {
                        operands.push(level_product);
                    }
                }
            }
            _ => {
                let n: u64 = token.parse()?;

                match last_operator {
                    Operator::Add => {
                        if operands.is_empty() {
                            return Err(SimpleError::new(format!("'+' preceded by no value: {expression}")));
                        }

                        let value = operands.pop().unwrap() + n;
                        operands.push(value);
                    }
                    Operator::Multiply => {
                        operands.push(n);
                    }
                }
            }
        }
    }

    if !levels.is_empty() {
        return Err(SimpleError::new(format!("expression has unbalanced parentheses: {expression}")));
    }

    if operands.is_empty() {
        return Err(SimpleError::new(format!("expression evaluated to empty: {expression}")));
    }

    Ok(operands.into_iter().product())
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(71), evaluate_no_precedence("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(Ok(51), evaluate_no_precedence("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(Ok(26), evaluate_no_precedence("2 * 3 + (4 * 5)"));
        assert_eq!(Ok(437), evaluate_no_precedence("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(Ok(12240), evaluate_no_precedence("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(Ok(13632), evaluate_no_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(231), evaluate_add_first("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(Ok(51), evaluate_add_first("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(Ok(46), evaluate_add_first("2 * 3 + (4 * 5)"));
        assert_eq!(Ok(1445), evaluate_add_first("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(Ok(669060), evaluate_add_first("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(Ok(23340), evaluate_add_first("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}
//! Day 19: Monster Messages
//! https://adventofcode.com/2020/day/19

use std::error::Error;
use std::iter;
use crate::SimpleError;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    index: usize,
    branches: Vec<Vec<usize>>,
}

impl Rule {
    fn empty() -> Self {
        Self { index: usize::MAX, branches: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    rules: Vec<Rule>,
    a_rule_index: usize,
    b_rule_index: usize,
    messages: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct StateStep {
    rule_index: usize,
    branch: usize,
    position: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    steps: Vec<StateStep>,
    expected_next: Option<char>,
}

fn solve_part(input: &str, replace_8_11_rules: bool) -> Result<usize, SimpleError> {
    let Input { rules, a_rule_index, b_rule_index, messages } = parse_input(input)?;

    let rules = sort_rules(&rules);

    let rules = if replace_8_11_rules {
        let mut rules = rules;
        replace_rules_for_part_two(&mut rules);
        rules
    } else {
        rules
    };

    let initial_states = generate_states_for_step(
        StateStep { rule_index: 0, branch: 0, position: 0 },
        &Vec::new(),
        &rules,
        a_rule_index,
        b_rule_index,
    );

    let mut valid_count = 0;
    for message in &messages {
        let mut states = initial_states.clone();

        for c in message.chars() {
            let (a_states, b_states): (Vec<_>, Vec<_>) = states.into_iter()
                .filter(|state| state.expected_next.is_some())
                .partition(|state| state.expected_next == Some('a'));

            let matching_states = if c == 'a' { a_states } else { b_states };

            states = matching_states.into_iter()
                .flat_map(|state| advance_state(state, &rules, a_rule_index, b_rule_index))
                .collect();
        }

        if states.into_iter().any(|state| state.expected_next.is_none()) {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn replace_rules_for_part_two(rules: &mut Vec<Rule>) {
    rules[8] = Rule {
        index: 8,
        branches: vec![
            vec![42],
            vec![42, 8],
        ],
    };
    rules[11] = Rule {
        index: 11,
        branches: vec![
            vec![42, 31],
            vec![42, 11, 31],
        ],
    };
}

fn advance_state(state: State, rules: &[Rule], a_rule_index: usize, b_rule_index: usize) -> Vec<State> {
    let State { mut steps, .. } = state;

    while let Some(last_step) = steps.pop() {
        let rule = &rules[last_step.rule_index];
        let branch = &rule.branches[last_step.branch];

        if last_step.position < branch.len() - 1 {
            let new_last_step = StateStep {
                position: last_step.position + 1,
                ..last_step
            };
            return generate_states_for_step(new_last_step, &steps, rules, a_rule_index, b_rule_index);
        }
    }

    vec![State { steps: Vec::new(), expected_next: None }]
}

fn generate_states_for_step(step: StateStep, path_so_far: &[StateStep], rules: &[Rule], a_rule_index: usize, b_rule_index: usize) -> Vec<State> {
    let next_rule_index = rules[step.rule_index].branches[step.branch][step.position];

    let new_path_so_far: Vec<_> = path_so_far.iter().copied().chain(iter::once(step)).collect();

    if next_rule_index == a_rule_index {
        vec![State { steps: new_path_so_far, expected_next: Some('a') }]
    } else if next_rule_index == b_rule_index {
        vec![State { steps: new_path_so_far, expected_next: Some('b') }]
    } else {
        let mut states = Vec::new();

        let rule = &rules[next_rule_index];
        for branch_index in 0..rule.branches.len() {
            let branch_step = StateStep { rule_index: next_rule_index, branch: branch_index, position: 0 };
            states.extend(generate_states_for_step(branch_step, &new_path_so_far, rules, a_rule_index, b_rule_index));
        }

        states
    }
}

fn sort_rules(rules: &[Rule]) -> Vec<Rule> {
    let max_rule_index = rules.iter().map(|rule| rule.index).max().unwrap();

    let mut sorted_rules = vec![Rule::empty(); max_rule_index + 1];
    for rule in rules {
        sorted_rules[rule.index] = rule.clone();
    }

    sorted_rules
}

fn parse_input(input: &str) -> Result<Input, SimpleError> {
    let mut lines = input.lines();

    let rule_lines: Vec<_> = lines.by_ref().take_while(|s| !s.is_empty()).collect();
    let messages: Vec<_> = lines.map(String::from).collect();

    let mut rules = Vec::new();
    let mut a_rule_index: Option<usize> = None;
    let mut b_rule_index: Option<usize> = None;
    for rule_line in &rule_lines {
        let (index, branches) = rule_line.split_once(": ").ok_or(
            SimpleError::new(format!("line did not contain a ': ': {rule_line}"))
        )?;

        let index = index.parse()?;

        match branches {
            "\"a\"" => {
                a_rule_index = Some(index);
            }
            "\"b\"" => {
                b_rule_index = Some(index);
            }
            _ => {
                let branches = branches.split(" | ")
                    .map(|branch| {
                        branch.split(' ').map(|s| s.parse::<usize>()).collect()
                    })
                    .collect::<Result<_, _>>()?;

                rules.push(Rule { index, branches });
            }
        }
    }

    if a_rule_index.is_none() || b_rule_index.is_none() {
        return Err(SimpleError::new(String::from("input did not contain both \"a\" and \"b\" rules")));
    }

    Ok(Input {
        rules,
        a_rule_index: a_rule_index.unwrap(),
        b_rule_index: b_rule_index.unwrap(),
        messages,
    })
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample19.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample19-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(2), solve_part(SAMPLE_INPUT, false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(12), solve_part(SAMPLE_INPUT_2, true));
    }
}
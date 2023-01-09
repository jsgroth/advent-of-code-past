use std::error::Error;

const ADD_OPCODE: i64 = 1;
const MULTIPLY_OPCODE: i64 = 2;
const INPUT_OPCODE: i64 = 3;
const OUTPUT_OPCODE: i64 = 4;
const JUMP_IF_TRUE_OPCODE: i64 = 5;
const JUMP_IF_FALSE_OPCODE: i64 = 6;
const LESS_THAN_OPCODE: i64 = 7;
const EQUAL_OPCODE: i64 = 8;
const HALT_OPCODE: i64 = 99;

const POSITION_MODE: i64 = 0;
const IMMEDIATE_MODE: i64 = 1;

pub fn execute(
    program: &mut Vec<i64>,
    mut input_fn: impl FnMut() -> i64,
    mut output_fn: impl FnMut(i64) -> (),
) {
    let mut ip = 0;
    while ip < program.len() {
        let parameter_modes = program[ip] / 100;
        let opcode = program[ip] % 100;
        match opcode {
            ADD_OPCODE => {
                let a = read_value(program, ip + 1, parameter_modes);
                let b = read_value(program, ip + 2, parameter_modes / 10);
                let c = program[ip + 3] as usize;
                program[c] = a + b;

                ip += 4;
            }
            MULTIPLY_OPCODE => {
                let a = read_value(program, ip + 1, parameter_modes);
                let b = read_value(program, ip + 2, parameter_modes / 10);
                let c = program[ip + 3] as usize;
                program[c] = a * b;

                ip += 4;
            }
            INPUT_OPCODE => {
                let input = input_fn();
                let a = program[ip + 1] as usize;
                program[a] = input;

                ip += 2;
            }
            OUTPUT_OPCODE => {
                let a = read_value(program, ip + 1, parameter_modes);
                output_fn(a);

                ip += 2;
            }
            JUMP_IF_TRUE_OPCODE => {
                if read_value(program, ip + 1, parameter_modes) != 0 {
                    ip = read_value(program, ip + 2, parameter_modes / 10) as usize;
                } else {
                    ip += 3;
                }
            }
            JUMP_IF_FALSE_OPCODE => {
                if read_value(program, ip + 1, parameter_modes) == 0 {
                    ip = read_value(program, ip + 2, parameter_modes / 10) as usize;
                } else {
                    ip += 3;
                }
            }
            LESS_THAN_OPCODE => {
                let a = read_value(program, ip + 1, parameter_modes);
                let b = read_value(program, ip + 2, parameter_modes / 10);
                let c = program[ip + 3] as usize;
                program[c] = if a < b { 1 } else { 0 };

                ip += 4;
            }
            EQUAL_OPCODE => {
                let a = read_value(program, ip + 1, parameter_modes);
                let b = read_value(program, ip + 2, parameter_modes / 10);
                let c = program[ip + 3] as usize;
                program[c] = if a == b { 1 } else { 0 };

                ip += 4;
            }
            HALT_OPCODE => {
                break;
            }
            _ => panic!("invalid opcode: {}", program[ip])
        }
    }
}

pub fn execute_no_io(program: &mut Vec<i64>) {
    execute(
        program,
        || panic!("did not expect input fn to get called"),
        |_| panic!("did not expect output fn to get called"),
    );
}

pub fn parse_program(input: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let result: Result<Vec<_>, _> = crate::read_single_line(input)?
        .split(',')
        .map(|n| n.parse::<i64>())
        .collect();
    Ok(result?)
}

fn read_value(program: &[i64], index: usize, parameter_mode: i64) -> i64 {
    match parameter_mode % 10 {
        POSITION_MODE => program[program[index] as usize],
        IMMEDIATE_MODE => program[index],
        _ => panic!("unexpected parameter mode: {}", parameter_mode % 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn execute_with_input(program: &Vec<i64>, input: impl FnMut() -> i64) -> Vec<i64> {
        let mut program = program.clone();
        let mut outputs = Vec::new();
        execute(&mut program, input, |output| outputs.push(output));
        outputs
    }

    #[test]
    fn test_add() {
        let mut program = vec![1, 0, 0, 0, 99];
        execute_no_io(&mut program);
        assert_eq!(vec![2, 0, 0, 0, 99], program);
    }

    #[test]
    fn test_multiply() {
        let mut program = vec![2, 3, 0, 3, 99];
        execute_no_io(&mut program);
        assert_eq!(vec![2, 3, 0, 6, 99], program);
    }

    #[test]
    fn test_parameter_modes() {
        let mut program = vec![1002, 4, 3, 4, 33];
        execute_no_io(&mut program);
        assert_eq!(vec![1002, 4, 3, 4, 99], program);
    }

    #[test]
    fn test_input() {
        let mut program = vec![3, 3, 99, 0];
        execute(&mut program, || 5, |_| panic!("output should not be called"));
        assert_eq!(vec![3, 3, 99, 5], program);
    }

    #[test]
    fn test_output() {
        let mut program = vec![4, 3, 99, 200];
        let mut outputs = Vec::new();
        execute(
            &mut program,
            || panic!("input should not be called"),
            |output| outputs.push(output),
        );
        assert_eq!(vec![200], outputs);
        assert_eq!(vec![4, 3, 99, 200], program);
    }

    #[test]
    fn test_equal_position() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let outputs = execute_with_input(&program, || 8);
        assert_eq!(vec![1], outputs);

        let outputs = execute_with_input(&program, || 9);
        assert_eq!(vec![0], outputs);
    }

    #[test]
    fn test_less_than_position() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let outputs = execute_with_input(&program, || 7);
        assert_eq!(vec![1], outputs);

        let outputs = execute_with_input(&program, || 8);
        assert_eq!(vec![0], outputs);

        let outputs = execute_with_input(&program, || 9);
        assert_eq!(vec![0], outputs);
    }

    #[test]
    fn test_equal_immediate() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let outputs = execute_with_input(&program, || 8);
        assert_eq!(vec![1], outputs);

        let outputs = execute_with_input(&program, || 9);
        assert_eq!(vec![0], outputs);
    }

    #[test]
    fn test_less_than_immediate() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let outputs = execute_with_input(&program, || 7);
        assert_eq!(vec![1], outputs);

        let outputs = execute_with_input(&program, || 8);
        assert_eq!(vec![0], outputs);

        let outputs = execute_with_input(&program, || 9);
        assert_eq!(vec![0], outputs);
    }

    #[test]
    fn test_jump_position() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];

        let outputs = execute_with_input(&program, || 0);
        assert_eq!(vec![0], outputs);

        let outputs = execute_with_input(&program, || 55);
        assert_eq!(vec![1], outputs);
    }

    #[test]
    fn test_jump_immediate() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let outputs = execute_with_input(&program, || 0);
        assert_eq!(vec![0], outputs);

        let outputs = execute_with_input(&program, || 55);
        assert_eq!(vec![1], outputs);
    }

    #[test]
    fn test_day2_sample_input_1() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        execute_no_io(&mut program);
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], program);
    }

    #[test]
    fn test_day2_sample_input_2() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        execute_no_io(&mut program);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], program);
    }

    #[test]
    fn test_day2_sample_input_3() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute_no_io(&mut program);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], program);
    }

    #[test]
    fn test_day5_sample_input() {
        let program = vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,
            20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99,
        ];

        let outputs = execute_with_input(&program, || 5);
        assert_eq!(vec![999], outputs);

        let outputs = execute_with_input(&program, || 8);
        assert_eq!(vec![1000], outputs);

        let outputs = execute_with_input(&program, || 9);
        assert_eq!(vec![1001], outputs);
    }
}
use std::collections::VecDeque;
use std::error::Error;
use std::iter;

const ADD_OPCODE: i64 = 1;
const MULTIPLY_OPCODE: i64 = 2;
const INPUT_OPCODE: i64 = 3;
const OUTPUT_OPCODE: i64 = 4;
const JUMP_IF_TRUE_OPCODE: i64 = 5;
const JUMP_IF_FALSE_OPCODE: i64 = 6;
const LESS_THAN_OPCODE: i64 = 7;
const EQUAL_OPCODE: i64 = 8;
const ADJUST_RELATIVE_BASE_OPCODE: i64 = 9;
const HALT_OPCODE: i64 = 99;

const POSITION_MODE: i64 = 0;
const IMMEDIATE_MODE: i64 = 1;
const RELATIVE_MODE: i64 = 2;

pub trait InputFn {
    fn call(&mut self) -> Option<i64>;
}

pub trait OutputFn {
    fn call(&mut self, output: i64);
}

impl<T: FnMut() -> i64> InputFn for T {
    fn call(&mut self) -> Option<i64> {
        Some(self())
    }
}

impl<T: FnMut(i64) -> ()> OutputFn for T {
    fn call(&mut self, output: i64) {
        self(output)
    }
}

#[derive(Debug, Clone)]
pub struct IntcodeProgram<I: InputFn, O: OutputFn> {
    program: Vec<i64>,
    ip: usize,
    input_fn: I,
    output_fn: O,
    relative_base: i64,
}

impl<I: InputFn, O: OutputFn> IntcodeProgram<I, O> {
    pub fn new(program: Vec<i64>, input_fn: I, output_fn: O) -> Self {
        Self {
            program,
            ip: 0,
            input_fn,
            output_fn,
            relative_base: 0,
        }
    }

    fn read_value(&self, index: usize, parameter_mode: i64) -> i64 {
        let address = match parameter_mode % 10 {
            POSITION_MODE => self.program[index] as usize,
            IMMEDIATE_MODE => index,
            RELATIVE_MODE => (self.relative_base + self.program[index]) as usize,
            _ => panic!("unexpected parameter mode: {}", parameter_mode % 10)
        };

        self.program.get(address).copied().unwrap_or(0)
    }

    fn write_value(&mut self, index: usize, value: i64, parameter_mode: i64) {
        let address = match parameter_mode % 10 {
            POSITION_MODE => self.program[index] as usize,
            IMMEDIATE_MODE => panic!("immediate parameter mode not supported for writes"),
            RELATIVE_MODE => (self.relative_base + self.program[index]) as usize,
            _ => panic!("unexpected parameter mode: {}", parameter_mode % 10)
        };

        if address >= self.program.len() {
            let expansion_size = address - self.program.len() + 1;
            self.program.extend(iter::repeat(0).take(expansion_size));
        }

        self.program[address] = value;
    }

    pub fn execute(&mut self) -> bool {
        while self.ip < self.program.len() {
            let parameter_modes = self.program[self.ip] / 100;
            let opcode = self.program[self.ip] % 100;
            match opcode {
                ADD_OPCODE => {
                    let a = self.read_value(self.ip + 1, parameter_modes);
                    let b = self.read_value(self.ip + 2, parameter_modes / 10);

                    self.write_value(self.ip + 3, a + b, parameter_modes / 100);

                    self.ip += 4;
                }
                MULTIPLY_OPCODE => {
                    let a = self.read_value(self.ip + 1, parameter_modes);
                    let b = self.read_value(self.ip + 2, parameter_modes / 10);

                    self.write_value(self.ip + 3, a * b, parameter_modes / 100);

                    self.ip += 4;
                }
                INPUT_OPCODE => {
                    let input = self.input_fn.call();
                    if input.is_none() {
                        return false;
                    }

                    self.write_value(self.ip + 1, input.unwrap(), parameter_modes);

                    self.ip += 2;
                }
                OUTPUT_OPCODE => {
                    let a = self.read_value(self.ip + 1, parameter_modes);
                    self.output_fn.call(a);

                    self.ip += 2;
                }
                JUMP_IF_TRUE_OPCODE => {
                    if self.read_value(self.ip + 1, parameter_modes) != 0 {
                        self.ip = self.read_value(self.ip + 2, parameter_modes / 10) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                JUMP_IF_FALSE_OPCODE => {
                    if self.read_value(self.ip + 1, parameter_modes) == 0 {
                        self.ip = self.read_value(self.ip + 2, parameter_modes / 10) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                LESS_THAN_OPCODE => {
                    let a = self.read_value(self.ip + 1, parameter_modes);
                    let b = self.read_value(self.ip + 2, parameter_modes / 10);

                    let c = if a < b { 1 } else { 0 };
                    self.write_value(self.ip + 3, c, parameter_modes / 100);

                    self.ip += 4;
                }
                EQUAL_OPCODE => {
                    let a = self.read_value(self.ip + 1, parameter_modes);
                    let b = self.read_value(self.ip + 2, parameter_modes / 10);

                    let c = if a == b { 1 } else { 0 };
                    self.write_value(self.ip + 3, c, parameter_modes / 100);

                    self.ip += 4;
                }
                ADJUST_RELATIVE_BASE_OPCODE => {
                    let a = self.read_value(self.ip + 1, parameter_modes);

                    self.relative_base += a;

                    self.ip += 2;
                }
                HALT_OPCODE => {
                    return true;
                }
                _ => panic!("invalid opcode: {opcode}")
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
struct InteractiveIntcodeInputFn {
    inputs: VecDeque<i64>,
}

#[derive(Debug, Clone)]
struct InteractiveIntcodeOutputFn {
    outputs: VecDeque<i64>,
}

impl InputFn for InteractiveIntcodeInputFn {
    fn call(&mut self) -> Option<i64> {
        self.inputs.pop_front()
    }
}

impl OutputFn for InteractiveIntcodeOutputFn {
    fn call(&mut self, output: i64) {
        self.outputs.push_back(output);
    }
}

#[derive(Debug, Clone)]
pub struct InteractiveIntcodeProgram {
    program: IntcodeProgram<InteractiveIntcodeInputFn, InteractiveIntcodeOutputFn>,
}

impl InteractiveIntcodeProgram {
    pub fn new(program: Vec<i64>) -> Self {
        let input_fn = InteractiveIntcodeInputFn {
            inputs: VecDeque::new(),
        };
        let output_fn = InteractiveIntcodeOutputFn {
            outputs: VecDeque::new(),
        };

        let program = IntcodeProgram::new(program, input_fn, output_fn);

        Self { program }
    }

    pub fn push_input(&mut self, input: i64) {
        self.program.input_fn.inputs.push_back(input);
    }

    pub fn fetch_outputs(&mut self) -> Vec<i64> {
        let mut outputs = Vec::with_capacity(self.program.output_fn.outputs.len());
        outputs.extend(&self.program.output_fn.outputs);

        self.program.output_fn.outputs.clear();

        outputs
    }

    pub fn execute(&mut self) -> bool {
        self.program.execute()
    }
}

pub fn execute(
    program: &mut Vec<i64>,
    input_fn: impl InputFn,
    output_fn: impl OutputFn,
) {
    let mut intcode_program = IntcodeProgram::new(program.clone(), input_fn, output_fn);
    intcode_program.execute();
    *program = intcode_program.program;
}

pub fn execute_no_io(program: &mut Vec<i64>) {
    execute(
        program,
        || panic!("did not expect input fn to get called"),
        |_| panic!("did not expect output fn to get called"),
    );
}

pub fn iterator_input_fn(mut iter: impl Iterator<Item = i64>) -> impl InputFn {
    move || iter.next().expect("input fn called after iterator was exhausted")
}

pub fn parse_program(input: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let result: Result<Vec<_>, _> = crate::read_single_line(input)?
        .split(',')
        .map(|n| n.parse::<i64>())
        .collect();
    Ok(result?)
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

    fn execute_no_input(program: &Vec<i64>) -> Vec<i64> {
        execute_with_input(program, || panic!("input fn should not have been called"))
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
    fn test_relative_mode() {
        let program = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];

        let outputs = execute_no_input(&program);
        assert_eq!(program, outputs);
    }

    #[test]
    fn test_large_numbers_1() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let outputs = execute_no_input(&program);
        assert_eq!(vec![1219070632396864], outputs);
    }

    #[test]
    fn test_large_numbers_2() {
        let program = vec![104, 1125899906842624, 99];

        let outputs = execute_no_input(&program);
        assert_eq!(vec![1125899906842624], outputs);
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
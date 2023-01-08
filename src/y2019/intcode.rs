pub fn execute_program(program: &mut Vec<i64>) {
    let mut ip = 0;
    while ip < program.len() {
        match program[ip] {
            1 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                let c = program[ip + 3] as usize;
                program[c] = a + b;
            }
            2 => {
                let a = program[program[ip + 1] as usize];
                let b = program[program[ip + 2] as usize];
                let c = program[ip + 3] as usize;
                program[c] = a * b;
            }
            99 => {
                break;
            }
            _ => panic!("invalid opcode: {}", program[ip])
        }

        ip += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut program = vec![1, 0, 0, 0, 99];
        execute_program(&mut program);
        assert_eq!(vec![2, 0, 0, 0, 99], program);
    }

    #[test]
    fn test_multiply() {
        let mut program = vec![2, 3, 0, 3, 99];
        execute_program(&mut program);
        assert_eq!(vec![2, 3, 0, 6, 99], program);
    }

    #[test]
    fn test_sample_input_1() {
        let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        execute_program(&mut program);
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], program);
    }

    #[test]
    fn test_sample_input_2() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        execute_program(&mut program);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], program);
    }

    #[test]
    fn test_sample_input_3() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute_program(&mut program);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], program);
    }
}
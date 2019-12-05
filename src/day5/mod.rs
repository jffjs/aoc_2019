use crate::util::read_input;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

struct Intcode(u32);

impl Intcode {
    fn mode(&self, param: u32) -> Mode {
        let place = 10u32.pow(param + 1);

        match (self.0 / place) % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Invalid mode in Intcode({})", self.0),
        }
    }

    fn op(&self) -> u32 {
        (self.0 % 10) + ((self.0 / 10) % 10) * 10
    }
}

#[derive(Debug)]
struct Output {
    pc: usize,
    value: i32,
}

pub fn day_5() {
    let mut file = read_input(5);

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // part 1
    let mut program = init_program(&buf);
    let output = run_program(&mut program, 1);

    println!("Day 5-1: {:?}", output);

    // part 2
    let mut program = init_program(&buf);
    let output = run_program(&mut program, 5);

    println!("Day 5-2: {:?}", output);
}

fn init_program(data: &String) -> Vec<i32> {
    data.trim()
        .split(',')
        .map(|code| code.parse::<i32>().unwrap())
        .collect()
}

fn run_program(program: &mut Vec<i32>, input: i32) -> Vec<Output> {
    let mut pc: usize = 0;
    let mut output: Vec<Output> = Vec::new();

    loop {
        assert!(program[pc] > 0, "Invalid Intcode({})", program[pc]);
        let intcode = Intcode(program[pc] as u32);

        match intcode.op() {
            1 => {
                let val1 = read_value(&intcode, 1, pc, program);
                let val2 = read_value(&intcode, 2, pc, program);
                let pos = program[pc + 3] as usize;
                program[pos] = val1 + val2;
                pc += 4;
            }
            2 => {
                let val1 = read_value(&intcode, 1, pc, program);
                let val2 = read_value(&intcode, 2, pc, program);
                let pos = program[pc + 3] as usize;
                program[pos] = val1 * val2;
                pc += 4;
            }
            3 => {
                let pos = program[pc + 1] as usize;
                program[pos] = input;
                pc += 2;
            }
            4 => {
                let value = read_value(&intcode, 1, pc, program);
                output.push(Output { pc, value });
                pc += 2;
            }
            5 => {
                let test = read_value(&intcode, 1, pc, program);
                if test != 0 {
                    pc = read_value(&intcode, 2, pc, program) as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let test = read_value(&intcode, 1, pc, program);
                if test == 0 {
                    pc = read_value(&intcode, 2, pc, program) as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let val1 = read_value(&intcode, 1, pc, program);
                let val2 = read_value(&intcode, 2, pc, program);
                let pos = program[pc + 3] as usize;
                program[pos] = if val1 < val2 { 1 } else { 0 };
                pc += 4;
            }
            8 => {
                let val1 = read_value(&intcode, 1, pc, program);
                let val2 = read_value(&intcode, 2, pc, program);
                let pos = program[pc + 3] as usize;
                program[pos] = if val1 == val2 { 1 } else { 0 };
                pc += 4;
            }
            99 => {
                break;
            }
            _ => panic!("Unknown instruction at position {}", pc),
        }
    }

    output
}

fn read_value(intcode: &Intcode, param: u32, pc: usize, program: &Vec<i32>) -> i32 {
    match intcode.mode(param) {
        Mode::Position => program[program[pc + param as usize] as usize],
        Mode::Immediate => program[pc + param as usize],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intcode_mode() {
        let intcode = Intcode(1002);

        assert_eq!(intcode.mode(1), Mode::Position);
        assert_eq!(intcode.mode(2), Mode::Immediate);
        assert_eq!(intcode.mode(3), Mode::Position);
    }

    #[test]
    fn test_intcode_op() {
        assert_eq!(Intcode(1001).op(), 1);
        assert_eq!(Intcode(102).op(), 2);
        assert_eq!(Intcode(3).op(), 3);
        assert_eq!(Intcode(1104).op(), 4);
        assert_eq!(Intcode(11099).op(), 99);
    }
}

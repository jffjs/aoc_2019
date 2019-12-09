use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

struct Intcode(u32);

impl Intcode {
    fn mode(&self, param: u32) -> Mode {
        let place = 10u32.pow(param + 1);

        match (self.0 / place) % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Invalid mode in Intcode({})", self.0),
        }
    }

    fn op(&self) -> u32 {
        (self.0 % 10) + ((self.0 / 10) % 10) * 10
    }
}

pub struct IntcodeComputer {
    memory: Vec<isize>,
    pc: usize,
    relative_base: isize,
    input_buffer: VecDeque<isize>,
    pub output_buffer: Vec<isize>,
    pub blocked: bool,
    pub halted: bool,
}

impl IntcodeComputer {
    pub fn new(mem: &String) -> IntcodeComputer {
        let mut memory: Vec<isize> = mem
            .trim()
            .split(',')
            .map(|code| code.parse::<isize>().unwrap())
            .collect();

        // Allocate additional memory at end
        for _ in 0..memory.len() * 100 {
            memory.push(0);
        }

        IntcodeComputer {
            memory,
            pc: 0,
            relative_base: 0,
            input_buffer: VecDeque::new(),
            output_buffer: Vec::new(),
            blocked: false,
            halted: false,
        }
    }

    pub fn input(&mut self, value: isize) {
        self.input_buffer.push_back(value);
    }

    pub fn run(&mut self) {
        loop {
            if self.input_buffer.len() > 0 {
                self.blocked = false;
            }

            if self.blocked || self.halted {
                break;
            }

            let intcode = Intcode(self.memory[self.pc] as u32);

            match intcode.op() {
                1 => {
                    let val1 = self.read_value(&intcode, 1);
                    let val2 = self.read_value(&intcode, 2);
                    let pos = self.read_addr(&intcode, 3);
                    self.memory[pos] = val1 + val2;
                    self.pc += 4;
                }
                2 => {
                    let val1 = self.read_value(&intcode, 1);
                    let val2 = self.read_value(&intcode, 2);
                    let pos = self.read_addr(&intcode, 3);
                    self.memory[pos] = val1 * val2;
                    self.pc += 4;
                }
                3 => {
                    // let pos = self.memory[self.pc + 1] as usize;
                    let addr = self.read_addr(&intcode, 1);
                    if let Some(input) = self.input_buffer.pop_front() {
                        self.memory[addr as usize] = input;
                        self.pc += 2;
                    } else {
                        self.blocked = true;
                    }
                }
                4 => {
                    let value = self.read_value(&intcode, 1);
                    self.output_buffer.push(value);
                    self.pc += 2;
                }
                5 => {
                    let test = self.read_value(&intcode, 1);
                    if test != 0 {
                        self.pc = self.read_value(&intcode, 2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let test = self.read_value(&intcode, 1);
                    if test == 0 {
                        self.pc = self.read_value(&intcode, 2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let val1 = self.read_value(&intcode, 1);
                    let val2 = self.read_value(&intcode, 2);
                    let pos = self.read_addr(&intcode, 3);
                    self.memory[pos] = if val1 < val2 { 1 } else { 0 };
                    self.pc += 4;
                }
                8 => {
                    let val1 = self.read_value(&intcode, 1);
                    let val2 = self.read_value(&intcode, 2);
                    let pos = self.read_addr(&intcode, 3);
                    self.memory[pos] = if val1 == val2 { 1 } else { 0 };
                    self.pc += 4;
                }
                9 => {
                    let value = self.read_value(&intcode, 1);
                    self.relative_base += value;
                    self.pc += 2;
                }
                99 => {
                    self.halted = true;
                }
                _ => panic!(
                    "Unknown instruction. Position: {}, code: {}",
                    self.pc, intcode.0
                ),
            }
        }
    }

    fn read_addr(&mut self, intcode: &Intcode, param: usize) -> usize {
        match intcode.mode(param as u32) {
            Mode::Position => self.memory[self.pc + param] as usize,
            Mode::Relative => (self.relative_base + self.memory[self.pc + param]) as usize,
            Mode::Immediate => panic!("Invalid mode for address"),
        }
    }

    fn read_value(&mut self, intcode: &Intcode, param: usize) -> isize {
        match intcode.mode(param as u32) {
            Mode::Position => self.memory[self.memory[self.pc + param] as usize],
            Mode::Immediate => self.memory[self.pc + param],
            Mode::Relative => {
                self.memory[(self.relative_base + self.memory[self.pc + param]) as usize]
            }
        }
    }
}

pub fn init_program(data: &String) -> Vec<i32> {
    let mut memory: Vec<i32> = data
        .trim()
        .split(',')
        .map(|code| code.parse::<i32>().unwrap())
        .collect();

    // Allocate additional memory at end
    for _ in 0..memory.len() * 3 {
        memory.push(0);
    }

    memory
}

pub fn run_program(program: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut pc: usize = 0;
    let mut output: Vec<i32> = Vec::new();
    let mut input_iter = input.iter();

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
                program[pos] = *input_iter.next().unwrap();
                pc += 2;
            }
            4 => {
                let value = read_value(&intcode, 1, pc, program);
                output.push(value);
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
            _ => panic!("Unknown instruction. Position: {}, code: {}", pc, intcode.0),
        }
    }

    output
}

fn read_value(intcode: &Intcode, param: usize, pc: usize, program: &Vec<i32>) -> i32 {
    match intcode.mode(param as u32) {
        Mode::Position => program[program[pc + param] as usize],
        Mode::Immediate => program[pc + param],
        Mode::Relative => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intcode_mode() {
        let intcode = Intcode(1202);

        assert_eq!(intcode.mode(1), Mode::Relative);
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

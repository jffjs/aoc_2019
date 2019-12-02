use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day_2() -> i32 {
    let path = Path::new("src/day2/input.txt");
    let mut file = File::open(path).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut program: Vec<i32> = buf
        .trim()
        .split(',')
        .map(|code| code.parse::<i32>().unwrap())
        .collect();

    program[1] = 12;
    program[2] = 2;

    run_program(&mut program);

    program[0]
}

fn run_program(program: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut pc: usize = 0;

    loop {
        match program[pc] {
            1 => {
                let val1 = program[program[pc + 1] as usize];
                let val2 = program[program[pc + 2] as usize];
                let pos = program[pc + 3] as usize;
                program[pos] = val1 + val2;
                pc += 4;
            }
            2 => {
                let val1 = program[program[pc + 1] as usize];
                let val2 = program[program[pc + 2] as usize];
                let pos = program[pc + 3] as usize;
                program[pos] = val1 * val2;
                pc += 4;
            }
            99 => {
                break;
            }
            _ => panic!("Unknown instruction at position {}", pc),
        }
    }

    program
}

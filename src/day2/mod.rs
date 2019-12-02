use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn day_2() {
    let path = Path::new("src/day2/input.txt");
    let mut file = File::open(path).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // let mut program: Vec<i32> = init_program(&buf);

    // run_program(&mut program, 12, 2);

    // program[0]

    if let Some((noun, verb)) = find_noun_and_verb(&buf, 19690720) {
        println!("Day 2: {}", 100 * noun + verb);
    } else {
        println!("Day 2: no answer found");
    }
}

fn init_program(data: &String) -> Vec<i32> {
    data.trim()
        .split(',')
        .map(|code| code.parse::<i32>().unwrap())
        .collect()
}

fn run_program(program: &mut Vec<i32>, noun: i32, verb: i32) -> &mut Vec<i32> {
    let mut pc: usize = 0;
    program[1] = noun;
    program[2] = verb;

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

fn find_noun_and_verb(data: &String, output: i32) -> Option<(i32, i32)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = init_program(data);

            run_program(&mut program, noun, verb);

            if program[0] == output {
                return Some((noun, verb));
            }
        }
    }

    None
}

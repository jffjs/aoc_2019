use crate::intcode::IntcodeComputer;
use crate::util::read_input;
use std::io::prelude::*;

pub fn day_9() {
    let mut file = read_input(9);

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // Part 1
    let mut cpu = IntcodeComputer::new(&buf);
    cpu.input(1);
    cpu.run();

    println!("Day 9-1: {}", cpu.output_buffer[0]);

    // Part 2
    let mut cpu = IntcodeComputer::new(&buf);
    cpu.input(2);
    cpu.run();

    println!("Day 9-2: {}", cpu.output_buffer[0]);
}

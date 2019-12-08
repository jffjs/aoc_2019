use crate::intcode::{init_program, run_program};
use crate::util::read_input;
use std::io::prelude::*;

pub fn day_5() {
    let mut file = read_input(5);

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // part 1
    let mut program = init_program(&buf);
    let output = run_program(&mut program, &vec![1]);

    println!("Day 5-1: {:?}", output);

    // part 2
    let mut program = init_program(&buf);
    let output = run_program(&mut program, &vec![5]);

    println!("Day 5-2: {:?}", output);
}

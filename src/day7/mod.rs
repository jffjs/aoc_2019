use crate::intcode::IntcodeComputer;
use crate::util::read_input;
use std::io::prelude::*;

type PhaseSeq = [u8; 5];

pub fn day_7() {
    let mut file = read_input(7);

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    // Part 1
    let sequences = phase_sequences(0, 4);
    let mut max_output = 0;
    for phase_seq in sequences.iter() {
        let output_a = run_amplifier_program(phase_seq[0] as i32, 0, &buf);
        let output_b = run_amplifier_program(phase_seq[1] as i32, output_a, &buf);
        let output_c = run_amplifier_program(phase_seq[2] as i32, output_b, &buf);
        let output_d = run_amplifier_program(phase_seq[3] as i32, output_c, &buf);
        let output_e = run_amplifier_program(phase_seq[4] as i32, output_d, &buf);

        max_output = std::cmp::max(max_output, output_e);
    }

    println!("Day 7-1: {}", max_output);

    // Part 2
    let sequences = phase_sequences(5, 9);
    let mut max_output = 0;
    for phase_seq in sequences.iter() {
        let mut amp_a = IntcodeComputer::new("a".to_owned(), &buf);
        amp_a.input(phase_seq[0] as i32);
        amp_a.input(0);

        let mut amp_b = IntcodeComputer::new("b".to_owned(), &buf);
        amp_b.input(phase_seq[1] as i32);

        let mut amp_c = IntcodeComputer::new("c".to_owned(), &buf);
        amp_c.input(phase_seq[2] as i32);

        let mut amp_d = IntcodeComputer::new("d".to_owned(), &buf);
        amp_d.input(phase_seq[3] as i32);

        let mut amp_e = IntcodeComputer::new("e".to_owned(), &buf);
        amp_e.input(phase_seq[4] as i32);

        loop {
            if amp_a.halted && amp_b.halted && amp_c.halted && amp_d.halted && amp_e.halted {
                break;
            }

            amp_a.run();

            amp_b.input(*amp_a.output_buffer.last().unwrap());
            amp_b.run();

            amp_c.input(*amp_b.output_buffer.last().unwrap());
            amp_c.run();

            amp_d.input(*amp_c.output_buffer.last().unwrap());
            amp_d.run();

            amp_e.input(*amp_d.output_buffer.last().unwrap());
            amp_e.run();

            amp_a.input(*amp_e.output_buffer.last().unwrap());
        }
        max_output = std::cmp::max(max_output, *amp_e.output_buffer.last().unwrap());
    }

    println!("Day 7-2: {}", max_output);
}

fn run_amplifier_program(phase: i32, input: i32, memory: &String) -> i32 {
    let mut cpu = IntcodeComputer::new("cpu".to_owned(), memory);
    cpu.input(phase);
    cpu.input(input);
    cpu.run();
    cpu.output_buffer[0]
}

fn phase_sequences(start: u8, end: u8) -> Vec<PhaseSeq> {
    let mut sequences: Vec<PhaseSeq> = Vec::new();

    for a in start..=end {
        for b in start..=end {
            for c in start..=end {
                for d in start..=end {
                    for e in start..=end {
                        sequences.push([a, b, c, d, e]);
                    }
                }
            }
        }
    }

    sequences
        .iter()
        .filter(|seq| {
            for (i, phase) in seq.iter().enumerate() {
                let mut iterator = seq.iter().skip(i + 1);
                while let Some(next) = iterator.next() {
                    if phase == next {
                        return false;
                    }
                }
            }
            true
        })
        .copied()
        .collect()
}

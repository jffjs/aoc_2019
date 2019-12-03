use crate::util::read_input;
use std::io::{self, BufRead};

pub fn day_1() {
    let reader = io::BufReader::new(read_input(1));

    let fuel = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |total, mass| total + fuel_calc(mass));

    println!("Day 1: {}", fuel);
}

fn fuel_calc(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + fuel_calc(fuel)
}

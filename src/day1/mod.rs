use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day_1() {
    let path = Path::new("src/day1/input.txt");
    let file = File::open(path).unwrap();

    let reader = io::BufReader::new(file);

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

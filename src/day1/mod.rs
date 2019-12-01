use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn calculate_fuel() -> i32 {
    let path = Path::new("src/day1/input.txt");
    let file = File::open(path).unwrap();
    let mut total_fuel: i32 = 0;

    let reader = io::BufReader::new(file);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        let mass = line.parse::<i32>().unwrap();

        total_fuel += fuel_calc(mass);
    }

    total_fuel
}

fn fuel_calc(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + fuel_calc(fuel)
}

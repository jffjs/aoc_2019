use crate::util::read_input;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

type Coord = (i32, i32);
type Grid = HashMap<Coord, HashSet<usize>>;

#[derive(Debug)]
enum Segment {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl Segment {
    fn new(segment: &str) -> Segment {
        match segment.chars().nth(0) {
            Some(c) => {
                let (_, distance) = segment.split_at(1);
                let distance = distance.parse::<i32>().unwrap();
                match c {
                    'U' => Segment::U(distance),
                    'D' => Segment::D(distance),
                    'R' => Segment::R(distance),
                    'L' => Segment::L(distance),
                    _ => panic!("Unknown segment type"),
                }
            }
            _ => panic!("Unknown segment type"),
        }
    }
}

pub fn day_3() {
    let file = read_input(3);
    let reader = BufReader::new(file);

    let mut grid = Grid::new();

    for (wire, line_) in reader.lines().enumerate() {
        let wire = wire + 1;
        let line = line_.unwrap();
        let path: Vec<Segment> = line
            .trim()
            .split(',')
            .map(|segment| Segment::new(&segment))
            .collect();

        let mut position: Coord = (0, 0);

        for segment in path.iter() {
            match segment {
                Segment::U(y) => {
                    let x = position.0;
                    let sy = position.1;
                    for dy in 1..=*y {
                        mark_grid(&mut grid, (x, sy + dy), wire);
                    }
                    position = (x, sy + y);
                }
                Segment::D(y) => {
                    let x = position.0;
                    let sy = position.1;
                    for dy in 1..=*y {
                        mark_grid(&mut grid, (x, sy - dy), wire);
                    }
                    position = (x, sy - y);
                }
                Segment::R(x) => {
                    let sx = position.0;
                    let y = position.1;
                    for dx in 1..=*x {
                        mark_grid(&mut grid, (sx + dx, y), wire);
                    }
                    position = (sx + x, y);
                }
                Segment::L(x) => {
                    let sx = position.0;
                    let y = position.1;
                    for dx in 1..=*x {
                        mark_grid(&mut grid, (sx - dx, y), wire);
                    }
                    position = (sx - x, y);
                }
            }
        }
    }

    // Part 1
    let mut shortest_distance: i32 = 999_999;
    for (pos, wires) in grid.iter() {
        if wires.len() > 1 {
            let distance = pos.0.abs() + pos.1.abs();
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }
    }

    println!("Day 3-1: {}", shortest_distance);

    // Part 2
}

fn mark_grid(grid: &mut Grid, pos: Coord, wire: usize) {
    if let Some(set) = grid.get_mut(&pos) {
        set.insert(wire);
    } else {
        let mut set = HashSet::new();
        set.insert(wire);
        grid.insert(pos, set);
    }
}

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

    let wire_paths: Vec<Vec<Segment>> = reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            line.trim()
                .split(',')
                .map(|segment| Segment::new(&segment))
                .collect()
        })
        .collect();

    for (wire, path) in wire_paths.iter().enumerate() {
        let wire = wire + 1;

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
    let intersections: Vec<Coord> = grid
        .iter()
        .filter(|(_, wires)| wires.len() > 1)
        .map(|(pos, _)| *pos)
        .collect();

    let mut shortest_steps = 999_999;
    for intersection in intersections.iter() {
        let mut total_steps = 0;
        for path in wire_paths.iter() {
            let mut position: Coord = (0, 0);
            let mut steps = 0;
            for segment in path.iter() {
                match segment {
                    Segment::U(y) => {
                        let x = position.0;
                        let sy = position.1;
                        for dy in 1..=*y {
                            position = (x, sy + dy);
                            steps += 1;

                            if position == *intersection {
                                total_steps += steps;
                                break;
                            }
                        }
                    }
                    Segment::D(y) => {
                        let x = position.0;
                        let sy = position.1;
                        for dy in 1..=*y {
                            position = (x, sy - dy);
                            steps += 1;

                            if position == *intersection {
                                total_steps += steps;
                                break;
                            }
                        }
                    }
                    Segment::R(x) => {
                        let sx = position.0;
                        let y = position.1;
                        for dx in 1..=*x {
                            position = (sx + dx, y);
                            steps += 1;

                            if position == *intersection {
                                total_steps += steps;
                                break;
                            }
                        }
                    }
                    Segment::L(x) => {
                        let sx = position.0;
                        let y = position.1;
                        for dx in 1..=*x {
                            position = (sx - dx, y);
                            steps += 1;

                            if position == *intersection {
                                total_steps += steps;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if total_steps < shortest_steps {
            shortest_steps = total_steps;
        }
    }

    println!("Day 3-2: {}", shortest_steps);
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

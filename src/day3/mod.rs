use crate::util::read_input;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

type Coord = (i32, i32);
type Grid = HashMap<Coord, Vec<usize>>;

enum Point {
    Wire1,
    Wire2,
    Intersect,
}

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
                    'D' => Segment::D(-distance),
                    'R' => Segment::R(distance),
                    'L' => Segment::L(-distance),
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
        let line = line_.unwrap();
        let path: Vec<Segment> = line
            .trim()
            .split(',')
            .map(|segment| Segment::new(&segment))
            .collect();

        let mut position: Coord = (0, 0);

        for segment in path.iter() {
            position = match segment {
                Segment::U(y) | Segment::D(y) => (position.0, position.1 + y),
                Segment::R(x) | Segment::L(x) => (position.0 + x, position.1),
            };

            grid.entry(position)
                .and_modify(|wires| wires.push(wire + 1))
                .or_insert(vec![wire + 1]);
        }
    }

    println!("{:?}", grid);

    let mut shortest_distance: i32 = 999_999;
    for (pos, wires) in grid.iter() {
        if wires.len() > 1 {
            let distance = pos.0.abs() + pos.1.abs();
            if distance < shortest_distance {
                shortest_distance = distance;
            }
        }
    }

    println!("Day 3: {}", shortest_distance);
}

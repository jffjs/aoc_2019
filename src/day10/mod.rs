use crate::util::read_input;
use std::collections::{HashMap, HashSet};
use std::io::{prelude::*, BufReader};

mod bresenham;
use bresenham::{plot_line, Point};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Space,
    Asteroid,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Tile::Space => write!(f, "."),
            &Tile::Asteroid => write!(f, "#"),
        }
    }
}

impl std::convert::From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Space,
            '#' => Tile::Asteroid,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.width]
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.tile(x, y));
            }
            print!("\n");
        }
        std::io::stdout().flush().unwrap();
    }
}

pub fn day_10() {
    let file = read_input(10);
    let reader = BufReader::new(file);

    // let mut grid = Grid {
    //     tiles: Vec::new(),
    //     width: 0,
    //     height: 0,
    // };

    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     grid.width = line.len();
    //     grid.height += 1;

    //     for c in line.chars() {
    //         let tile = Tile::from(c);
    //         grid.tiles.push(tile);
    //     }
    // }

    let mut asteroids: HashSet<Point> = HashSet::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }

    let mut asteroids_los: HashMap<Point, usize> = HashMap::new();

    for a1 in asteroids.iter() {
        let mut in_los = 0;
        for a2 in asteroids.iter().filter(|a| a != &a1) {
            let mut obscured = false;
            let line = plot_line(a1.x, a1.y, a2.x, a2.y);

            for point in line.iter().skip(1) {
                if let Some(a3) = asteroids.get(point) {
                    obscured = true;
                    break;
                }
            }

            if !obscured {
                in_los += 1;
            }
        }

        asteroids_los.insert(a1.clone(), in_los);
    }

    println!("{:?}", asteroids_los);
    let mut max_los = 0;
    let mut best_asteroid: Point;

    for (asteroid, los) in asteroids_los.iter() {
        if *los > max_los {
            max_los = *los;
            best_asteroid = Point {
                x: asteroid.x,
                y: asteroid.y,
            };
        }
    }

    println!("Day 10-1: {}", max_los);
}

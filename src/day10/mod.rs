use crate::util::read_input;
use std::io::{prelude::*, BufReader};

mod bresenham;

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

    let mut grid = Grid {
        tiles: Vec::new(),
        width: 0,
        height: 0,
    };

    for line in reader.lines() {
        let line = line.unwrap();
        grid.width = line.len();
        grid.height += 1;

        for c in line.chars() {
            let tile = Tile::from(c);
            grid.tiles.push(tile);
        }
    }
}

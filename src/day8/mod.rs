use crate::util::read_input;
use std::io::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}

impl std::convert::From<u32> for Pixel {
    fn from(val: u32) -> Self {
        match val {
            0 => Pixel::Black,
            1 => Pixel::White,
            2 => Pixel::Transparent,
            _ => panic!("Invalid pixel value: {}", val),
        }
    }
}

pub fn day_8() {
    let mut file = read_input(8);

    let mut file_buf = String::new();
    file.read_to_string(&mut file_buf).unwrap();

    let mut image_buf: Vec<u32> = Vec::new();
    for c in file_buf.chars() {
        if let Some(digit) = c.to_digit(10) {
            image_buf.push(digit);
        }
    }

    // Part 1
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const PIXELS: usize = WIDTH * HEIGHT;
    type Layer = Vec<u32>;

    let mut layers: Vec<Layer> = vec![vec![]];
    let mut zeroes: Vec<u32> = vec![0];
    let mut ones: Vec<u32> = vec![0];
    let mut twos: Vec<u32> = vec![0];

    let mut layer = 0;
    let mut i = 0;
    loop {
        if i == image_buf.len() {
            break;
        }

        let pixel = image_buf[i];
        layers[layer].push(pixel);

        if pixel == 0 {
            zeroes[layer] += 1;
        }
        if pixel == 1 {
            ones[layer] += 1;
        }

        if pixel == 2 {
            twos[layer] += 1;
        }

        i += 1;
        if i % PIXELS == 0 && i < image_buf.len() {
            layer += 1;
            layers.push(Vec::new());
            zeroes.push(0);
            ones.push(0);
            twos.push(0);
        }
    }

    let mut zeroes_count = u32::max_value();
    let mut fewest_zeroes = 0;
    for (i, count) in zeroes.iter().enumerate() {
        if *count < zeroes_count {
            zeroes_count = *count;
            fewest_zeroes = i;
        }
    }
    println!("Day 8-1: {}", ones[fewest_zeroes] * twos[fewest_zeroes]);

    // Part 2
    let mut image: [Pixel; PIXELS] = [Pixel::Transparent; PIXELS];
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let i = x + (y * WIDTH);

            for layer in layers.iter() {
                let pixel = Pixel::from(layer[i]);

                if pixel == Pixel::Black || pixel == Pixel::White {
                    image[i] = pixel;
                    break;
                }
            }
        }
    }

    let black = ' ';
    let white = '█';
    let mut out = String::new();
    for (i, pixel) in image.iter().enumerate() {
        if i % WIDTH == 0 {
            out.push('\n');
        }
        match pixel {
            Pixel::Black => out.push(black),
            Pixel::White => out.push(white),
            _ => continue,
        }
    }

    println!("Day 8-2:");
    println!("{}", out);
}

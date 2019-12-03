use std::fs::File;
use std::path::Path;

pub fn read_input(day: usize) -> File {
    let path_to_input = format!("src/day{}/input.txt", day);
    let path = Path::new(&path_to_input);
    File::open(path).unwrap()
}

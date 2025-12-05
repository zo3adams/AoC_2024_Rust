
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn read_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn part_one(values: &[(i32)]) -> i32 {
    0
}

fn part_two(values: &[(i32)]) -> i32 {
    0
}

pub fn run() -> io::Result<()> {
    let values = read_data("src/input.txt")?;

    println!("Part 1: {}", part_one( &values));
    //println!("Part 2: {}", part_two(start, &instructions));

    Ok(())
}
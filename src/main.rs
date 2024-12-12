
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}


fn part_one(map: Vec<Vec<char>>) -> u32 {
   0
}

fn part_two(map: Vec<Vec<char>>) -> u32 {
   0
}


fn main() {
    let input = "./src/input.txt";     

    match read_lines(input) {
        Ok(value) => {     
            let mut map: Vec<Vec<char>> = value.iter().map(|s| s.chars().collect()).collect();   
            //println!("{:?}", map);
            println!("Answer to part one:  {}", part_one(map));
            //println!("Answer to part two:  {}", part_two(map));
        },
        Err(e) => println!("Error: {}", e),
    }
}
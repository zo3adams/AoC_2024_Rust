
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn read_instructions<P: AsRef<Path>>(filename: P) -> io::Result<Vec<(char, i32)>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    reader.lines()
        .map(|line| {
            let line = line?;
            let mut chars = line.chars();

            let dir = chars.next().unwrap();
            let dist: i32 = chars.as_str().parse().unwrap();

            Ok((dir, dist))
        })
        .collect()
}


fn _part_one(position: i32, instructions: &[(char, i32)]) -> i32 {
    let mut next_position = position;
    let mut count_zero = 0;

    for (direction, distance) in instructions {
        let delta = match direction {
            'L' => -(distance % 100),
            'R' =>  distance % 100,
            _ => panic!("Invalid direction"),
        };

        next_position = (next_position + delta).rem_euclid(100);
        //println!("Got direction {} and distance {} => now at {} ", direction, distance, next_position);
        if next_position == 0 {
            count_zero += 1;
        }
    }
    
    count_zero
}

fn part_two(position: i32, instructions: &[(char, i32)]) -> i32 {
    let mut count = 0;
    let mut last_position = position;

    for &(dir, mut dist) in instructions {
        // Count full rotations
        count += dist / 100;
        dist %= 100;

        let delta = match dir {
            'L' => -dist,
            'R' =>  dist,
            _ => panic!("Invalid direction"),
        };

        let mut new_pos = last_position + delta;

        // Detect wrap crossing
        if new_pos < 0 || new_pos > 99 {
            if last_position != 0 {
                count += 1;
            }
        } else if new_pos == 0 {
            count += 1;
        }

        new_pos = new_pos.rem_euclid(100);
        last_position = new_pos;
    }

    count
}

pub fn run()-> io::Result<()> {
    println!("AoC 2025 Day 01!");

    let instructions = read_instructions("src/input.txt")?;
    let start = 50;

    //println!("Part 1: {}", part_one(start, &instructions));
    println!("Part 2: {}", part_two(start, &instructions));

    Ok(())
}
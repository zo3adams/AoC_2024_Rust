
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet};
use std::cmp::{min, max};

fn read_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn part_one(fresh_ingredients: & Vec<(i64,i64)> , available_ingredients: & Vec<i64>) -> i64 {
    let mut fresh_count= 0;
    for ingredient_id in available_ingredients {

        for (f_id_start, f_id_end) in fresh_ingredients {
            if ingredient_id >= f_id_start && ingredient_id <= f_id_end {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn part_two(fresh_ingredients: &Vec<(i64, i64)>) -> i64 {
    // sort ranges by starting value
    let mut ranges = fresh_ingredients.clone();
    ranges.sort_by_key(|r| r.0);

    // find overlaps and merge
    let mut merged = Vec::new();
    merged.push(ranges[0]);

    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();

        if start <= last.1 {
            // overlaps - since we're sorted start should be lower, so add on to end
            last.1 = last.1.max(end);
        } else {
            // no overlap here, add as new distinct range
            merged.push((start, end));
        }
    }

    merged
        .iter()
        .map(|(s, e)| 1 + (e - s))
        .sum()
}


pub fn run() -> io::Result<()> {
    let values = read_data("src/input.txt")?;
    let mut fresh_ingredients: Vec<(i64,i64)> =Vec::new();
    let mut available_ingredients: Vec<i64> = Vec::new();


    for line in values {
        if line.contains('-') {
            let tokens: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
            let start: i64 = tokens[0].parse().expect("parse failure");
            let end: i64 = tokens[1].parse().expect("parse failure");
            fresh_ingredients.push((start,end));
        }else {
            let ingredient_id: Result<i64, _> = line.parse();
            match ingredient_id {
                Ok(id) => {
                    available_ingredients.push(id);
                }
                Err(e) => {

                }
            }
        }
        
    }

    //println!("Got fresh ingredients {:?}", fresh_ingredients);
    //println!("Got available ingredients {:?}", available_ingredients);
    println!("Part 1: {}", part_one( &fresh_ingredients, &available_ingredients));
    println!("Part 2: {}", part_two(&fresh_ingredients));

    Ok(())
}
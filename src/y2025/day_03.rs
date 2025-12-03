
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn part_one(values: &[String]) -> u32 {

    let mut sum = 0;

    for bank in values {
        let mut first_max_seen = 0;
        let mut second_max_seen = 0;
        let mut max_index = 0;
        let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
        //println!("bank {:?}", digits);
        for i  in 0..digits.len().saturating_sub(1) {
            if digits[i] > first_max_seen { 
                first_max_seen = digits[i];
                max_index = i;
            };
        }

        for j in (max_index+1)..digits.len() {
            if digits[j] > second_max_seen { 
                second_max_seen = digits[j];
            };
        }

        let bank_value = first_max_seen * 10 + second_max_seen;
        //println!("bank value {}", bank_value);
        sum += bank_value;
    }
    sum
}


fn part_two(values: &[String]) -> u128 {
    let mut sum: u128 = 0;
    for bank in values {
        //println!("working bank {} with length {}", bank, bank.len());
        let digits: Vec<u32> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();

        //go through string choosing max digit that has at least 12  - size of existing string digits left
        
        let mut digits_so_far: Vec<u32> = Vec::new();
        let mut i = 0;
        while i < bank.len() && digits_so_far.len() < 12 {

            let mut temp_max = 0;
            let mut temp_max_index = 0;
            let stop_point = bank.len() - 12 + (digits_so_far.len());
            //println!("i now {}, stop point now {} and dsf {:?}", i, stop_point, digits_so_far);
            for j in i..=stop_point {
                if digits[j] > temp_max {
                    //println!("\t found new temp max {} at index {}/ {}  dsf {:?}", as_int, j, stop_point, digits_so_far);
                    temp_max = digits[j];
                    temp_max_index = j;
                }
            }
            
            digits_so_far.push(temp_max);
            i = temp_max_index+1;
        }

        let bank_value = digits_so_far.into_iter().fold(0_u128, |acc, d| acc * 10 + d as u128);
        //println!("bank value {}", bank_value);
        sum += bank_value;
    }
    sum
}

pub fn run() -> io::Result<()>{
    println!("Running AoC 2025 Day 3...");
    let values = read_data("src/input.txt")?;

    println!("Part 1: {}", part_one( &values));
    println!("Part 2: {}", part_two(&values));

    Ok(())
}
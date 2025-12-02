
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let n = s.len();

    // A repeated pattern must divide the total length
    for len in 1..=n/2 {
        if n % len == 0 {
            let pattern = &s[..len];
            let repeats = n / len;

            if pattern.repeat(repeats) == s {
                return true;
            }
        }
    }

    false
}


fn read_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<u64>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut numbers: Vec<u64> = Vec::new();

    for line in lines {
        let number_ranges: Vec<&str> = line.split(',').collect();

        for number_range_str in number_ranges {
            let number_range: Vec<&str> = number_range_str.split('-').collect();
             //println!("number range {:?}", number_range);

            let first_number: u64 = number_range[0].parse().expect("parse failure");
            let second_number: u64 = number_range[1].parse().expect("parse failure");

            for i in first_number..=second_number {
                numbers.push(i);
            }
        }
       
    }

    Ok(numbers)

}


fn _part_one(values: Vec<u64>) -> u64 {
    
    //get digit count
    let mut repeats : Vec<u64> = Vec::new();
    let mut repeat_sum = 0;

    for value in values {

        let v_as_str = format!("{}", value);
        if v_as_str.len() %2 == 1 { continue;};
        let half_point = v_as_str.len() / 2;

        let first_half = &v_as_str[0..half_point];
        let second_half = &v_as_str[half_point..];

        //println!("{} = Checking {} vs {}", v_as_str, first_half, second_half);

        if first_half == second_half {
            //println!("basic equality check worked!");
            repeats.push(value);
            repeat_sum += value;
        }
    }

    println!("Found repeats {:?} with sum {}", repeats, repeat_sum);
    
    repeat_sum
}

fn part_two(values: Vec<u64>) -> u64 {
    //get digit count
    let mut repeats : Vec<u64> = Vec::new();
    let mut repeat_sum = 0;

    for value in values {
        if is_invalid_id(value) {
            repeats.push(value);
            repeat_sum += value;
        }
    }

    println!("Found repeats {:?} with sum {}", repeats, repeat_sum);
    
    repeat_sum
}

pub fn run() -> io::Result<()> {
    println!("AoC 2025 Day 02!");
    let values = read_data("src/sample_input.txt")?;

    //println!("Part 1: {}", part_one( values));
    println!("Part 2: {}", part_two(values));

    Ok(())
}
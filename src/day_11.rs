
use std::collections::{HashMap};
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



//terminal states ?
// 1 -> 2024 -> 20, 24 -> 2 0 2 4 ->  4048 1 4048 8096 -> 40  48  2024  40  48  80 96 -> 
fn part_one(data: Vec<u64>) -> u64 {
    let mut total_digit_count = 0;
    for val in data {
        let mut vals_for_now: Vec<u64> = vec![val];
        //remember, we are treating vals_for_now as reversed
        for _i in 0..25 {
            let mut temp_vals:Vec<u64> = Vec::new();
            while !vals_for_now.is_empty() {
                let v = vals_for_now.pop().unwrap();
                let digit_count = v.to_string().len() as u32;
                if v == 0 {
                    temp_vals.push(1);                    
                } else if digit_count % 2 == 0 {
                    temp_vals.push( v % (10 as u64).pow(digit_count/2) as u64);
                    temp_vals.push( v / (10 as u64).pow(digit_count/2) as u64);                    
                } else {
                    temp_vals.push(v * 2024);
                }                              
            }
            vals_for_now = temp_vals;
            println!("vals_for_now is {:?}", vals_for_now);
        }
        total_digit_count += vals_for_now.len() as u64;
    }
    
    total_digit_count
  
}

fn part_two(data: Vec<u64>) -> u64 {
    let mut total_digit_count = 0;

    let mut all_stone_values: HashMap<u64, u64> = HashMap::new();
    for key in data {
        all_stone_values.insert(key,1);
    }

    for i in 1..76
     {    
        //println!("Starting blink {}  have seen {} unique values", i, all_stone_values.keys().len());
        let temp_all_stones = all_stone_values.clone();
        for (key,val) in temp_all_stones.iter(){
            if *val == 0 {
                continue;
            }
            //println!("working the key {}...", key);
            let orig_instance_count = *val;
            let mut new_vals_to_add: Vec<u64> = Vec::new();
            let digit_count = key.to_string().len() as u32;
            if *key==0 {
                new_vals_to_add.push(1);
            } else if digit_count % 2 == 0 {
                let first_new_val = key % (10 as u64).pow(digit_count/2) as u64;
                let second_new_val = key / (10 as u64).pow(digit_count/2) as u64;
                new_vals_to_add.push(first_new_val);
                new_vals_to_add.push(second_new_val);
            } else {
                new_vals_to_add.push(key * 2024);
            }
        
            for val in new_vals_to_add {
                //println!("    -> adding {} copies of  {}", orig_instance_count, val);
                all_stone_values.entry(val).and_modify(|v| *v += orig_instance_count).or_insert(orig_instance_count); 
            }
            //println!("    -> removing {} copies of {}", orig_instance_count, key);
            all_stone_values.entry(*key).and_modify(|v| *v -= orig_instance_count).or_insert(0);         
        }

        total_digit_count = 0;
        for v in all_stone_values.values() {
            total_digit_count += v;
        }
        println!("After blink {} we see {} total stones with {} unique values", i, total_digit_count, all_stone_values.keys().len());
   
    }
    
    total_digit_count = 0;
    for v in all_stone_values.values() {
        total_digit_count += v;
    }
    total_digit_count
}


fn main() {
    let input = "./src/input.txt";     

    match read_lines(input) {
        Ok(value) => {     
            let mut data: Vec<u64> = Vec::new();   

            for line in value {
                println!("{:?}", line);
                let this_data: Vec<u64> = line.split(' ').map(|c| c.parse().expect("failed to parse u32")).collect();
                data = this_data;
            }
            //println!("Answer to part one:  {}", part_one(data));
            println!("Answer to part two:  {}", part_two(data));
        },
        Err(e) => println!("Error: {}", e),
    }
}
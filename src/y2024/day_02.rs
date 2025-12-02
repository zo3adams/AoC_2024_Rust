
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


fn is_safe(row:&Vec<i32>) -> bool {
    println!("{:?}", row);
    let mut increasing = true;
    if row[0] > row[1] {
        increasing = false
    }
    let mut last_entry = row[0];
    let mut is_safe = true;
    for entry in &row[1..] {
        if *entry == last_entry {
            println!("not safe - found equality {} == {}", entry, last_entry);
            is_safe = false;
            break;
        }

        if *entry > last_entry && !increasing {
            println!("not safe - found a decrease {} -> {}", entry, last_entry);
            is_safe = false;
            break;
        }

        if *entry < last_entry && increasing {
            println!("not safe - found a decrease {} -> {}", entry, last_entry);
            is_safe = false;
            break;
        }

        if 3 < (entry - last_entry).abs() {
            println!("not safe - found excessive jump  {} -> {}", entry, last_entry);
            is_safe = false;
            break;
        }

        last_entry = *entry;
    }
    is_safe
}


fn _part_one(data: Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for row in data {    
        if is_safe(&row) {
            safe_count = safe_count + 1;
        }
    }
    println!("Found {} safe rows", safe_count);
    safe_count

}

fn part_two(data: Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;

    for row in data {

        if is_safe(&row) {
            safe_count = safe_count + 1;
            println!(" was safe on first try");
            continue;
        }

        //try removing each index and testing.
        let mut removal_index:usize = 0;
        let mut was_safe = false;
        while removal_index < row.len() {

            //make copy without that index
            let mut new_row:Vec<i32> = Vec::new();
            let mut index:usize = 0;
            while index < row.len() {
                if index != removal_index {
                    new_row.push(row[index]);
                }    
                index = index + 1;            
            }
            println!("{:?}", new_row);
            removal_index = removal_index + 1;
            //check for safety
            if is_safe(&new_row) {
                was_safe = true;
                break;
            }
            
        }
        if was_safe {
            safe_count = safe_count + 1;
        }     
    }

    println!("Found {} safe rows", safe_count);
    safe_count
}

pub fn run()-> io::Result<()> {

    let input = "./src/input.txt";

    let mut data: Vec<Vec<i32>> = Vec::new();
           
    match read_lines(input) {
        Ok(value) => {    
                 
            for line in value {
                //println!("{}", line);
                let ints_in_str:Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().expect("parse error")).collect();
                //println!("{:#?}", ints_in_str);
                data.push(ints_in_str);                
            }
        },
    
        Err(e) => println!("Error: {}", e),
    }
    //part_one(data);
    part_two(data);

    Ok(())

}


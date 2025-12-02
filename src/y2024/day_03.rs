
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


fn _is_safe(row:&[i32]) -> bool {
    row.windows(2).all(|window| {
        let diff = window[1] - window[0];
        diff.abs() <= 3 && 
        (window[0] < window[1] || 
         window[0] > window[1])
    }) && 
    row.iter().all(|&x| row.iter().filter(|&&y| y == x).count() == 1)
}


fn extract_digits(input: &str) -> u32 {
    let as_digits: Vec<u32> = input.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let concatenated = as_digits.iter()
        .map(|&num| num.to_string())
        .collect::<String>();
    concatenated.parse::<u32>().expect("Parse error?")
}

fn _part_one(data: Vec<String>) -> u32 {

    let mut total: u32 = 0;
    
    for operand in data {
        print!("{}", operand);
        let parts: Vec<&str> = operand.split(',').collect();
        let left_hand = extract_digits(parts[0]);
        let right_hand = extract_digits(parts[1]);

        let product = left_hand * right_hand;
        println!(" adding {}*{} =  {} to total ({})", left_hand, right_hand, product, total);
        total = total + product;
        }
    total

}


fn part_two(data: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    let mut do_enabled: bool = true;
    
    for operand in data {

        if operand == "do()" {
            do_enabled = true;
            println!("now on od");
            continue;
        }
 
        if operand == "don't()" {
            do_enabled = false;
            println!("now on don't");
            continue;
        }

        if do_enabled {
            let parts: Vec<&str> = operand.split(',').collect();
            let left_hand = extract_digits(parts[0]);
            let right_hand = extract_digits(parts[1]);

            let product = left_hand * right_hand;
            println!(" adding {}*{} =  {} to total ({})", left_hand, right_hand, product, total);
            total = total + product;
        } else {
            
        }
        
        }
    total
 
}

pub fn run()-> io::Result<()> {

    //ran zsh command below to filter for part one
    // cat sample_input.txt | grep -ohE 'mul\([0-9]{1,3},[0-9]{1,3}\)' > updated_sample_input.txt

    //and for part two:
    //cat sample_input.txt | grep -ohE "mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)" > updated_sample_input.txt
    let input = "./src/updated_input.txt";          
    match read_lines(input) {
        Ok(value) => {    

            println!("Got total {}", part_two(value));
                 
        },
    
        Err(e) => println!("Error: {}", e),
    }
  
  Ok(())

}

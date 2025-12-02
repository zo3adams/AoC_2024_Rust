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

fn _part_one(mut right_column:Vec<i32>, mut left_column:Vec<i32>) -> i32 {
    right_column.sort();
    left_column.sort();

    let mut index:usize = 0;
    let mut sum_of_difference:i32 = 0;
    while index < right_column.len() {
        let diff = (right_column[index] - left_column[index]).abs();
        println!(" Diff of {} and {} is {}",right_column[index], left_column[index],  diff);
        sum_of_difference = sum_of_difference + diff;
        index = index +1;
    }

    println!("Total sum of differences was {}", sum_of_difference);
    sum_of_difference

}

fn part_two(right_column:Vec<i32>, left_column:Vec<i32>) -> i32 {

    let mut sim_score:i32 = 0;


    for left_val in left_column {
        let instances:i32 = right_column.iter().filter(|&n|  *n == left_val).count() as i32;
        let sim_addition = left_val * instances;
        println!(" There are {} instances of value {} so adding {}", instances, left_val, sim_addition);
        sim_score = sim_score + sim_addition;
    }

    println!("Total similarity score {}", sim_score);
    sim_score
}

pub fn run()-> io::Result<()> {

    let input = "./src/input.txt";

    let mut right_column: Vec<i32> = Vec::new();
    let mut left_column: Vec<i32> = Vec::new();
           
    match read_lines(input) {
        Ok(value) => {    
                 
            for line in value {
                //println!("{}", line);
                let ints_in_str:Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().expect("parse error")).collect();
                //println!("{:#?}", ints_in_str);

                left_column.push(ints_in_str[0]);
                right_column.push(ints_in_str[1]);
                
            }
        },
    
        Err(e) => println!("Error: {}", e),
    }
    //part_one(right_column, left_column);
    part_two(right_column, left_column);

     Ok(())
}


use std::fs::File;
use std::cmp::min;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use colored::Colorize;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}


// a*x_a + b*x_b = prize+x,    a*y_a + b * b_y = prize_y,  derivative  3a + 1b = 0 to find min
fn part_one(a_positions: Vec<(i32,i32)>, b_positions: Vec<(i32,i32)>, prize_locations: Vec<(i32,i32)>) -> i32 {
let mut min_tokens_sum = 0;

for i in 0..a_positions.len()  {
    
    let mut min_cost_seen = i32::MAX;

    let target_x = prize_locations[i].0;
    let target_y = prize_locations[i].1;

    println!("searching for A:{},{}   B:{},{},  Prize:{},{}", a_positions[i].0, a_positions[i].1,b_positions[i].0, b_positions[i].1, target_x, target_y );

    let max_a_presses = min(101, min(1 + (target_x / a_positions[i].0), 1 + (target_y / a_positions[i].1)));
    let max_b_presses = min(101,min( 1+ (target_x / b_positions[i].0), 1 + (target_y / b_positions[i].1)));

    for j in 0..max_a_presses {
        for k in 0..max_b_presses {
            if (j * a_positions[i].0 + k * b_positions[i].0) == target_x && (j * a_positions[i].1 + k * b_positions[i].1) == target_y {
                let cost = 3*j + 1 *k;

                if cost < min_cost_seen {
                    println!("{} {}  and {} will hit target and {}", "Found!".green(), format!("{} presses of A",j).red(), 
                        format!("{} presses of B", k).blue(), 
                        format!(" cost will be {}", cost).bold().yellow());
                    min_cost_seen = cost;
                }

            }
         }
    }
    if min_cost_seen != i32::MAX {
        min_tokens_sum += min_cost_seen;
    }

}

min_tokens_sum
}


// too large for simple addition, try Craners rule
fn part_two(a_positions: Vec<(i32,i32)>, b_positions: Vec<(i32,i32)>, prize_locations: Vec<(i32,i32)>) -> i64 {
    let mut min_tokens_sum = 0;
    
    for i in 0..a_positions.len()  {   
    
        let target_x: i64 = prize_locations[i].0 as i64 + 10000000000000;
        let target_y: i64 = prize_locations[i].1 as i64 + 10000000000000;

        let a_x = a_positions[i].0 as i64;
        let a_y = a_positions[i].1 as i64;
        let b_x = b_positions[i].0 as i64;
        let b_y = b_positions[i].1 as i64;
    
        println!("searching for A:{},{}   B:{},{},  Prize:{},{}", a_positions[i].0, a_positions[i].1,b_positions[i].0, b_positions[i].1, target_x, target_y );
        let CRAMERS_A = (target_x * b_y - target_y*b_x) / (a_x*b_y - a_y*b_x);
        let CRAMERS_B = (a_x*target_y - a_y*target_x) / (a_x*b_y - a_y*b_x);
        //check if it's valid
        if (CRAMERS_A * a_x + CRAMERS_B * b_x) == target_x && (CRAMERS_A * a_y + CRAMERS_B * b_y) == target_y {
            let cost = 3*CRAMERS_A + 1 * CRAMERS_B;
            println!("{} {}  and {} will hit target and {}", "Found!".green(), format!("{} presses of A",CRAMERS_A).red(), format!("{} presses of B", 
            CRAMERS_B).blue(), 
            format!(" cost will be {}", cost).bold().yellow());
            min_tokens_sum += cost;
        }

    }
    
    min_tokens_sum
}


fn main() {
    let input = "./src/input.txt";     
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    match read_lines(input) {
        Ok(value) => {    

            
            let mut a_additions: Vec<(i32, i32)> = Vec::new();
            let mut b_additions: Vec<(i32, i32)> = Vec::new();
            let mut prize_locations: Vec<(i32, i32)> = Vec::new();

            let all_lines_combined = value.join("\n");
            //println!("{}", all_lines_combined);
            for (index, captures) in re.captures_iter(&all_lines_combined).enumerate() {
                let x: i32 = captures[1].parse().unwrap();
                let y: i32 = captures[2].parse().unwrap();

                //println!("x,y {},{}", x, y);
                let index_mod_three = index % 3;
        
                match index_mod_three {
                    0 => a_additions.push((x, y)),        // First match for A additions
                    1 => b_additions.push((x, y)),        // Second match for B additions
                    2 => prize_locations.push((x, y)),     // Third match for Prize location
                    _ => {} // cannot see with modulus
                }
            }
            //println!("{:?}", a_additions);
            //println!("{:?}", b_additions);
            //println!("{:?}", prize_locations);

            if a_additions.len() != b_additions.len() || b_additions.len() != prize_locations.len() {
                println!("Parse error, got unequal list sizes?");
            }

         
            //println!("Answer to part one:  {}", part_one(a_additions, b_additions, prize_locations));
            println!("Answer to part two:  {}", part_two(a_additions, b_additions, prize_locations));
        },
        Err(e) => println!("Error: {}", e),
    }
}
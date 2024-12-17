use std::fs::File;
use std::cmp::min;
use std::i32;
use std::collections::HashMap;
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


fn print_robot_positions(robot_positions: Vec< (i32,i32,i32,i32)>, space_width: usize, space_height: usize) {

    let mut v: Vec<Vec<char>> = vec![vec!['.'; space_width]; space_height];
    
    let mut max_overlap_seen = 0;

    for robot in robot_positions {

       if  v[robot.1 as usize][robot.0 as usize] == '.' {
           v[robot.1 as usize][robot.0 as usize] = '1'
       }else {
           
           let robot_count = 1 + v[robot.1 as usize][robot.0 as usize].to_digit(10).expect("failed to parse to digit"); 

           if robot_count > max_overlap_seen {
            max_overlap_seen = robot_count;
            println!("See new max of {} robots on one square", max_overlap_seen);
           }
           v[robot.1 as usize][robot.0 as usize] = char::from_digit(robot_count,10).unwrap();
       }
    }

    for row in 0..space_height {
        for col in 0..space_width {
            //uncomment to only show quadrants
            //if row == space_height/2 || col == space_width/2 {
            //    print!(" ");
            //} else {
            print!("{}", v[row][col]);
            //}                             
        }
        println!("");
    }

}


// p=2,4 v=2,-3    4,1    6,5    8,2,    10,6     1,3  
fn part_one(robot_positions: &mut Vec< (i32,i32,i32,i32)>, space_width: usize, space_height: usize) -> u32 {
    let iterations = 100;

    let mut calc_final_positions: Vec<(i32, i32)> = Vec::new();

    for i in 0..robot_positions.len() {
        let final_pos_x = (((robot_positions[i].0 + iterations * robot_positions[i].2) % space_width as i32) + space_width as i32 ) % space_width as i32;
        let final_pos_y = (((robot_positions[i].1 + iterations * robot_positions[i].3) % space_height as i32) + space_height as i32) % space_height as i32;
        calc_final_positions.push((final_pos_x, final_pos_y));
    }

    for i in 0..iterations {
        for robot in &mut * robot_positions {
            robot.0 = (((robot.0 + robot.2) % space_width as i32) + space_width as i32 ) % space_width as i32;
            robot.1 = (((robot.1 + robot.3) % space_height as i32) + space_height as i32) % space_height as i32;
            //println!("new positions: {},{}", robot.0, robot.1);
        }
    }

    let mut quad_counts : Vec<u32> = vec![0;4];
    for i in 0..robot_positions.len() {
        if robot_positions[i].0 != calc_final_positions[i].0 || robot_positions[i].1 != calc_final_positions[i].1 {
            println!("Found a mismatch in calculated and iterated positions!!  iter: {},{},  but calced {},{}", robot_positions[i].0, robot_positions[i].1, calc_final_positions[i].0, calc_final_positions[i].1);
        }

        let x_cutoff: i32 = space_width as i32 / 2;
        let y_cutoff: i32 = space_height as i32 / 2;

        if robot_positions[i].0 < x_cutoff && robot_positions[i].1 < y_cutoff{
            quad_counts[0] += 1;
        } else if robot_positions[i].0 < x_cutoff && robot_positions[i].1 > y_cutoff {
            quad_counts[1] += 1;
        } else if robot_positions[i].0 > x_cutoff && robot_positions[i].1 < y_cutoff {
            quad_counts[2] += 1;
        } else if robot_positions[i].0 > x_cutoff  && robot_positions[i].1 > y_cutoff {
            quad_counts[3] += 1;
        }
    }
    
    print_robot_positions(robot_positions.to_vec(), space_width, space_height);
    println!("q counts {:?}", quad_counts);
    quad_counts.iter().product()
}

fn part_two(robot_positions: &mut Vec< (i32,i32,i32,i32)>, space_width: usize, space_height: usize) -> u32 {
    let iterations: u64 = 100000000000;

    for i in 0..iterations {

        let mut bots_per_row: HashMap<i32, Vec<i32>> = HashMap::new();
        
        for robot in &mut * robot_positions {
            robot.0 = (((robot.0 + robot.2) % space_width as i32) + space_width as i32 ) % space_width as i32;
            robot.1 = (((robot.1 + robot.3) % space_height as i32) + space_height as i32) % space_height as i32;   

            bots_per_row.entry(robot.1).and_modify(|val| val.push(robot.0)).or_insert(vec![robot.0]);                
        }
        
        for (k,v) in bots_per_row.iter() {

            let mut sorted_version = v.clone();
            sorted_version.sort();

            let mut in_a_row = 0;
            let mut last_row = 0;

            for this_row in sorted_version {
                if this_row == (last_row + 1) {
                    in_a_row += 1;                                        
                } else {
                    in_a_row = 0;
                }

                last_row = this_row;
            }

            if in_a_row > 10 {          
                print_robot_positions(robot_positions.to_vec(), space_width, space_height);
                println!("Saw more than 10 robots lined up at {} seconds", i+1);
            }
        }
    }

    0
}


fn main() {
    let input = "./src/input.txt";  
    let space_width: usize = 101;
    let space_height: usize =  103;

    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();

    match read_lines(input) {
        Ok(value) => {    

            let all_lines_combined = value.join("\n");
            let mut robot_positions: Vec< (i32,i32,i32,i32)> = Vec::new();
            //println!("{}", all_lines_combined);
            for (_index, captures) in re.captures_iter(&all_lines_combined).enumerate() {

                let nums: Vec<i32> = captures.iter()
                    .skip(1) // Skip the full match
                    .filter_map(|m| m.map(|x| x.as_str().parse::<i32>().unwrap()))
                    .collect();

                robot_positions.push((nums[0],nums[1],nums[2],nums[3]));
                }
           
           
                //println!("Answer to part one:  {}", part_one(&mut robot_positions, space_width, space_height));
                println!("Answer to part two:  {}", part_two(&mut robot_positions, space_width, space_height));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        
    }
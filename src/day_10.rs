
use std::collections::HashSet;
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


fn find_paths(data: &Vec<Vec<u32>>, starting_h:usize, starting_w:usize) -> Vec<Vec<(usize,usize)>> {
    let max_height:i32 = data.len() as i32;
    let max_width:i32 = data[0].len() as i32;
    let directions: Vec<(i32, i32)> = vec![(0,1), (0,-1), (1,0), (-1,0)];

    let mut exploration_stack: Vec<Vec<(usize,usize)>> = Vec::new();
    exploration_stack.push(vec![(starting_h,starting_w)]);

    let mut found_paths: Vec<Vec<(usize,usize)>> = Vec::new();
    //println!(" In DFS for trailhead starting at pos {},{}", starting_h, starting_w);

    while !exploration_stack.is_empty() {

        let in_prog_path = exploration_stack.pop().unwrap();


        for direction in directions.iter() {
            let current_position = in_prog_path.last().unwrap();
            let new_h: i32 = current_position.0  as i32 + direction.0;
            let new_w: i32 = current_position.1  as i32 + direction.1;

            //println!("    directon {},{} gave us new position {},{}", direction.0, direction.1, new_h, new_w);
            
     
            if (new_h >= 0) && (new_h < max_height) {
                if (new_w >= 0) && (new_w < max_width) {
                    
                    if (data[current_position.0][current_position.1]+1) == data[new_h as usize][new_w as usize] {                        

                        let mut new_path = in_prog_path.clone();
                        new_path.push((new_h as usize, new_w as usize));
                        //println!("      new_path is now {:?}  ending in {} ", new_path, data[new_h as usize][new_w as usize]);
                        if data[new_h as usize][new_w as usize] == 9 {
                            //found a full path
                            println!("      found full path ending at 9 {:?} ", new_path);
                            found_paths.push(new_path);
                        } else {

                            exploration_stack.push(new_path);
                        }   


                    }                    
                }
             }
        }
    }
    return found_paths;
}


fn part_one(data: Vec<Vec<u32>>) -> u64 {
    let max_height:usize = data.len();
    let max_width:usize = data[0].len();

    let mut total_score = 0;

    println!("{:?}", data);

    for i in 0..max_height {
        for j in 0..max_width {
            if data[i][j] == 0 {
                println!("Found a trail head at pos {}, {}", i,j);
                let found_paths = find_paths(&data, i, j);
                println!("Found {} paths at position {},{}", found_paths.len(), i,j);

                let mut end_points: HashSet<(usize,usize)> = HashSet::new();

                for path in found_paths.iter() {
                        end_points.insert(*path.last().unwrap());
                        println!("trail head starting at {},{} can reach {} unique 9 squares", i,j, end_points.len());
                }
                total_score += end_points.len();
            }

        }
    }

   total_score as u64
  
}

fn part_two(data: Vec<Vec<u32>>) -> u64 {
    let max_height:usize = data.len();
    let max_width:usize = data[0].len();

    let mut total_score = 0;

    println!("{:?}", data);

    for i in 0..max_height {
        for j in 0..max_width {
            if data[i][j] == 0 {
                println!("Found a trail head at pos {}, {}", i,j);
                let found_paths = find_paths(&data, i, j);
                println!("Found {} paths at position {},{}", found_paths.len(), i,j);

                total_score += found_paths.len();
            }

        }
    }

   total_score as u64
}


fn main() {
    let input = "./src/input.txt";     

    match read_lines(input) {
        Ok(value) => {     
            let mut data: Vec<Vec<u32>> = Vec::new();   

            for line in value {
                let this_data: Vec<u32> = line.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect();
                data.push(this_data);
            }

            println!("Answer to part two:  {}", part_two(data));
        
        },
        Err(e) => println!("Error: {}", e),
    }

    
}
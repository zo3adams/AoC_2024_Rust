
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}


fn cycle_or_exit(map:&Vec<Vec<char>>, max_steps: u32) -> (u32, HashSet<(i32,i32)>) {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let directions: Vec<(i32,i32)> = vec![(-1,0), (0,1), (1,0), (0,-1)];
    let mut current_direction_index = 0;

    let mut current_position = map.iter().enumerate().flat_map(|(i, vec)| vec.iter().position(|&c| c == '^').map(|j| (i as i32, j as i32))).next().expect("reason");

    //println!("Starting position {:?} in a  {} by {} matrix", current_position, height, width);
    let mut squares_seen: HashSet<(i32,i32)> = HashSet::new();
    let mut steps_taken = 0;

    while current_position.0 > 0 && current_position.1 > 0 && current_position.0 < width && current_position.1 < height {
        //print!("{:?}", current_position);
        squares_seen.insert(current_position);
        let next_position_0 = current_position.0 as i32 + directions[current_direction_index].0;
        let next_position_1= current_position.1 as i32 + directions[current_direction_index].1;

        if next_position_0 >= 0 && next_position_0 < width && next_position_1 >= 0 && next_position_1 < height {
            //print!(" next square has a {}", map[next_position_0 as usize][next_position_1 as usize]);
            if map[next_position_0 as usize][next_position_1 as usize] == '#' {
                //rotate
                current_direction_index += 1;
                if current_direction_index >= directions.len() {
                    current_direction_index = 0;
                }
                //println!("-> rotating, now facing {:?}", directions[current_direction_index]);
                continue;
            }
        } 
        current_position.0 = next_position_0;
        current_position.1 = next_position_1;
        steps_taken = steps_taken + 1;
        if steps_taken > max_steps {
            return (steps_taken, squares_seen);
        }
        //println!("-> updating position, , now at  {:?}", current_position);
    }
    (squares_seen.len() as u32, squares_seen)
}


fn part_one(map: &mut Vec<Vec<char>>) -> u32 {
    let result = cycle_or_exit(&map, 1000000);
    result.0
}

fn part_two(map: &mut Vec<Vec<char>>) -> u32 {

    let mut possible_obstructions = 0;
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    //get iniital path as candidates for blockage
    let initial_result = cycle_or_exit(&map, 1000000);

    println!("Blocks that created a loop:");

    //for each square that isn't already obstructed in initial path
    for position in initial_result.1 {
        //obstruct
        let orig_char = map[position.0 as usize][position.1 as usize];
        if map[position.0 as usize][position.1 as usize] != '^' && map[position.0 as usize][position.1 as usize] != '#' {
            //print!("blocking off {},{}   ", i,j);
            map[position.0 as usize][position.1 as usize] = '#';
        }
        else {
            //println!("skipping {},{} - was already blocked.  ", i,j);
            continue;
        }

        //detect if loop (steps exceeds total number of squares)
        let this_result = cycle_or_exit(&map, (width*height) as u32);
        //println!(" -> completed test in {} steps", steps_used);

        //count
        if this_result.0 > (width*height) as u32 {
            print!("({},{}), ", position.0,position.1);
            possible_obstructions = possible_obstructions + 1;
        }

        map[position.0 as usize][position.1 as usize] = orig_char;
    }    
    println!("Done with part two");
    possible_obstructions
}


pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";     
    let mut map:Vec<Vec<char>>;
    match read_lines(input) {
        Ok(value) => { 
            map = value.iter().map(|s| s.chars().collect()).collect();   
            println!("{:?}", map);
            println!("Answer to part one {}", part_one(&mut map));     
            println!("Answer to part two {}", part_two(&mut map));      
        },
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
    
}
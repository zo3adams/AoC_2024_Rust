
use std::collections::{HashMap, VecDeque};
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



fn part_one(map: Vec<Vec<char>>) -> u32 {
    let mut total_fence_cost: u32 = 0;

    let max_h = map.len();
    let max_w = map[0].len();
    let directions: Vec<(i32, i32)> = vec![(1,0), (-1,0), (0,1), (0,-1)];

    //plant type, area, and perim for each region.
    let mut regions: HashMap<(usize,usize), (char,u32,u32)> = HashMap::new();
    let mut region_members: HashMap<(usize,usize), Vec<(usize,usize)>> = HashMap::new();
    let mut squares_visited: HashMap<(usize,usize), (usize,usize)> = HashMap::new();

    for i in 0..max_h {
        for j in 0..max_w {

            if squares_visited.contains_key(&(i,j)) {
                continue;
            }
            //print!("  seeing {} at {},{}...  ",map[i][j], i,j);

            //new region - add list of one
            //region_members.insert((i,j), vec![(i,j)]);
            

            let mut queue: VecDeque<(usize,usize)> = VecDeque::new();
            queue.push_back((i,j));
            while let Some(next_square) = queue.pop_front() {

                if !squares_visited.contains_key(&(next_square.0,next_square.1)) {
                    //print!("  seeing {} at {},{}...  ", map[next_square.0][next_square.1], next_square.0,next_square.1);
                    squares_visited.insert((next_square.0,next_square.1), (i,j)); //visited, saving root but not using anywhere 
                    region_members.entry((i,j)).and_modify(|v| v.push((next_square.0,next_square.1))).or_insert(vec![(next_square.0,next_square.1)]); 

                    for direction in directions.iter() {
                        let new_h: i32 = next_square.0 as i32 + direction.0 ; 
                        let new_w: i32 = next_square.1 as i32 + direction.1;
                        if new_h < 0 || new_w < 0 || new_h as usize >= max_h || new_w as usize >= max_w {
                            //outside bounds case,  no further searching
                            continue;
                        } else if map[i][j] == map[new_h as usize][new_w as usize] {
                            queue.push_back((new_h as usize, new_w as usize));        
                        } 
                    }
                    

                }  
            }
            //print!("\n  added {} squares  ", region_members[&(i,j)].len());
            println!("Region exhausted {:?}", region_members[&(i,j)]);
        }
    }

    //calc area and perim for regions
    for (region_start, region_members) in region_members.iter() {

        let region_area: u32 = region_members.len() as u32;
        let mut region_perimiter: u32 = 0;

        for (i,j) in region_members {
            //search neighbors for perim            
            let mut this_perim = 0;
            for direction in directions.iter() {
                let new_h: i32 = *i as i32 +direction.0 ;
                let new_w: i32 = *j as i32 + direction.1;
                if new_h < 0 || new_w < 0 || new_h as usize >= max_h || new_w as usize >= max_w {
                    //outside bounds case,  fence is required
                    this_perim += 1;
                } else if map[*i][*j] != map[new_h as usize][new_w as usize] {
                    this_perim += 1;
                }                
            }
            region_perimiter += this_perim;
        }
        regions.insert(*region_start, (map[region_start.0][region_start.1], region_area, region_perimiter));
    }
  
    for (k,v) in regions.iter() {        
        let area: u32 = v.1;
        let perimiter: u32 = v.2;
        let total_cost: u32 = area * perimiter;

        println!("A region of {} plants with price {} * {} = {}.   Starting at  {}, {}", v.0, area, perimiter, total_cost, k.0,k.1);
        total_fence_cost +=  total_cost;
    }

    total_fence_cost

}

fn part_two(map: Vec<Vec<char>>) -> u32 {
    let mut total_fence_cost: u32 = 0;

    let max_h = map.len();
    let max_w = map[0].len();
    let directions: Vec<(i32, i32)> = vec![(1,0), (-1,0), (0,1), (0,-1)];

    //plant type, area, and perim for each region.
    let mut regions: HashMap<(usize,usize), (char,u32,u32)> = HashMap::new();
    let mut region_members: HashMap<(usize,usize), Vec<(usize,usize)>> = HashMap::new();
    let mut squares_visited: HashMap<(usize,usize), (usize,usize)> = HashMap::new();

    for i in 0..max_h {
        for j in 0..max_w {

            if squares_visited.contains_key(&(i,j)) {
                continue;
            }
            //print!("  seeing {} at {},{}...  ",map[i][j], i,j);

            //new region - add list of one
            //region_members.insert((i,j), vec![(i,j)]);
            

            let mut queue: VecDeque<(usize,usize)> = VecDeque::new();
            queue.push_back((i,j));
            while let Some(next_square) = queue.pop_front() {

                if !squares_visited.contains_key(&(next_square.0,next_square.1)) {
                    //print!("  seeing {} at {},{}...  ", map[next_square.0][next_square.1], next_square.0,next_square.1);
                    squares_visited.insert((next_square.0,next_square.1), (i,j)); //visited, saving root but not using anywhere 
                    region_members.entry((i,j)).and_modify(|v| v.push((next_square.0,next_square.1))).or_insert(vec![(next_square.0,next_square.1)]); 

                    for direction in directions.iter() {
                        let new_h: i32 = next_square.0 as i32 + direction.0 ; 
                        let new_w: i32 = next_square.1 as i32 + direction.1;
                        if new_h < 0 || new_w < 0 || new_h as usize >= max_h || new_w as usize >= max_w {
                            //outside bounds case,  no further searching
                            continue;
                        } else if map[i][j] == map[new_h as usize][new_w as usize] {
                            queue.push_back((new_h as usize, new_w as usize));        
                        } 
                    }
                    

                }  
            }
            print!("\n  added {} squares  ", region_members[&(i,j)].len());
            println!("Region exhausted {:?}", region_members[&(i,j)]);
        }
    }

    //calc area and perim for regions
    for (region_start, region_members) in region_members.iter() {

        let region_area: u32 = region_members.len() as u32;
        let mut region_perimiter: u32 = 0;  
        let mut sides_by_direction: HashMap<(i32, i32, usize), Vec<usize>>  = HashMap::new();

        println!(" ====== begining perimiter eval  ======  ");
        
        for (i,j) in region_members {
            //search neighbors for perim            
            
            println!("checking on square {},{}", i,j);
    
            //put all sides in a map, keyed by the direction they are facing and position orthognal to that direction    (dir.0, dir.1, pos_for_dir_element_that_is_zero): (pos_for_dir_that_is_non_zero)            
            for direction in directions.iter() {
                //is even a perim?
                let mut is_a_perim = false;
                let new_h: i32 = *i as i32 +direction.0 ;
                let new_w: i32 = *j as i32 + direction.1;
                if new_h < 0 || new_w < 0 || new_h as usize >= max_h || new_w as usize >= max_w {
                    //outside bounds case,  fence is required
                    is_a_perim = true;
                } else if map[*i][*j] != map[new_h as usize][new_w as usize] {
                    is_a_perim = true;
                }  

                if is_a_perim {
                    println!("we see a perimeter at {},{}  facing {},{}  -- adding to our collection", i,j, direction.0, direction.1);
                    if direction.0 == 0 {      
                        sides_by_direction.entry((direction.0, direction.1, *j)).and_modify(|v| v.push((*i))).or_insert(vec![(*i)]); 
                        println!("  collection at ({},{},{}) grew to {:?}",direction.0, direction.1, *j, sides_by_direction[&(direction.0, direction.1, *j)]);
                    } else if direction.1 == 0 {
                        sides_by_direction.entry((direction.0, direction.1, *i)).and_modify(|v| v.push((*j))).or_insert(vec![(*j)]); 
                        println!("  collection at {},{},{}  grew to {:?}", direction.0, direction.1, *i, sides_by_direction[&(direction.0, direction.1, *i)]);
                    }
                }
            }
        }

        //go over each entry and count continuous (position tangent to direction is incremented by 1 when sorted) lines
        for (fence_section, positions) in sides_by_direction.iter() {
            println!("section facing {},{}  at orthogonal {} and tangent positions {:?} \n\n", fence_section.0, fence_section.1, fence_section.2, positions);
            let mut this_perim = 0;
            this_perim += 1;
            let copy_of_positions: Vec<usize> = {
                let mut copy = positions.clone(); // Create a copy of the original vector
                copy.sort(); // Sort the copy in ascending order
                copy // Return the sorted copy
            };
            
            let mut last_pos = copy_of_positions[0];
            for m in 1..copy_of_positions.len() {
                if copy_of_positions[m as usize] > (last_pos + 1) {
                    //add up continuous lines
                    this_perim += 1;
                }
                last_pos = copy_of_positions[m as usize];
            }  
            println!(" found {} contiguous blocks in fence section at {},{},{}", this_perim, fence_section.0, fence_section.1, fence_section.2);                
            region_perimiter += this_perim;
        }    
        regions.insert(*region_start, (map[region_start.0][region_start.1], region_area, region_perimiter));        
    }
    
  
    for (k,v) in regions.iter() {        
        let area: u32 = v.1;
        let perimiter: u32 = v.2;
        let total_cost: u32 = area * perimiter;

        println!("A region of {} plants with price {} * {} = {}.   Starting at  {}, {}", v.0, area, perimiter, total_cost, k.0,k.1);
        total_fence_cost +=  total_cost;
    }

    total_fence_cost
}


fn main() {
    let input = "./src/input.txt";     

    match read_lines(input) {
        Ok(value) => {     
            let mut map: Vec<Vec<char>> = value.iter().map(|s| s.chars().collect()).collect();   
            println!("{:?}", map);
            //println!("Answer to part one:  {}", part_one(map));
            println!("Answer to part two:  {}", part_two(map));
        },
        Err(e) => println!("Error: {}", e),
    }
}
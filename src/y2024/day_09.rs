
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}


fn get_expanded_view(data: Vec<u32>) -> Vec<i32>{
    let mut expanded_view:Vec<i32> = Vec::new();

   let mut is_file:bool = false;
   let mut file_index = 0;

   for digit in data {
    is_file = !is_file;
    if is_file {
        for _i in 0..digit{
            expanded_view.push(file_index);
        }
        file_index += 1;
    } else {
        for _i in 0..digit {
            expanded_view.push(-1);
        }
    }
       
   }

   expanded_view
}

fn _part_one(data: Vec<u32>) -> u64 {

    println!("input data has length {} implying {} unique files", data.len(), (data.len()+1)/2);
   //create initial expanded view
   let mut initial_expansion = get_expanded_view(data);
   println!("{:?}", initial_expansion);

    let mut last_valid_file: usize = initial_expansion.len() - 1;

   for i in 0..initial_expansion.len() {

        if i >= last_valid_file {
            println!("exiting because i and last valid have crossed");
            break;
        }

        if initial_expansion[i] == -1 {
            //swap with last char
            while initial_expansion[last_valid_file] == -1 {
                last_valid_file -= 1;
            }

            if last_valid_file > i { // only do the swap if we didn't JUST cross
                let temp = initial_expansion[i];
                initial_expansion[i] = initial_expansion[last_valid_file];
                initial_expansion[last_valid_file] = temp;            
            }
            //println!("{:?}      {}, {}", initial_expansion, i, last_valid_file);
        }
   }

   //calc checksum
   let mut checksum:u64 = 0;

   for index in 0..initial_expansion.len() {

        if initial_expansion[index] != -1 {
            let value_of_file_index = initial_expansion[index] as u64;
            checksum += (index as u64) * value_of_file_index;
        }
   }
   println!("{:?} has checksum  {}", initial_expansion, checksum);
   
   checksum
  
}

fn part_two(data: Vec<u32>) -> u64 {


     //get map of file indices and lengths
   let mut file_positions: HashMap<i32, (usize, usize)> = HashMap::new();
   let mut was_in_space = false;
   let mut file_index = 0;  
   let mut pos = 0;
    for digit in &data {
        
        if !was_in_space {
            //println!("adding file index {} with pos {} and length {} to map", file_index, pos, digit);
            file_positions.insert(file_index, (pos, *digit as usize));
            file_index += 1;
        } 
        was_in_space = !was_in_space;
        pos += *digit as usize;
    }

    println!("data: {:?}", data);
    //println!("input data has length {} implying {} unique files", data.len(), (data.len()+1)/2);
   //create initial expanded view
   let mut initial_expansion = get_expanded_view(data);
   println!("Expansion {:?}", initial_expansion);
   println!("Files {:?}", file_positions);
   //println!("Spaces {:?}", space_positions);

   let mut highest_file_index = *file_positions.keys().max().unwrap();

   while highest_file_index > 0 {

    //recalc space positions
    let mut space_index = -1;
    let mut pos_of_current_space = 0;
    let mut length_of_current_space = 0;
    was_in_space = false;
    let mut max_space_index = 0;
    let mut space_positions: HashMap<i32, (usize, usize)> = HashMap::new();
    for i in 0..initial_expansion.len() {
        if initial_expansion[i] != -1 && was_in_space {   //exiting a space         
            space_positions.insert(space_index, (pos_of_current_space, length_of_current_space));
            was_in_space = false;       

            if max_space_index < space_index {
                max_space_index = space_index;
            }    
        } else if initial_expansion[i] == -1 && was_in_space { //still in a space
            length_of_current_space += 1;
        } else if initial_expansion[i] == -1 && !was_in_space { //entering a space
            pos_of_current_space = i;
            length_of_current_space = 1;
            space_index += 1;
            was_in_space = true;
        }  //else is still not in a space, do no thing 
    }
    //println!("Spaces after shuffle {:?}", space_positions);

    for i in 0..max_space_index {
           //enough space to fit                                         and  is an actual move back
        if space_positions[&i].1 >= file_positions[&highest_file_index].1 &&  space_positions[&i].0 < file_positions[&highest_file_index].0 {
            println!("Moving file {} into space {}", highest_file_index, i);
            let start = space_positions[&i].0;
            let end = start + file_positions[&highest_file_index].1;
            let mut file_pos = file_positions[&highest_file_index].0;

            for j in start..end {
                let temp = initial_expansion[j];
                initial_expansion[j] = initial_expansion[file_pos];
                initial_expansion[file_pos] = temp; 
                file_pos += 1;
            }

            break;
        }
    }
    highest_file_index -= 1;
    //println!("{:?}    {}", initial_expansion,  highest_file_index);
   }
   
    //calc checksum
   let mut checksum:u64 = 0;
   for index in 0..initial_expansion.len() {

        if initial_expansion[index] != -1 {
            let value_of_file_index = initial_expansion[index] as u64;
            checksum += (index as u64) * value_of_file_index;
        }
   }
   println!("{:?} has checksum  {}", initial_expansion, checksum);
   
   checksum    
}


pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";     

    match read_lines(input) {
        Ok(value) => {     
            let mut data: Vec<u32> = Vec::new();   

            for line in value {
                let mut this_data: Vec<u32> = line.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect();
                data.append(&mut this_data);
            }

            println!("Answer to part two {}", part_two(data));
        
        },
        Err(e) => println!("Error: {}", e),
    }

    Ok(())  
}
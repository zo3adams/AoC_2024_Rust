
use std::fs::File;
use std::i32;
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

//takes in a primary keypad movement and returns the character stream to get there (not press, add A outside)
fn decode_position_change(initital_pos: (i32,i32), target_pos: (i32,i32), forbidden_cell: (i32,i32)) -> String {
    //println!("         decoding from {:?} to {:?}  with forbidden {:?}", initital_pos, target_pos, forbidden_cell);
    let row_diff = initital_pos.0 - target_pos.0;
    let row_moves = String::from(if row_diff > 0 { "^" } else { "v" }).repeat(row_diff.abs() as usize);
    let col_diff = initital_pos.1 - target_pos.1;
    let col_moves = String::from(if col_diff > 0 { "<" } else { ">" }).repeat(col_diff.abs() as usize);
    //prefer getting around forbidden cell first
    if target_pos.0 == forbidden_cell.0 && initital_pos.1  == forbidden_cell.1 {
        return format!("{}{}", col_moves, row_moves); // moving into forbidden row, from above/below,  so change cols first
    } else if target_pos.1 == forbidden_cell.1  && initital_pos.0 == forbidden_cell.0 {
        return format!("{}{}",row_moves, col_moves); // moving into forbidden column, from left/right, so change rows first
    } else {
        if col_moves.contains('<') { //since left is the most expensive move (farthest from A,)  do it first, next press will be closed to A
            return format!("{}{}", col_moves, row_moves); 
        } else {
            return format!("{}{}",row_moves, col_moves);
        }
    }
}

fn get_press_count(initital_pos: (i32,i32), target_pos: (i32,i32), depth_level: i32, results_cache: &mut HashMap<String, u64>,
 keypad_lookup: & HashMap<char,(i32,i32)>,  directional_lookup: & HashMap<char, (i32,i32)> ) -> u64 {

    let cache_key: String = format!("{:?}_{:?}_{}", initital_pos, target_pos, depth_level);
    if let Some(&ref result) = results_cache.get(&cache_key) {
        return *result; // Return cached result if available
    }
    let forbidden_cell = if depth_level == 0 {(3,0)} else { (0,0)};
    let path = format!("{}{}", decode_position_change(initital_pos, target_pos, forbidden_cell),"A");
    if depth_level == 25 {
        //println!("gpc is returning path {:?} for {:?} to {:?}", path, initital_pos, target_pos);
        path.len() as u64
    } else {
        let mut total_length:u64 = 0;
        let mut last_button = 'A';
        for next_button in path.chars() {
            let last_button_pos = directional_lookup[&last_button.clone()];
            let next_button_pos = directional_lookup[&next_button.clone()];
            
            let next_segment_length = get_press_count(last_button_pos, next_button_pos, depth_level+1, results_cache, keypad_lookup, directional_lookup);
            total_length += next_segment_length;
            last_button = next_button;
        }
        results_cache.insert(cache_key.clone(), total_length);
        total_length
}
}


fn _part_one(codes:  Vec<String>) -> i32 {
    println!("{:?}", codes);
    let mut total_complexity = 0;
    let mut primary_key_pad_positions: HashMap<char, (i32,i32)> = HashMap::new();
    primary_key_pad_positions.insert('7', (0,0));
    primary_key_pad_positions.insert('8', (0,1));
    primary_key_pad_positions.insert('9', (0,2));
    primary_key_pad_positions.insert('4', (1,0));
    primary_key_pad_positions.insert('5', (1,1));
    primary_key_pad_positions.insert('6', (1,2));
    primary_key_pad_positions.insert('1', (2,0));
    primary_key_pad_positions.insert('2', (2,1));
    primary_key_pad_positions.insert('3', (2,2));
    primary_key_pad_positions.insert('0', (3,1));
    primary_key_pad_positions.insert('A', (3,2));

    let mut directional_keypad: HashMap<char, (i32,i32)> = HashMap::new();
    directional_keypad.insert('^', (0,1));
    directional_keypad.insert('A', (0,2));
    directional_keypad.insert('<', (1,0));
    directional_keypad.insert('v', (1,1));
    directional_keypad.insert('>', (1,2));

    //println!("movement from {} to {} is {}   --- ({:?} to {:?}", i_char, j_char,  decode_position_change(primary_key_pad_positions[&i_char], primary_key_pad_positions[&j_char]), primary_key_pad_positions[&i_char], primary_key_pad_positions[&j_char]);
    let mut last_button_primary_key_pad;
    let mut last_button_first_directional_key_pad;
    let mut last_button_second_directional_key_pad;

    for code in codes {
        //unpack to movements on primary keypad
        let mut path: String = String::from("");
        last_button_primary_key_pad = 'A';
        for next_button in code.chars() {
            //println!("calling decode for {} to {}", last_button_primary_key_pad, next_button);
            path.push_str(&decode_position_change(primary_key_pad_positions[&last_button_primary_key_pad], 
                primary_key_pad_positions[&next_button],
            (3,0)));
            path.push('A');
            last_button_primary_key_pad = next_button;
        }
        
        let mut path_on_dir_one: String = String::from("");
        last_button_first_directional_key_pad = 'A';
        for next_button in path.chars() {
            //println!("calling directional decode for {} to {}", last_button_first_directional_key_pad, next_button);
            path_on_dir_one.push_str(&decode_position_change(directional_keypad[&last_button_first_directional_key_pad],
                 directional_keypad[&next_button],
                (0,0)));
            path_on_dir_one.push('A');
            last_button_first_directional_key_pad = next_button;
        }

        let mut path_on_dir_two: String = String::from("");
        last_button_second_directional_key_pad = 'A';
        for next_button in path_on_dir_one.chars() {
            //println!("calling directional decode for {} to {}", last_button_second_directional_key_pad, next_button);
            path_on_dir_two.push_str(&decode_position_change(directional_keypad[&last_button_second_directional_key_pad],
                 directional_keypad[&next_button],
                (0,0)));
            path_on_dir_two.push('A');
            last_button_second_directional_key_pad = next_button;
        }

        

        let final_sequence_length = path_on_dir_two.len() as i32;
        let numeric_portion: i32 = code.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().expect("Failed to parse numeric part");
        let complexity = final_sequence_length * numeric_portion;
        println!("code {} has complexity {}   ( {}*{} ) ", code,complexity, final_sequence_length, numeric_portion);

        //println!("   code {} has path {} on primary key pad", code, path);
        //println!("    code {} has path {} on first directional key pad", code, path_on_dir_one);
        //println!("     code {} has path {} on second directional key pad", code, path_on_dir_two);


        total_complexity += complexity;
    }
    total_complexity
}


//  from website <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
//  from my code v<<A>^>AvA^Av<A<AA>^>AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>^>AAA<Av>A^A
               
fn part_two(codes:  Vec<String>) -> u64 {
    println!("{:?}", codes);
    let mut total_complexity: u64 = 0;
    let mut primary_keypad_positions: HashMap<char, (i32,i32)> = HashMap::new();
    primary_keypad_positions.insert('7', (0,0));
    primary_keypad_positions.insert('8', (0,1));
    primary_keypad_positions.insert('9', (0,2));
    primary_keypad_positions.insert('4', (1,0));
    primary_keypad_positions.insert('5', (1,1));
    primary_keypad_positions.insert('6', (1,2));
    primary_keypad_positions.insert('1', (2,0));
    primary_keypad_positions.insert('2', (2,1));
    primary_keypad_positions.insert('3', (2,2));
    primary_keypad_positions.insert('0', (3,1));
    primary_keypad_positions.insert('A', (3,2));

    let mut directional_keypad_positions: HashMap<char, (i32,i32)> = HashMap::new();
    directional_keypad_positions.insert('^', (0,1));
    directional_keypad_positions.insert('A', (0,2));
    directional_keypad_positions.insert('<', (1,0));
    directional_keypad_positions.insert('v', (1,1));
    directional_keypad_positions.insert('>', (1,2));

    //println!("movement from {} to {} is {}   --- ({:?} to {:?}", i_char, j_char,  decode_position_change(primary_key_pad_positions[&i_char], primary_key_pad_positions[&j_char]), primary_key_pad_positions[&i_char], primary_key_pad_positions[&j_char]);
    let mut last_button;

    for code in codes {
        //unpack to movements on primary keypad
        let mut total_length:u64  = 0;
        last_button = 'A';

        let mut results_cache:HashMap<String, u64> = HashMap::new();

        for next_button in code.chars() {
            //println!("checking code {}", code);
            let last_button_pos = primary_keypad_positions[&last_button.clone()];
            let next_button_pos = primary_keypad_positions[&next_button.clone()];
            total_length += get_press_count(last_button_pos, next_button_pos, 0, &mut results_cache, &primary_keypad_positions, &directional_keypad_positions);
            last_button = next_button;
        }

        let numeric_portion: u64 = code.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().expect("Failed to parse numeric part");
        let complexity = total_length * numeric_portion;
        println!("code {} has complexity {}   ( {}*{} ) ", code,complexity, total_length, numeric_portion);
        total_complexity += complexity;
    }
    total_complexity
}



pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut codes: Vec<String> = Vec::new();
            for line in value {
                codes.push(line);
            } 
            //println!("Answer to part one:  {}", part_one( codes));
            println!("Answer to part two:  {}", part_two(codes));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        Ok(())

    }
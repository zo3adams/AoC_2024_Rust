
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
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


fn get_unique_chars(map: &mut Vec<Vec<char>>) -> HashMap<char, Vec<(usize,usize)>>  {
    let mut char_positions:HashMap<char, Vec<(usize,usize)>> = HashMap::new();

    let height = map.len();
    let width = map[0].len();

    for i in 0..height {
        for j in 0..width {
            if map[i][j] != '.' {

                if !char_positions.contains_key(&map[i][j]) {
                    char_positions.insert(map[i][j], Vec::new());
                }
                if let Some(list) = char_positions.get_mut(&map[i][j]) {
                    list.push((i,j));
                }
            }
           
        }
    }
    char_positions
}

fn part_one(map: &mut Vec<Vec<char>>) -> u32 {
    
    let height = map.len();
    let width = map[0].len();

    println!("working with matrix size {},{}", height, width);
    println!("{:?}", map);

    let char_positions:HashMap<char, Vec<(usize,usize)>> = get_unique_chars(map);
    println!("all chars... {:?}", char_positions.keys());

    let mut unique_antinodes = HashSet::new();

    for c in char_positions.keys() {
        println!("Working character {} positions {:?}", c, char_positions.get(c));

        let mut pairs = Vec::new();

        if let Some(list) = char_positions.get(c) {
            let length_of_list = list.len();
            for i in 0..length_of_list {
                for j in i + 1..length_of_list {
                    if i != j {
                        pairs.push((list[i], list[j]));
                    } 
                }
            }

            for pair in pairs {
                let h_diff: i32 = pair.0.0 as i32 - pair.1.0 as i32;
                let w_diff: i32 = pair.0.1 as i32 - pair.1.1 as i32;

                let antinode_1_0 = pair.0.0 as i32 + h_diff;
                let antinode_1_1 = pair.0.1 as i32 + w_diff;

                if antinode_1_0 >= 0 && antinode_1_0 < map.len() as i32 && antinode_1_1 >= 0 && antinode_1_1 < map[0].len() as i32 {
                    println!("Found an antinode at {},{}", antinode_1_0, antinode_1_1);
                    unique_antinodes.insert((antinode_1_0, antinode_1_1));
                }

                let antinode_2_0 = pair.1.0 as i32 - h_diff;
                let antinode_2_1 = pair.1.1 as i32 - w_diff;

                if antinode_2_0 >= 0 && antinode_2_0 < map.len() as i32 && antinode_2_1 >= 0 && antinode_2_1 < map[0].len() as i32 {
                    println!("Found an antinode at {},{}", antinode_2_0, antinode_2_1);
                    unique_antinodes.insert((antinode_2_0, antinode_2_1));
                }
            }
        }       
    }

    unique_antinodes.len() as u32
   
}

fn part_two(map: &mut Vec<Vec<char>>) -> u32 {
    let height = map.len();
    let width = map[0].len();

    println!("working with matrix size {},{}", height, width);
    println!("{:?}", map);

    let char_positions:HashMap<char, Vec<(usize,usize)>> = get_unique_chars(map);
    println!("all chars... {:?}", char_positions.keys());

    let mut unique_antinodes = HashSet::new();

    for c in char_positions.keys() {
        println!("Working character {} positions {:?}", c, char_positions.get(c));

        let mut pairs = Vec::new();

        if let Some(list) = char_positions.get(c) {
            let length_of_list = list.len();
            for i in 0..length_of_list {
                for j in i + 1..length_of_list {
                    if i != j {
                        pairs.push((list[i], list[j]));
                    } 
                }
            }

            for pair in pairs {
                let h_diff: i32 = pair.0.0 as i32 - pair.1.0 as i32;
                let w_diff: i32 = pair.0.1 as i32 - pair.1.1 as i32;

                let mut antinode_1_0:i32 = pair.0.0 as i32;
                let mut antinode_1_1:i32 = pair.0.1 as i32;


                 while antinode_1_0 >= 0 && antinode_1_0 < map.len() as i32 && antinode_1_1 >= 0 && antinode_1_1 < map[0].len() as i32 {
                    println!("Found an antinode at {},{}", antinode_1_0, antinode_1_1);
                    unique_antinodes.insert((antinode_1_0, antinode_1_1));

                    antinode_1_0 += h_diff;
                    antinode_1_1 += w_diff;
                }

                let mut antinode_2_0 = pair.1.0 as i32;
                let mut antinode_2_1 = pair.1.1 as i32;

                while antinode_2_0 >= 0 && antinode_2_0 < map.len() as i32 && antinode_2_1 >= 0 && antinode_2_1 < map[0].len() as i32 {
                    println!("Found an antinode at {},{}", antinode_2_0, antinode_2_1);
                    unique_antinodes.insert((antinode_2_0, antinode_2_1));

                    antinode_2_0 -= h_diff;
                    antinode_2_1 -= w_diff;
                }
            }
        }       
    }

    unique_antinodes.len() as u32
}


fn main() {
    let input = "./src/input.txt";     
    let mut map:Vec<Vec<char>> = Vec::new();
    match read_lines(input) {
        Ok(value) => { 
            for line in value {
                let chars: Vec<char> = line.chars().collect();
                map.push(chars);
            }

            println!("Answer to part two {}", part_two(&mut map));
        
        },
        Err(e) => println!("Error: {}", e),
    }

    
}
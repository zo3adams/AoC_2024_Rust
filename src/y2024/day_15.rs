use std::fs::File;
use std::i32;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::Path;
use colored::Colorize;

use std::thread;
use std::time::Duration;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}


fn print_map(map: &Vec<Vec<char>>) {

    for i in 0..map.len() {
        for j in 0..map[0].len() {

            if map[i][j] == '@' {
                print!("{}", format!("{}",map[i][j]).red());
            }else {
                print!("{}", map[i][j]);
            }

            
        }
        println!("");
    }    
    io::stdout().flush().unwrap();
}



fn _part_one(map:  &mut  Vec<Vec<char>>, moves: Vec<char>) -> i32 {
    let directions: HashMap<char,(i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0))
    ]);
    
    let max_x = map[0].len() as i32;
    let max_y = map.len() as i32;

    let mut robot_position: (i32,i32) = (0,0);
    //get robot position
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                robot_position = (i as i32, j as i32);
            }
        }
    } 

    print_map(map);

    for each_move in moves {     
        

        let mut first_empty: (i32,i32) = (-1,-1);
        println!("Move {}: ", each_move);

        //if there is at least one 'open' space in this direction, move everything between it and the robot that way
        let mut next_y: i32 = robot_position.0 as i32;
        let mut next_x: i32 = robot_position.1 as i32;
        while next_x >= 0 && next_x < max_x && next_y >= 0  && next_y < max_y && map[next_y as usize][next_x as usize] != '#' {
            if map[next_y as usize][next_x as usize] == '.' {
                //println!("found first empty at {},{} when robot was  at {},{}, moving {}",next_y, next_x, robot_position.0, robot_position.1, each_move);
                first_empty = (next_y, next_x);
                break;
            }
            next_y = next_y + directions[&each_move].0;
            next_x = next_x + directions[&each_move].1;

            //println!(" checking for empty at {},{} direction was {}    {},{}",next_y, next_x, each_move, directions[&each_move].0, directions[&each_move].1);
        }

        if first_empty != (-1, -1) {
            while first_empty != robot_position {
                let temp = map[first_empty.0 as usize][first_empty.1 as usize];
                next_x = first_empty.1 - directions[&each_move].1;
                next_y = first_empty.0 - directions[&each_move].0;

                //println!("swapping  {},{}  with {},{}  ",next_y, next_x, first_empty.0, first_empty.1);
                map[first_empty.0 as usize][first_empty.1 as usize] = map[next_y as usize][next_x as usize];
                map[next_y as usize][next_x as usize] = temp;

                first_empty.0 = next_y;
                first_empty.1 = next_x;                
            }
            robot_position.0 = first_empty.0 + directions[&each_move].0;
            robot_position.1 = first_empty.1 + directions[&each_move].1;
            print_map(map);
        }
    }

    //get sum of GPS
    let mut sum_of_gps: i32 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                sum_of_gps += (100* i + j) as i32;
            }
        }
    } 

   sum_of_gps
}

fn part_two(map:  &mut  Vec<Vec<char>>, moves: Vec<char>) -> i32 {
    let directions: HashMap<char,(i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0))
    ]);
    

    let mut new_map:Vec<Vec<char>> = Vec::new();
    for i in 0..map.len() {
        let mut new_row = Vec::new();        
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                new_row.push('#');
                new_row.push('#');
            } 

            if map[i][j] == 'O' {
                new_row.push('[');
                new_row.push(']');
            } 

            if map[i][j] == '.' {
                new_row.push('.');
                new_row.push('.');
            } 

            if map[i][j] == '@' {
                new_row.push('@');
                new_row.push('.');
            }           
        }
        new_map.push(new_row);
    } 


    let mut robot_position: (i32,i32) = (0,0);
    //get robot position
    for i in 0..new_map.len() {
        for j in 0..new_map[0].len() {
            if new_map[i][j] == '@' {
                robot_position = (i as i32, j as i32);
            }
        }
    } 

    print_map(&new_map);
    println!("Robot starting position is {},{}", robot_position.0, robot_position.1);

    for each_move in moves {     
        
        println!("Move {}: ", format!("{}",each_move).green());
        thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
        //find group with a bfs - searching out from robot position
        let mut group: Vec<(i32,i32)> = Vec::new();
        //group.push((robot_position.0, robot_position.1));

        if each_move == '<'  || each_move == '>' {
            let mut next_x = robot_position.1;
            while new_map[robot_position.0 as usize][next_x as usize] != '.' && new_map[robot_position.0 as usize][next_x as usize] != '#' {
                group.push((robot_position.0, next_x));
                next_x = next_x + directions[&each_move].1;  
            }
        } else {
            let mut explore_group: Vec<(i32,i32)> = Vec::new();
            explore_group.push((robot_position.0, robot_position.1));

            while !explore_group.is_empty() {
                let being_visited = explore_group.pop().unwrap();
                group.push(being_visited);

                let next_square = (being_visited.0 + directions[&each_move].0, being_visited.1);

                println!("being visited {},{} and next_square {},{}",being_visited.0, being_visited.1, next_square.0, next_square.1 );
                if new_map[next_square.0 as usize][next_square.1 as usize] == '[' {
                    explore_group.push(next_square);
                    explore_group.push((next_square.0, next_square.1+1));
                } else if new_map[next_square.0 as usize][next_square.1 as usize] == ']' {
                    explore_group.push(next_square);
                    explore_group.push((next_square.0, next_square.1-1));
                }
            }
        }
     println!("group after search was {:?}", group);
    
        // if entire group is movable
        //divide into rows or cols, each with a length
        let mut slices: HashMap<i32, (i32, i32)> = HashMap::new();
        if each_move == '<' || each_move == '>' {

            let min_col = group.iter().map(|&(_, x)| x).min().unwrap();
            let max_col = group.iter().map(|&(_, x)| x).max().unwrap();
            slices.insert(robot_position.0, (min_col, max_col));
        } else {
            let min_col = group.iter().map(|&(_, x)| x).min().unwrap();
            let max_col = group.iter().map(|&(_, x)| x).max().unwrap();

            println!("up or down move,  min col {} and max col {}", min_col, max_col);

            for col in min_col..max_col+1 {
                println!("Checking for rows where col == {}", col);
                let min_row = group.iter().filter(|&(_,x)| *x == col).map(|&(y,_)| y).min().unwrap();
                let max_row = group.iter().filter(|&(_,x)| *x == col).map(|&(y,_)| y).max().unwrap();
                println!("inserting {} : {},{}", col, min_row, max_row);
                slices.insert(col, (min_row, max_row));
            }
        }
        println!("Got slices {:?}", slices);

        let mut free_to_move = true;
        for (k,v) in slices.iter() {
            //println!("checking when k is {}", *k);

            if each_move == '<'  {
                if new_map[*k as usize][(v.0 - 1) as usize] != '.' {  // chec
                    free_to_move = false;
                }
            } else if each_move == '>'  {
                if new_map[*k as usize][(v.1+1) as usize] != '.' { //max move to right
                    free_to_move = false;
                }
            } else if each_move == '^'  {
                if new_map[(v.0-1) as usize][*k as usize] != '.' { //min move up
                    free_to_move = false;
                }
            } else if each_move == 'v'  {
                println!("new_map square at position {},{} has chat {}", (v.1+1) as usize, *k as usize, new_map[(v.1+1) as usize][*k as usize]);
                if new_map[(v.1+1) as usize][*k as usize] != '.' { //max move down
                    free_to_move = false;
                }
            } 
        }
     

        //perform move for each slice
        if free_to_move {
            println!("{}", "Attempting move".green());
            for (k,v) in slices.iter() {
                if each_move == '<'  {
                    let mut current = v.0;
                    while current <= v.1 {
                        //println!("Current column {}", current);
                        let temp_char: char = new_map[*k as usize][(current-1) as usize];
                        new_map[*k as usize][(current-1) as usize] = new_map[*k as usize][current as usize];
                        new_map[*k as usize][current as usize] = temp_char;
                        current = current + 1;
                        //print_map(&new_map);
                    }
                }

                if each_move == '>'  {
                    let mut current = v.1;
                    while current >= v.0 {
                        let temp_char: char = new_map[*k as usize][(current+1) as usize];
                        new_map[*k as usize][(current+1) as usize] = new_map[*k as usize][current as usize];
                        new_map[*k as usize][current as usize] = temp_char;
                        current = current - 1;
                    }
                }

                if each_move == '^'  {
                    let mut current = v.0;
                    while current <= v.1 {
                        let temp_char: char = new_map[(current-1) as usize][*k as usize];
                        new_map[(current-1) as usize][*k as usize] = new_map[current as usize][*k as usize];
                        new_map[current as usize][*k as usize] = temp_char;
                        current = current + 1;
                    }
                }

                if each_move == 'v'  {
                    let mut current = v.1;
                    while current >= v.0 {
                        let temp_char: char = new_map[(current+1) as usize][*k as usize];
                        new_map[(current+1) as usize][*k as usize] = new_map[current as usize][*k as usize];
                        new_map[current as usize][*k as usize] = temp_char;
                        current = current - 1;
                    }
                }
                
            }
            robot_position = (robot_position.0 + directions[&each_move].0, robot_position.1 + directions[&each_move].1);
        } else {
            println!("{}", "did NOT move".red());
        }
        
        print_map(&new_map);
        println!("robot position is {},{}",robot_position.0, robot_position.1 );
    }

    //get sum of GPS
    let mut sum_of_gps: i32 = 0;
    for i in 0..new_map.len() {
        for j in 0..new_map[0].len() {
            if new_map[i][j] == '[' {
                sum_of_gps += (100* i + j) as i32;
            }
        }
    } 

   sum_of_gps
}


pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut have_closed_map = false;

            let mut map: Vec<Vec<char>> = Vec::new();
            let mut moves: Vec<char> = Vec::new();

            for line in value {

                if line.is_empty() {
                    have_closed_map = true;
                    continue;                    
                }

                if !have_closed_map {
                    map.push(line.chars().collect());
                } else {
                    moves.extend(line.chars());
                }


            }

            
            //println!("{}", all_lines_combined);
        
                //println!("Answer to part one:  {}", part_one( &mut map, moves));
                println!("Answer to part two:  {}", part_two( &mut map, moves));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
       Ok(())     
    }
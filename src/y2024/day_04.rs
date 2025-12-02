
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::clone::Clone;

fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
    HorizontalBackwards,
    VerticalBackwards,
    DiagonalDownRight,
    DiagonalDownLeft,
    DiagonalUpRight,
    DiagonalUpLeft,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PatternMatch {
    row: usize,
    col: usize,
    direction: Direction,
}

fn _find_pattern(grid: &Vec<Vec<char>>, pattern: &str) -> Vec<PatternMatch> {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let mut matches = Vec::new();

    let rows = grid.len();
    if rows == 0 {
        return matches;
    }
    let cols = grid[0].len();

    // Directions to search: Horizontal, Vertical, Diagonal, Backward
    let directions = [
        (0,1, Direction::Horizontal),
        (1,0, Direction::Vertical),
        (0,-1, Direction::HorizontalBackwards),
        (-1,0, Direction::VerticalBackwards),
        (1, 1, Direction::DiagonalDownRight), // Diagonal down-right
        (1, -1, Direction::DiagonalDownLeft), // Diagonal down-left
        (-1, 1, Direction::DiagonalUpRight), // Diagonal up-right
        (-1, -1, Direction::DiagonalUpLeft), // Diagonal up-left
    ];

    for start_row in 0..rows {
        for start_col in 0..cols {
            // Check each direction
            for &(dr, dc, direction) in &directions {
                // Forward search
                if let Some(_match_result) = _check_pattern_forward(
                    grid, 
                    &pattern_chars, 
                    start_row, 
                    start_col, 
                    dr, 
                    dc
                ) {
                    matches.push(PatternMatch {
                        row: start_row,
                        col: start_col,
                        direction,
                    });
                }             
            }
        }
    }

    matches
}

fn _check_pattern_forward(
    grid: &Vec<Vec<char>>, 
    pattern: &Vec<char>, 
    start_row: usize, 
    start_col: usize, 
    dr: i32, 
    dc: i32
) -> Option<bool> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Check if the entire pattern fits within the grid
    let end_row = start_row as i32 + (pattern.len() as i32 - 1) * dr;
    let end_col = start_col as i32 + (pattern.len() as i32 - 1) * dc;

    if end_row < 0 || end_row >= rows || end_col < 0 || end_col >= cols {
        return None;
    }

    // Check if the pattern matches
    for (i, &ch) in pattern.iter().enumerate() {
        let current_row = start_row as i32 + (i as i32 * dr);
        let current_col = start_col as i32 + (i as i32 * dc);

        if current_row < 0 || current_row >= rows || 
           current_col < 0 || current_col >= cols ||
           grid[current_row as usize][current_col as usize] != ch {
            return None;
        }
    }
    Some(true)
}

fn _part_one(data: &Vec<Vec<char>>) -> u32 {
    let pattern_matches = _find_pattern(&data, "XMAS");
   // println!(" pattern matches {:?}", pattern_matches);
    //println!("Found {} matches", pattern_matches.len());
    pattern_matches.len() as u32
}


fn part_two(data: &Vec<Vec<char>>) -> u32 {
   let mut xmas_count:u32 = 0;
   let match_strings:Vec<String> = vec!["M_M_S_S".to_string(), "S_M_S_M".to_string(), "M_S_M_S".to_string(), "S_S_M_M".to_string()];

   for i in 1..(data.len()-1) {
    for j in 1..(data[0].len()-1)  {
        if data[i][j] == 'A' {
            let xstring = format!("{}_{}_{}_{}", data[i-1][j-1], data[i-1][j+1],data[i+1][j-1],data[i+1][j+1]);
            println!("Found an A - at coordinates {} {}", i, j);
            if match_strings.contains(&xstring) {
                println!("WAS a match with signature {}", xstring);
                xmas_count = xmas_count + 1;
            }
        }
    }

   }
   xmas_count

}

pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";     
 
    match read_lines(input) {
        Ok(value) => {  
            let matrix: Vec<Vec<char>> = value.into_iter().map(|s| s.chars().collect::<Vec<_>>()).collect();
            println!("Got matrix  {:?}", matrix);

            //println!("Found {} instances", part_one(&matrix));
            println!("Found {} x cross instances", part_two(&matrix));
        },
    
        Err(e) => println!("Error: {}", e),
    }
  
    Ok(())
}
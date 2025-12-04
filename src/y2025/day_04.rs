
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn read_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}


fn get_removable_list(grid: & Vec<Vec<char>> ) -> Vec<(usize,usize)> {
    // build char 2D array
    let y_length = grid.len();
    let x_length = grid[0].len();

    // count adjacent square contents < 4 is accessible and out of bounds count as zero
    let mut removables : Vec<(usize,usize)> = Vec::new();
    let directions: Vec<(i32, i32)> = vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    for x in 0..x_length {
        for y in 0..y_length {

            if grid[y][x] != '@' { continue;}
            let mut neighbor_count = 0;

            for direction in directions.iter() {
                let neighbor_x = x as i32 + direction.0;
                let neighbor_y = y as i32 + direction.1;

                if neighbor_x >= 0 && neighbor_x < x_length as i32 && neighbor_y >= 0 && neighbor_y < y_length as i32 {
                    if '@' == grid[neighbor_y as usize][neighbor_x as usize] {
                        neighbor_count += 1;
                        
                    }
                }
            }

            if neighbor_count < 4 {
                removables.push((x,y));
                //println!("valid at x,y {},{}", x,y);
            }
        }
    }
    removables
}

fn part_one(values: &[String]) -> i32 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for row in values {
        let as_chars_row: Vec<char> = row.chars().collect();
        grid.push(as_chars_row);
    }
    
    let removables = get_removable_list(&grid);
    removables.len() as i32
}

fn part_two(values: &[String]) -> i32 {
       // build char 2D array
    let mut grid: Vec<Vec<char>> = Vec::new();

    let y_length = values.len();
    
    for row in values {
        let as_chars_row: Vec<char> = row.chars().collect();
        grid.push(as_chars_row);
    }

    let mut total_removed_count: i32 = 0;
    let mut removed_count = 9999;
    while removed_count > 0 {
        let removables = get_removable_list(&grid);
        removed_count = removables.len();
        for remove_op in removables {
            grid[remove_op.1][remove_op.0] = '.';
        }

        total_removed_count += removed_count as i32;
    }
    total_removed_count
}

pub fn run() -> io::Result<()>  {
    let values = read_data("src/input.txt")?;

    println!("Part 1: {}", part_one( &values));
    println!("Part 2: {}", part_two(&values));

    Ok(())
}
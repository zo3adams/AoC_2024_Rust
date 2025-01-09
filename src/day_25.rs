
use std::fs::{File, OpenOptions, remove_file};
use std::io::{self, Write};
use std::ptr::addr_eq;
use std::{any, i64};
use std::io::{BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};
use indexmap::IndexMap;

#[derive(Debug, Clone)]
struct Operation {
    input1: String,
    input2: String,
    operator: String,
    output: String,
}

fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn part_one(locks: Vec<Vec<u32>>, keys: Vec<Vec<u32>>) -> u32 {
    println!("Got locks {:?}", locks);
    println!("Got keys {:?}", keys);

    let mut overlap_count = 0;
    let mut fit_count = 0;


    for lock in &locks {
        for key in &keys {

            let mut fit = true;
            for i in 0..lock.len() {
                if (lock[i] + key[i]) >= 6 {
                    fit = false;
                    break;
                } 
            }
            if fit {
                fit_count += 1;
            } else {
                overlap_count += 1;
            }
        }
    }

    println!("{} combos overlap and  {} combos fit", overlap_count, fit_count);
    fit_count
 }        

fn main() {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut locks: Vec<Vec<u32>> = Vec::new();
            let mut keys:  Vec<Vec<u32>> = Vec::new();

            let tokens: Vec<&[String]> = value.split(|x| x == "").collect();

            for token in tokens {
                println!("token {:?}", token);

                if token[0] == "#####" {
                    //a lock
                    let mut new_lock = vec![0 as u32; token[0].len()];
                    for line in token[1..].iter() {
                        for i in 0..line.len() {
                            if let Some(c) = line.chars().nth(i) {                                
                                if c == '#' {
                                    new_lock[i] += 1;         
                                }                                                       
                            }
                        }
                    }
                    locks.push(new_lock);
                }

                if token[0] == "....." {
                    //a key
                    let mut new_key: Vec<u32> = vec![0 as u32; token[0].len()];
                    for line in token[..token.len() - 1].iter().rev() {
                        for i in 0..line.len() {
                            if let Some(c) = line.chars().nth(i) {
                                if c == '#' {
                                    new_key[i] += 1;         
                                }                                                       
                            }
                        }
                    }
                    keys.push(new_key);
                }
            }
            println!("Answer to part two:  {}", part_one( locks, keys));
         },
         Err(e) => println!("Error: {}", e),
          
        }
        
    }
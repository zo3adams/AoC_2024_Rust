use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::{self, BufRead, Write};
use std::path::Path;
use colored::Colorize;




fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

//returns the towel combo that makes the design, or blank if not possible
fn dfs_into_design(design: String, towels: & Vec<String>) -> Vec<Vec<String>> {

    let mut final_combos: Vec<Vec<String>> = Vec::new();
    let mut candidates: Vec<Vec<String>> = vec![vec!["".to_string()]];
    while !candidates.is_empty() {
        let candidate = candidates.pop().unwrap();
        for towel in towels {
            let mut merged = candidate.clone();
            merged.extend(vec![towel.clone()]);
            //println!("checking merged {:?}", merged);
            if design == merged.join("") {
                //println!("{:?} equality passed!", merged);
                final_combos.push(merged.clone());
            }
            if design.starts_with(&merged.join("")) {
                //println!("{:?} starts with passed!", merged);
                candidates.push(merged);
            }
        }
    }
    final_combos
}


fn cached_recursive_dfs(target_design: String, towels: & Vec<String>, prefix_design: String, results_cache: &mut HashMap<String, u64>) -> u64 {
    if let Some(&ref result) = results_cache.get(&prefix_design) {
        //println!("Retrieving cache results for {}", prefix_design);
        return *result; // Return cached result if available
    }
    if prefix_design == target_design {
        return 1;
    } else {
        let mut count: u64 = 0;
        for towel in towels {
            let new_prefix = prefix_design.clone() + towel;
            let target_clone = target_design.clone();
            if target_clone.starts_with(&new_prefix) {
                let result = cached_recursive_dfs(target_clone, towels, new_prefix.clone(), results_cache);
                //println!("Caching result for {} - result was {:?}", new_prefix, result);
                results_cache.insert(new_prefix, result);
                count += result;
            } 
        }
        return count;
    }
}

fn part_one(towels: Vec<String>, designs: Vec<String>) -> i32 {
    let mut possible_designs: i32 = 0;

    for design in designs {
        print!("Checking design {}", design);
        let combo = dfs_into_design(design, &towels);
        if !combo.is_empty() {
            println!("  => can be made {} different ways", combo.len());
            possible_designs += 1;
        } else {
            println!("  => cannot find a way to make it");
        }
    }
    possible_designs
}


fn part_two(towels: Vec<String>, designs: Vec<String>) -> u64 {
    let mut total_possible_designs: u64 = 0;
    
    for i in 0..designs.len() {
        println!("Checking design {}/{} :  {}", i, designs.len(), designs[i]);
        let mut results_cache: HashMap<String, u64> = HashMap::new();
        let possible_designs = cached_recursive_dfs(designs[i].clone(), &towels, "".to_string(), &mut results_cache);
        if possible_designs > 0 {
            println!("  => can be made {} different ways", possible_designs);
            total_possible_designs += possible_designs;
        } else {
            println!("  => cannot find a way to make it");
        }
    }
    total_possible_designs
}



fn main() {
    let input = "./src/input.txt";  

    match read_lines(input) {
        Ok(value) => {    

            let towels_str = value.first().unwrap();
            let towels: Vec<String> = towels_str.split(",").map(|s| s.trim().to_string()).collect();
            println!("towels: {:?}", towels);
            let mut designs:Vec<String> = Vec::new();

            for line in &value[2..] {
                designs.push(line.to_string());

            }
           
            //println!("Answer to part one:  {}", part_one(towels, designs));
            println!("Answer to part two:  {}", part_two(towels, designs));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        
    }
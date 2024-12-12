
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::ptr::eq;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}



fn part_one(operands: Vec<Vec<u64>>) -> u64 {
    let mut sum_of_possible_equations = 0;
    for equation in operands {
        //println!("working equation {:?}", equation);
        let target_result = equation[0];
        let mut in_prog_results: Vec<u64> = Vec::new();
        in_prog_results.push(equation[1]);
        let mut inputs_index: usize = 2;
        while inputs_index < equation.len() {

            //println!("in prog results at input index {} =>   {:?}", inputs_index, in_prog_results);
            let mut new_results: Vec<u64> = Vec::new();
            for in_prog_val in in_prog_results {
                new_results.push(in_prog_val + equation[inputs_index]);
                new_results.push(in_prog_val * equation[inputs_index]);
            }
            in_prog_results = new_results;
            inputs_index += 1;
        }

        if in_prog_results.iter().any(|&x| x == target_result) {
            println!("equation {:?} could be true", equation);
            sum_of_possible_equations += target_result;
        }       

    }

    sum_of_possible_equations
}

fn part_two(operands:Vec<Vec<u64>>) -> u64 {
    let mut sum_of_possible_equations = 0;
    for equation in operands {
        //println!("working equation {:?}", equation);
        let target_result = equation[0];
        let mut in_prog_results: Vec<u64> = Vec::new();
        in_prog_results.push(equation[1]);
        let mut inputs_index: usize = 2;
        while inputs_index < equation.len() {

            //println!("in prog results at input index {} =>   {:?}", inputs_index, in_prog_results);
            let mut new_results: Vec<u64> = Vec::new();
            for in_prog_val in in_prog_results {
                new_results.push(in_prog_val + equation[inputs_index]);
                new_results.push(in_prog_val * equation[inputs_index]);

                let cocat_value = format!("{}{}", in_prog_val, equation[inputs_index]);
                new_results.push(cocat_value.parse().expect("failed to parse after cocat?"));

            }
            in_prog_results = new_results;
            inputs_index += 1;
        }

        if in_prog_results.iter().any(|&x| x == target_result) {
            println!("equation {:?} could be true", equation);
            sum_of_possible_equations += target_result;
        }       

    }

    sum_of_possible_equations
}


fn main() {
    let input = "./src/sample_input.txt";     
    let mut operands:Vec<Vec<u64>> = Vec::new();
    match read_lines(input) {
        Ok(value) => { 

            for line in value {
                //println!("{}",line);
                let split_borrowed: Vec<&str> = line.split(':').collect();
                let mut this_line: Vec<u64> = Vec::new();
                this_line.push(split_borrowed[0].parse().expect("Failed to parse integer out"));
                let inputs: Vec<u64> = split_borrowed[1].split_whitespace().map(|s| s.parse().expect("Failed to parse integer out")).collect();
                for input in inputs  {
                    this_line.push(input);
                }
                operands.push(this_line);
            }
            
                    
            println!("{:?}", operands);
            //println!("Answer to part one {}", part_one(operands));     
            println!("Answer to part two {}", part_two(operands));     
          
        },
        Err(e) => println!("Error: {}", e),
    }

    
}
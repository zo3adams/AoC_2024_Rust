
use std::fs::File;
use std::i64;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap};

fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}


fn get_generated_secrets(start_secrets: Vec<i64>, iterations: usize) -> Vec<Vec<i64>> {
    
    let mut all_secrets: Vec<Vec<i64>> = Vec::new();
    for mut secret in start_secrets {
        let mut in_prog_secrets:Vec<i64> = Vec::new();
        in_prog_secrets.push(secret);
        for _i in 0..iterations {
            //println!("on turn {} secret is {}", i, secret);
            let step_one = ((secret * 64) ^ secret) % 16777216;
            let step_two = ((step_one / 32) ^ step_one) % 16777216;
            let step_three = ((step_two * 2048) ^ step_two) % 16777216;
            //println!("after turn {} secret steps include {} {} {} ", i, step_one, step_two, step_three);
            secret = step_three;
            in_prog_secrets.push(secret);
        }
        all_secrets.push(in_prog_secrets);
    }
    //println!("Got  final secrets {:?}", final_secrets);
    all_secrets
}

fn _part_one(start_secrets:  Vec<i64>) -> i64 {
    get_generated_secrets(start_secrets, 2000).iter().map(|v| v.last().unwrap_or(&0)).sum()
}

fn _get_sequence_return(sequence: (i64,i64,i64,i64), all_deltas: & Vec<Vec<i64>>, generated_secrets: & Vec<Vec<i64>>) -> (i64, Vec<i64>) {
    //for each delta, scan through to find sequence, then pull value from that index in generated_secrets
    let mut val_sum: i64 = 0;
    let mut individual_values: Vec<i64> = Vec::new();
    for secret_chain_index in 0..all_deltas.len() {
        for j in 3..all_deltas[secret_chain_index].len() {
            if all_deltas[secret_chain_index][j] == sequence.3 && all_deltas[secret_chain_index][j-1] == sequence.2 
            && all_deltas[secret_chain_index][j-2] == sequence.1 && all_deltas[secret_chain_index][j-3] == sequence.0 {
                //found it - add to sum
                individual_values.push(generated_secrets[secret_chain_index][j] % 10);
                val_sum += generated_secrets[secret_chain_index][j] % 10;
                break;
            }
        }
    }
    (val_sum, individual_values)
}
          
fn part_two(start_secrets:  Vec<i64>) -> i64 {
    let iterations: usize = 2000;
    let generated_secrets: Vec<Vec<i64>> = get_generated_secrets(start_secrets, iterations);
    //println!("Got GENERATED secrets: {:?}", generated_secrets);
    //get list of all deltas
    let mut all_deltas: Vec<Vec<i64>> = Vec::new();
    for secret_chain in &generated_secrets {
        let mut deltas: Vec<i64> = vec![0 as i64; secret_chain.len()];
        for i in 1..deltas.len() {
            deltas[i] = (secret_chain[i]%10) - (secret_chain[i-1]%10);
        }
        all_deltas.push(deltas);
    }

    //used to track where found, save time searching later
    let mut sequences_found_in: HashMap<(i64,i64,i64,i64), HashMap<usize,usize>> = HashMap::new(); 

    for secret_chain_index in 1..all_deltas.len() {
        //println!("checking sequences in secret chain index {}", secret_chain_index);
        for j in 4..all_deltas[secret_chain_index].len() {
            let sequence: (i64,i64,i64,i64) = (all_deltas[secret_chain_index][j-3], all_deltas[secret_chain_index][j-2], 
                all_deltas[secret_chain_index][j-1], all_deltas[secret_chain_index][j]);

            if sequences_found_in.contains_key(&sequence) && !sequences_found_in[&sequence].contains_key(&secret_chain_index) {
                sequences_found_in.get_mut(&sequence).unwrap().insert(secret_chain_index, j);
            } else if !sequences_found_in.contains_key(&sequence) {
                sequences_found_in.insert(sequence, HashMap::from([(secret_chain_index,j)]));
            } 
        }
    }
    println!("Found {} unique sequences", sequences_found_in.keys().len());
    let mut max_val_seen: i64 = 0;

    for (sequence,  locations) in sequences_found_in {
        let mut this_val = 0;
        for location in locations {
            this_val += generated_secrets[location.0][location.1] % 10;            
        }

        if this_val > max_val_seen {
            max_val_seen = this_val;
            println!("Found new max {}  with sequence {:?}", max_val_seen, sequence);
        }
    }
        
    max_val_seen
}


pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut start_secrets: Vec<i64> = Vec::new();
            for line in value {
                start_secrets.push(line.parse::<i64>().unwrap());
            } 
            //println!("Answer to part one:  {}", part_one( start_secrets));
            println!("Answer to part two:  {}", part_two(start_secrets));
         },
         Err(e) => println!("Error: {}", e),
          
        }

    Ok(())
        
    }
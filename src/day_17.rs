use std::fs::File;
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


fn get_combo_operand( operand: u8, reg_a: i64, reg_b: i64, reg_c: i64) -> u32 {
    //println!("operand was {} and reg_c was {}", operand, reg_c);
    if operand < 4 {
        return operand as u32;
    } else if operand == 4 {
        return reg_a as u32;
    } else if operand == 5 {
        return reg_b as u32;
    } else if operand == 6 {
        return reg_c as u32;
    }
    println!("ERROR no match, wtf?");
    99
}

fn part_one(mut reg_a: i64, mut reg_b: i64, mut reg_c: i64, instructions: Vec<u8>, quine_mode: bool) -> Vec<u8> {
   //println!("Starting reg values a:{} b:{} c:{} and instructions {:?} instrucion lenght is {} ", reg_a, reg_b, reg_c, instructions, instructions.len());

   let mut pc = 0;
   let mut output: Vec<u8> = Vec::new();
   while pc < instructions.len() {
        let instruction = instructions[pc];
        let operand = instructions[pc +1];

        //print!("pc: {} instruction: {} operand: {}  reg_a {}, reg_b {}, reg_c {}", pc, instruction, operand, reg_a, reg_b, reg_c);
        match instruction  {
            0 =>  {  //adv
                let result: i64 = reg_a / (2 as i64).pow(get_combo_operand(operand, reg_a, reg_b, reg_c));
                reg_a = result;        
            },
            1 => {  //bxl
                let result = reg_b ^ operand as i64;
                reg_b = result; 
            },
            2 => { //bst
                let result = get_combo_operand(operand, reg_a, reg_b, reg_c) % 8;
                reg_b = result as i64;
            },
            3 => { //jnz 
                if reg_a != 0 {
                    pc = operand as usize;                  
                } 
            },
            4 => { //bxc 
                let result = reg_b ^ reg_c;
                reg_b = result; 
            },
            5 => { // out 
                let result = get_combo_operand(operand, reg_a, reg_b, reg_c) % 8;
                output.push(result as u8);

                //optimziation for part two 
                if quine_mode {

                    if output.len() > instructions.len() || result as u8 != instructions[output.len() - 1] {
                        return output; //early drop out
                    }
                }
            },
            6 => { //bdv 
                let result = reg_a / (2 as i64).pow(get_combo_operand(operand, reg_a, reg_b, reg_c));
                reg_b = result;  
            },
            7 => {
                let result = reg_a / (2 as i64).pow(get_combo_operand(operand, reg_a, reg_b, reg_c));
                reg_c = result;
            },
            _   => println!("Unknown Instruction!"),
        }

        if instruction != 3 ||  (instruction == 3  && reg_a == 0) {
            pc += 2; 
        }
   }

   //println!("at end of program  reg_a {}, reg_b {}, reg_c {}", reg_a, reg_b, reg_c);
   println!("{:?}", output);
   output
   
}




fn part_two(reg_b: i64, reg_c: i64, instructions: Vec<u8>) -> i64 {

    //below should work, but takes very long to run on the real input
    let start = 0;
    for i in start..i64::MAX{
        let response = part_one(i, reg_b, reg_c, instructions.clone(), true);
        if response == instructions {
            println!(" is a quine (sp?) at reg a value {} with output {:?}", i, response);
            return i;
        } 
    }
    0
}


fn main() {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut reg_a: i64 = 0;
            let mut reg_b: i64 = 0;
            let mut reg_c: i64 = 0;

            let mut instructions: Vec<u8> = Vec::new();

            for line in value {
                if line.contains("Register A") {
                    reg_a = line.split(": ")
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                }

                if line.contains("Register B") {
                    reg_b = line.split(": ")
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                }

                if line.contains("Register C") {
                    reg_c = line.split(": ")
                    .nth(1)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                }

                if line.contains("Program") {
                    instructions = line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                }
            }

            //println!("Answer to part one:  {:?}", part_one(reg_a, reg_b, reg_c, instructions, false));
            println!("Answer to part two:  {}", part_two( reg_b, reg_c, instructions));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        
    }
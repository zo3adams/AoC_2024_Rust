
use std::fs::{File};
use std::io::{self, Write};
use std::{i64};
use std::io::{BufRead};
use std::path::Path;
use std::collections::{HashMap, VecDeque};
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

//performs the operation and returns output value
fn perform_operation(op: Operation, wires: &HashMap<String, bool> ) -> bool {
    if op.operator == "AND" {
        return wires[&op.input1] & wires[&op.input2];
    } else if op.operator == "OR"  {
        return wires[&op.input1] | wires[&op.input2];
    } else if op.operator == "XOR"  {
        return wires[&op.input1] ^ wires[&op.input2];
    } else {
        println!("GOT AN UNRECOGNIZED OPERAND {}    WE ARE BROKEN", op.operator);
        return false;
    }
}


fn get_z_values(wires: &mut HashMap<String, bool>, ops_by_output: HashMap<String, Operation>) -> Vec<bool> {
    //println!("wires {:?} \n", wires);
    //println!("operations {:?} \n", ops_by_output);

    //build list of all wires
    let mut all_unknown_wires: VecDeque<String> = VecDeque::new();

    for operation in ops_by_output.values() {  
        if wires.contains_key(&operation.input1) && wires.contains_key(&operation.input2)  {
            wires.insert(operation.output.clone(), perform_operation(operation.clone(), wires));
        } else {
            all_unknown_wires.push_back(operation.output.clone())
        }
    }
    //println!("after ingestion we have unknown wires {:?}", all_unknown_wires);
    //println!("and known wires {:?}", wires);


    //iterate through list until all wire values are known
    while all_unknown_wires.len() > 0 {

        let wire = all_unknown_wires.pop_back().unwrap();
        let op = ops_by_output[&wire].clone();
        if wires.contains_key(&op.input1) && wires.contains_key(&op.input2)  {
            wires.insert(op.output.clone(), perform_operation(op.clone(), wires));
        } else {
            all_unknown_wires.push_front(wire);
        }
    }

    //print out z values
    let mut z_wire_names: Vec<String> = Vec::new();
    let mut z_values: Vec<bool> = Vec::new();

    for (wire, _value) in wires.iter() {
        if wire.starts_with('z') {
            z_wire_names.push(wire.clone());
        }
    }
    z_wire_names.sort();
    //print!(" z value output:   ");
    
    
    for z in z_wire_names {
        //print!("{}, ", wires[&z]);
        z_values.push(wires[&z]);        
    }
    z_values
}

fn part_one(wires: &mut HashMap<String, bool>, ops_by_output: HashMap<String, Operation>) -> i64 {
    let z_values = get_z_values(wires, ops_by_output);
    let mut pow_index = 0;
    let mut sum = 0;
    for z in z_values {
        if z {
            sum += i64::from(2).pow(pow_index);
        }
        pow_index += 1;
    }
   sum
}
          
fn part_two(wires: &mut HashMap<String, bool>, ops_by_output: HashMap<String, Operation>) -> String {

    //calc as if it were a simple add
    let mut x_inputs: Vec<&String> = wires.keys().filter(|k| k.starts_with('x')).collect();
    let mut y_inputs: Vec<&String> = wires.keys().filter(|k| k.starts_with('y')).collect();
    x_inputs.sort();
    y_inputs.sort();

    //create a map of all gates,  keep it keyed (and ordered) by the original gate text description
    let mut working_adder_comp:  IndexMap<String, Operation> =  IndexMap::new();
    let mut carry_over:String = "".to_string();
    for i in 0..x_inputs.len() {

        let x_string = format!("x{:02}",i);
        let y_string = format!("y{:02}",i);

        let and_string = format!("and_naive_sum{:02}",i); 
        let xor_string = format!("xor_naive_carry{:02}",i);
        let out_string = format!("z{:02}",i); // xor inptus buts and carry to get this output bit
        let candxor_string = format!("c_and_xor{:02}",i); // carry and xor  for first part of carry circuit
        let carry_string = format!("final_carry{:02}",i); //final or gate  for next bit carry
       
        if i >= 1 {
            working_adder_comp.insert(format!("{} {} {} -> {}", x_string,"AND",y_string,and_string), Operation{ input1: x_string.clone(),operator: "AND".to_string(),input2: y_string.clone(),output: and_string.clone() });            
            working_adder_comp.insert(format!("{} {} {} -> {}",x_string, "XOR", y_string, xor_string), Operation{ input1: x_string.clone(), operator: "XOR".to_string(),input2: y_string, output: xor_string.clone() });
            working_adder_comp.insert(format!("{} {} {} -> {}",carry_over, "XOR",  xor_string, out_string), Operation{ input1: carry_over.clone(), input2: xor_string.clone(), operator: "XOR".to_string(),output: out_string.clone() });
            working_adder_comp.insert(format!("{} {} {} -> {}",carry_over, "AND", xor_string, candxor_string), Operation{ input1: carry_over.clone(), input2: xor_string.clone(), operator: "AND".to_string(),output: candxor_string.clone() });
            working_adder_comp.insert(format!("{} {} {} -> {}",candxor_string, "OR", and_string, carry_string), Operation{input1: candxor_string.clone(), operator: "OR".to_string(),input2: and_string, output: carry_string.clone()});
        } else {
            //for first bit, out is just XOR, and carry is AND of input bits, as carry is always zero
            working_adder_comp.insert(format!("{} {} {} -> {}",x_string, "XOR", y_string,  out_string), Operation{ input1: x_string.clone(), input2: y_string.clone(), operator: "XOR".to_string(),output: out_string });
            working_adder_comp.insert(format!("{} {} {} -> {}",x_string, "AND", y_string, carry_string), Operation{ input1: x_string.clone(), input2: y_string.clone(), operator: "AND".to_string(),output: carry_string.clone() });
        }        
        carry_over = carry_string.clone();

    }
        
    //let final_z_string = format!("z{:02}",x_inputs.len());
    //working_adder_comp.insert(format!("{} {} {} -> {}",format!("*can{:02}",x_inputs.len()-1), "OR", format!("*and{:02}",x_inputs.len()-1), final_z_string), 
    //Operation{input1: format!("*can{:02}",x_inputs.len()-1), input2: format!("*and{:02}",x_inputs.len()-1), operator: "OR".to_string(), output:  final_z_string.clone()});

    for (description, _gate) in working_adder_comp.iter() {
        println!("{}", description);
    }

    let mut untagged_wires: VecDeque<String> = VecDeque::new();
    let mut tagged_wires: IndexMap<String, String> = IndexMap::new(); // maps a description string (key of working_adder_comp) to the wire it is tagged to

    for (actual_wire, _actual_op) in ops_by_output.iter() {
        untagged_wires.push_back(actual_wire.to_string());
    }

    let mut last_untagged_size = untagged_wires.len() + 1;
    while last_untagged_size > untagged_wires.len() {
        
        last_untagged_size = untagged_wires.len();
      
        let mut output_file = File::create("src/wire_diagram_output.txt").expect("FAILED TO OPEN FILE");                

        for _i in 0..last_untagged_size {
            let candidate = untagged_wires.pop_back().unwrap();
            let actual_op = &ops_by_output[&candidate];
            println!("Working candidate {}", candidate);
            let mut name_swap = "".to_string();

            for (description, gate) in working_adder_comp.iter_mut() {   

                if (actual_op.input1 == gate.input1 && actual_op.input2 == gate.input2 && actual_op.operator == gate.operator) 
                || (actual_op.input1 == gate.input2 && actual_op.input2 == gate.input1 && actual_op.operator == gate.operator)  { //allow reverse order
                    name_swap = gate.output.clone();
                    println!("found a match {} -{}", candidate, name_swap);
                    tagged_wires.insert(description.to_string(), candidate.clone());
                    gate.output = candidate.clone();
                    break;
                }  


            }

            if name_swap != "" {
                //change to candidate for all inputs
                for (description, gate) in working_adder_comp.iter_mut() {  
                    if gate.input1 == name_swap {
                        println!("changing {} to {} in {} ",gate.input1, candidate, description );
                        gate.input1 = candidate.clone();
                    }
                    if gate.input2 == name_swap {
                        println!("changing {} to {} in {} ",gate.input2, candidate, description );
                        gate.input2 = candidate.clone();
                    }
                }
                //println!(" after name_swap {} to {}  contents of working adder comp {:?}", name_swap, candidate, working_adder_comp);
            } else {
                untagged_wires.push_front(candidate); // we never found a match, it may happen on a future rev, though
            }
        }
         //and finally, print out.
         for (description, gate) in working_adder_comp.iter_mut() {  
            let mut add_on_string = "".to_string();
            if  tagged_wires.contains_key(description) {
                let actual_wire_name = tagged_wires.get(description).unwrap();
                add_on_string += actual_wire_name;
                add_on_string +=  &format!("({} {} {})", ops_by_output[actual_wire_name].input1, ops_by_output[actual_wire_name].operator, ops_by_output[actual_wire_name].input2).to_string();
            }
            let updated_working_gate = format!("{} {} {} -> {}", gate.input1, gate.operator, gate.input2, gate.output);
            let _result = writeln!(output_file, "{:54} | {:18}  | {}", description, add_on_string,updated_working_gate);
        }
    }


    println!("in the end we have {} untagged wires  {:?}", untagged_wires.len(), untagged_wires);

    "".to_string()
}


pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut wires: HashMap<String, bool> = HashMap::new();
            let mut ops_by_output: HashMap<String, Operation> = HashMap::new();
            let tokens: Vec<&[String]> = value.split(|x| x == "").collect();

            for wire in tokens[0] {
                let parts: Vec<&str> = wire.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    //println!("{}",parts[1].trim());
                    let value = if parts[1].trim() == "1" {true} else {false};
                    wires.insert(key, value);
                }
            }
                      
            for line in tokens[1] {
                if line.contains("->") {
                    let parts: Vec<&str> = line.split("->").collect();
                    if parts.len() == 2 {
                        let operation_parts: Vec<&str> = parts[0].trim().split_whitespace().collect();
                        if operation_parts.len() == 3 {

                            let op: Operation = Operation {
                                input1: operation_parts[0].to_string(),
                                operator: operation_parts[1].to_string(),
                                input2: operation_parts[2].to_string(),
                                output: parts[1].trim().to_string(),
                            };
                            ops_by_output.insert(parts[1].trim().to_string(), op);
                        }
                    }
                }
            }
           
        
            println!("Answer to part one:  {}", part_one( &mut wires, ops_by_output.clone()));
            println!("Answer to part two:  {}", part_two( &mut wires, ops_by_output));
         },
         Err(e) => println!("Error: {}", e),
          
        }
        Ok(())
    }
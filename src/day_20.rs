use std::fs::File;
use std::i32;
use std::io::{self, BufRead, Write};
use std::path::Path;
use colored::Colorize;

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;


fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    node: String,
}

// Reverse ordering for BinaryHeap to act as a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse to make BinaryHeap a min-heap
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



fn print_map(map: &Vec<Vec<char>>, predecessors: HashSet<String>) {

    for i in 0..map.len() {
        for j in 0..map[0].len() {

            if map[i][j] == 'S' {
                print!("{}", format!("{}",map[i][j]).green());
            }else if map[i][j] == 'E' {
                print!("{}", format!("{}",map[i][j]).red());
            } else  {    
                let no_cheat_node_string =   format!("{}_{}_0",i,j);
                let cheat_node_string =   format!("{}_{}_1",i,j);

                if predecessors.contains(&no_cheat_node_string) {
                    print!("{}", "O".bright_green());
                }else if predecessors.contains(&cheat_node_string) {
                    print!("{}", "O".yellow());
                } else {
                    print!("{}", format!("{}",map[i][j]));
                }
                
            }                 
        }
        println!("");
    }    
    io::stdout().flush().unwrap();
}

// Dijkstra's algorithm function
fn dijkstra(graph: &HashMap<String, Vec<(String, i32)>>, start: &String) -> HashMap<String, (i32,Vec<String>)> {
    let mut distances: HashMap<String, (i32,Vec<String>)> = HashMap::new();
    let mut heap = BinaryHeap::new();

    // Initialize distances to "infinity" for all nodes except start
    for node in graph.keys() {
        distances.insert(node.to_string(), (i32::MAX, Vec::new()));
    }
    distances.insert(start.to_string(), (0,Vec::new()));

    // Add the starting node to the heap
    heap.push(State { cost: 0, node: start.to_string() });

    // Dijkstra's loop
    while let Some(State { cost, node }) = heap.pop() {
        // Skip processing if we've already found a shorter path
        if cost > distances[&node].0 {
            continue;
        }

        // Process neighbors
        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, weight) in neighbors {
                let next_cost = cost + weight;

                // Relaxation step: Update distance if a shorter path is found
                if next_cost < distances[neighbor].0 {
                    distances.insert(neighbor.clone(), (next_cost, vec![node.clone()]));
                    heap.push(State { cost: next_cost, node: neighbor.clone()});
                } else if next_cost == distances[neighbor].0 {
                    distances.get_mut(neighbor).unwrap().1.push(node.clone());
                }
            }
        }
    }

    distances
}


fn get_path_and_cheats(start_node: String, end_node: String, distances: & HashMap<String, (i32, Vec<String>)>) -> (HashSet<String>, Vec<(String,String)>)  {
    let mut predecessors: HashSet<String> = HashSet::new();
    predecessors.insert(start_node.clone());
    predecessors.insert(end_node.clone());
  
    let mut predecessors_tracked: Vec<String> = distances[&end_node].1.clone();

    let mut last_cell: String = end_node.clone();
    let mut cheat_steps : Vec<(String,String)> = Vec::new();

    while !predecessors_tracked.is_empty() {
        let predecessor = predecessors_tracked.pop().unwrap();
        //println!("pred: {} and last cell: {}", predecessor, last_cell);
        if last_cell.ends_with('1') && predecessor.ends_with('0') {
            let cheat_one = predecessor.clone();
            let cheat_two = last_cell.clone();
            cheat_steps.push((cheat_one,cheat_two));

        }
        predecessors.insert(predecessor.clone());
        last_cell = predecessor.clone();

        for new_pred in distances[&predecessor].1.clone() {
            if new_pred != "" {
                predecessors_tracked.push(new_pred);
            }                
        }            
    }       

    (predecessors, cheat_steps)
}

//Djikstra's algorithm with a novel cost function that tracks position + cheat count
fn part_one(map:  &mut  Vec<Vec<char>>) -> u32 {
    let directions: HashMap<char,(i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0))
    ]);

    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let mut start_node: String = String::from("");
    let mut end_node_with_cheat: String = String::from("");
    let mut end_node_with_no_cheat: String = String::from("");

    //node id will always be string: y_x_cheatcount
    for i in 0..(map.len()) {
        for j in 0..(map[0].len()) {

            if map[i][j] == 'S' {
                start_node = format!("{}_{}_{}", i,j,0);
            }

            if map[i][j] == 'E' {
                end_node_with_cheat = format!("{}_{}_{}", i,j,1);
                end_node_with_no_cheat  = format!("{}_{}_{}", i,j,0);
            }

            let node_id_not_cheated = format!("{}_{}_{}", i,j,0); //id is position + cheat count
            let node_id_has_cheated = format!("{}_{}_{}", i,j,1); //id is position + cheat count
            let mut reachable_nodes_not_cheated: Vec<(String, i32)> = Vec::new();
            let mut reachable_nodes_has_cheated: Vec<(String, i32)> = Vec::new();

            for (_direction_facing, add_ons) in directions.iter() {
                //add neighbor if you can reach it
                let next_y = i as i32 + add_ons.0;
                let next_x = j as i32 + add_ons.1;  

                if next_y < 0 || next_x < 0 || next_y >= map.len() as i32 || next_x >= map[0].len() as i32 {
                    continue;
                }

                if map[next_y as usize][next_x as usize]  == '.' || map[next_y as usize][next_x as usize]  == 'E' {
                    reachable_nodes_not_cheated.push((format!("{}_{}_{}", next_y,next_x,0), 1));
                    reachable_nodes_has_cheated.push((format!("{}_{}_{}", next_y,next_x,1), 1));
                } else if map[next_y as usize][next_x as usize]  == '#'  {
                    reachable_nodes_not_cheated.push((format!("{}_{}_{}", next_y,next_x,1), 1)); // but has now!
                }  
            }

            if map[i][j] != '#' {
                graph.insert(node_id_not_cheated, reachable_nodes_not_cheated); 
            }
            graph.insert(node_id_has_cheated, reachable_nodes_has_cheated); 
        }
    }
    //println!("graph of distances {:#?}", graph);    
    let distances: HashMap<String, (i32, Vec<String>)> = dijkstra(&graph, &start_node);
    //println!(" distances from  {:#?}", distances);
    //get min distance by searching ever direction
    let distance_to_end_without_cheat = distances[&end_node_with_no_cheat].0;
    let distance_to_end_with_cheat = distances[&end_node_with_cheat].0;
    let mut savings = distance_to_end_without_cheat - distance_to_end_with_cheat;
    println!("Without cheating path steps is: {}", distance_to_end_without_cheat);
    let mut worthy_cheat_count: u32 = 0;
    let mut results = get_path_and_cheats(start_node.clone(), end_node_with_cheat.clone(), &distances);

    println!("There are {} cheats that save {} picoseconds", results.1.len(), savings);
    worthy_cheat_count += results.1.len() as u32;

    while (savings >= 100) {

        for cheats in results.1.iter() {
            //println!("cheats: {},{}", cheats.0, cheats.1);
            //set distance to max, rule out this cheat
            let pair_to_eliminate = graph.get_mut(&cheats.0).unwrap();
            pair_to_eliminate.retain(|(key, _)| *key != cheats.1);
            //println!("after retain call {:?}", graph[&cheats.0]);
        }
        let distances: HashMap<String, (i32, Vec<String>)> = dijkstra(&graph, &start_node);
        let distance_to_end_with_cheat = distances[&end_node_with_cheat].0;
        savings = distance_to_end_without_cheat - distance_to_end_with_cheat;
        results = get_path_and_cheats(start_node.clone(), end_node_with_cheat.clone(), &distances);
        println!("There are {} cheats that save {} picoseconds", results.1.len(), savings);

        if savings >= 100 {
            worthy_cheat_count += results.1.len() as u32;
        }
    }
    print_map(map, results.0);
    worthy_cheat_count
}



//Djikstra's algorithm to find best path - then find deducation for cheats
fn part_two(map:  &mut  Vec<Vec<char>>) -> u32 {
    let directions: HashMap<char,(i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0))
    ]);

    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let mut start_node: String = String::from("");
    let mut end_node_with_no_cheat: String = String::from("");

    //node id will not be  y_x_bool for is has_cheated
    //this loop will just populate the no cheat pathways
    for i in 0..(map.len()) {
        for j in 0..(map[0].len()) {

            let node_id = format!("{}_{}_{}", i,j,0);
            let node_id_that_cheated = format!("{}_{}_{}", i,j,1);

            if map[i][j] == 'S' {
                start_node = node_id.clone();
            }
            if map[i][j] == 'E' {
                end_node_with_no_cheat  = node_id.clone();
            }

            let mut reachable_nodes : Vec<(String, i32)> = Vec::new();
            let mut reachable_nodes_that_cheated : Vec<(String, i32)> = Vec::new();
            for (_direction_facing, add_ons) in directions.iter() {
            //add neighbor if you can reach it
            let next_y = i as i32 + add_ons.0;
            let next_x = j as i32 + add_ons.1;  

            if next_y < 0 || next_x < 0 || next_y >= map.len() as i32 || next_x >= map[0].len() as i32 {
                continue;
            }

            if map[next_y as usize][next_x as usize]  == '.' || map[next_y as usize][next_x as usize]  == 'E' {
                reachable_nodes.push((format!("{}_{}_{}", next_y,next_x,0), 1));
                reachable_nodes_that_cheated.push((format!("{}_{}_{}", next_y,next_x,1), 1));
            }   

            }
            graph.insert(node_id, reachable_nodes); 
            graph.insert(node_id_that_cheated, reachable_nodes_that_cheated);
        }
    }

    //println!("graph of distances {:#?}", graph);    
    let distances: HashMap<String, (i32, Vec<String>)> = dijkstra(&graph, &start_node);
    let distance_to_end_without_cheat = distances[&end_node_with_no_cheat].0;
    println!("Distance to end with no cheat is {}", distance_to_end_without_cheat);

    //now add all cheats 2 to 20 picoseconds in duration as if they are wormholes in best path (node where cheating starts -> ends)
    let mut worthy_cheat_count: u32 = 0;
    let mut map_of_time_saved: HashMap<i32, i32> = HashMap::new();

    for i in 0..(map.len()) {
        for j in 0..(map[0].len()) {
            let originating_node= format!("{}_{}_{}", i,j,0);
            //across full block of potential cheat landings, check if possible landing
            for cheat_y in -20..21 as i32 {
                for cheat_x in -20..21 as i32 {
                    let duration_of_cheat = cheat_x.abs() + cheat_y.abs();
                    if duration_of_cheat <= 20 {
                        let next_y = i as i32 + cheat_x;
                        let next_x = j as i32 + cheat_y;

                        if next_y < 0 || next_x < 0 || next_y >= map.len() as i32 || next_x >= map[0].len() as i32 {
                            continue;
                        }

                        if map[next_y as usize][next_x as usize]  == '.' || map[next_y as usize][next_x as usize]  == 'E' {

                            let cheat_landing_id = format!("{}_{}_{}", next_y,next_x,0);
                            let distance_before_cheat_to_end = distances[&cheat_landing_id].0;
                            let distance_before_cheat_to_origin = distances[&originating_node].0;

                            if distance_before_cheat_to_origin == i32::MAX || distance_before_cheat_to_end == i32::MAX {
                                continue;
                            }

                            let savings = (distance_before_cheat_to_end - distance_before_cheat_to_origin) - duration_of_cheat;

                            if savings >= 100 {
                                //println!("found cheat by jumping {} to {} and saving {}", originating_node, cheat_landing_id, savings);
                                worthy_cheat_count += 1;
                                map_of_time_saved.entry(savings).and_modify(|count| *count += 1).or_insert(1);
                            }

                            
                            //store in graph for validation with Djisktra
                            graph.get_mut(&originating_node.clone()).unwrap().push((cheat_landing_id.clone(), duration_of_cheat));
                        }
                    }
                }
            }
        }
    }

    
    for (key,value)  in map_of_time_saved.iter() {
        println!("There are {} cheats that save {} picoseconds", value, key);
    }
    
    worthy_cheat_count
}



fn main() {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut map: Vec<Vec<char>> = value.iter().map(|s| s.chars().collect()).collect();  
            //println!("Answer to part one:  {}", part_one( &mut map));
            println!("Answer to part two:  {}", part_two( &mut map));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        
    }
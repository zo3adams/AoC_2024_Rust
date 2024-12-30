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



fn print_map(map: &Vec<Vec<char>>, predecessors: &HashSet<String>) {

    for i in 0..map.len() {
    for j in 0..map[0].len() {
            let node_str =   format!("{}_{}",i,j);

            if predecessors.contains(&node_str) && map[i][j] != '#' {
                print!("{}", "O".yellow());
            }else {
                print!("{}", format!("{}",map[i][j]));
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
            //println!("Already have a shorter path {}", distances[&node].0);
            continue;
        }

        // Process neighbors
        if let Some(neighbors) = graph.get(&node) {
            //println!("possible  neighbors {:?}", neighbors);
            for (neighbor, weight) in neighbors {
                //println!("Checking neighbor {}", neighbor);
                let next_cost = cost + weight;

                //println!("Found lower cost path to {} at {}", neighbor, next_cost);

                // Relaxation step: Update distance if a shorter path is found
                if next_cost < distances[neighbor].0 {
                    distances.insert(neighbor.clone(), (next_cost, vec![node.clone()]));
                    heap.push(State { cost: next_cost, node: neighbor.clone()});
                } 
            }
        }
    }

    distances
}

fn get_travel_distances(map: &Vec<Vec<char>>,grid_size: usize) -> HashMap<String, Vec<(String, i32)>> {
    let directions: HashMap<char,(i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0))
    ]);
    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();

    //node id will always be string: y_x
    for i in 0..grid_size {
        for j in 0..grid_size {

            if map[i][j] == '#' {
                continue;
            }
            let node_id = format!("{}_{}", i,j);
            //println!("Checking node id {}", node_id);
            let mut reachable_nodes: Vec<(String, i32)> = Vec::new();
            for (_direction_facing, add_ons) in directions.iter() {
                //add neighbor if you can reach it
                let next_y = i as i32 + add_ons.0;
                let next_x = j as i32 + add_ons.1;  

                if next_y < 0 || next_x < 0 || next_y >= grid_size as i32 || next_x >= grid_size as i32 {
                    continue;
                }

                if map[next_y as usize][next_x as usize]  == '.' {
                    let next_node_id = format!("{}_{}", next_y,next_x);
                    //println!("Adding reachable neighbor {}", next_node_id);
                    reachable_nodes.push((next_node_id, 1));
                }          
             
            }
            graph.insert(node_id.clone(), reachable_nodes);   
        }
    }

    graph
}

fn get_path(start_node: String, end_node: String, distances: & HashMap<String, (i32, Vec<String>)>) -> HashSet<String> {
    let mut predecessors: HashSet<String> = HashSet::new();
    predecessors.insert(start_node);
    predecessors.insert(end_node.clone());
    let mut predecessors_tracked: Vec<String> = distances[&end_node.clone()].1.clone();

    while !predecessors_tracked.is_empty() {
        let predecessor = predecessors_tracked.pop().unwrap();
        predecessors.insert(predecessor.clone());

        for new_pred in distances[&predecessor].1.clone() {
            if new_pred != "" {
                predecessors_tracked.push(new_pred);
            }                
        }            
    }  

    predecessors
}

//Djikstra's algorithm with a novel cost function that counts turns, so nodes are position + direction.
fn part_one(bit_positions:  &mut  Vec<(usize,usize)>, grid_size: usize, bits_to_wait_on: usize) -> i32 {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; grid_size]; grid_size];
    let start_node:String = String::from("0_0");

    //populate map
    for i in 0..bits_to_wait_on {
        map[bit_positions[i].0][bit_positions[i].1] = '#';
    }
    let graph = get_travel_distances(&map, grid_size);
    let distances: HashMap<String, (i32, Vec<String>)> = dijkstra(&graph, &start_node);

    //get min distance form start to end
    let end_node = format!("{}_{}",grid_size-1,grid_size-1);
    //println!("checking distance for node {} in the distance graph {:?}", end_node, distances);
    let distance_to_end = distances[&end_node].0;
    //println!("distance to end is  {}", distance_to_end);
   
    distance_to_end
}



fn part_two(bit_positions:  &mut  Vec<(usize,usize)>, grid_size: usize) -> i32 {
    //find shortest path on open  map
    let mut map: Vec<Vec<char>> = vec![vec!['.'; grid_size]; grid_size];
    let start_node:String = String::from("0_0");
    let end_node = format!("{}_{}",grid_size-1,grid_size-1);

    let mut graph = get_travel_distances(&map, grid_size);
    let mut distances: HashMap<String, (i32, Vec<String>)> = dijkstra(&graph, &start_node);
    let mut predecessors = get_path(start_node.clone(), end_node.clone(), &distances);
    println!("Found path {:?} with distance {}", predecessors, distances[&end_node].0);
    
    //add bits until one is in pathway
    let mut index = 0;
    let mut next_bit: String = "".to_string();
    map[bit_positions[index].0][bit_positions[index].1] = '#';

    while distances[&end_node].0 < i32::MAX {    
        while !predecessors.contains(&next_bit) && index < bit_positions.len() {
            next_bit = format!("{}_{}", bit_positions[index].0, bit_positions[index].1);
            map[bit_positions[index].0][bit_positions[index].1] = '#';
            index += 1;
        }
    
        graph = get_travel_distances(&map, grid_size);
        distances = dijkstra(&graph, &start_node);
        predecessors = get_path(start_node.clone(), end_node.clone(), &distances);
        println!("Found path {:?} with distance {}", predecessors, distances[&end_node].0);

        println!("First bit in path is {}", next_bit);
        print_map(&map, &predecessors);
    }

    println!("the last bit before no path exists is {},{}", bit_positions[index-1].1, bit_positions[index-1].0);
    //find new shortest path - or exit
    0
}



fn main() {
    let input = "./src/input.txt";  

    match read_lines(input) {
        Ok(value) => {    
            let mut bit_positions: Vec< (usize,usize)> = Vec::new();
            let grid_size: usize = 71;
            //let bits_to_wait_on: usize = 12;

            for line in value {
                let parts: Vec<usize> = line
                    .split(',')                // Split by comma
                    .map(|num| num.trim())     // Trim whitespace
                    .filter_map(|num| num.parse::<usize>().ok()) // Parse into usize
                    .collect();
        
                if parts.len() == 2 {
                    bit_positions.push((parts[1], parts[0])); // Push tuple into vector
                }
            }
           
            //println!("Answer to part one:  {}", part_one(&mut bit_positions, grid_size, bits_to_wait_on));
            println!("Answer to part two:  {}", part_two(&mut bit_positions, grid_size));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        
    }
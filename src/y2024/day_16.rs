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
                print!("{}", format!("{}",map[i][j]).red());
            }else if map[i][j] == 'E' {
                print!("{}", format!("{}",map[i][j]).green());
            } else  {    
                let node_str =   format!("{}_{}",i,j);

                if predecessors.contains(&node_str) {
                    print!("{}", "O".yellow());
                }else {
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


//Djikstra's algorithm with a novel cost function that counts turns, so nodes are position + direction.
fn part_one(map:  &mut  Vec<Vec<char>>) -> i32 {
    let directions: HashMap<char,(i32, i32)> = HashMap::from([
        ('<', (0, -1)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('v', (1, 0))
    ]);

     
    let mut graph: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let mut reindeer_node: String = String::from("");
    let mut end_node: String = String::from("");

    //node id will always be string: y_x_dir
    for i in 1..(map.len()-1) {
        for j in 1..(map[0].len()-1) {

            if map[i][j] == '#' {
                continue;
            }

            if map[i][j] == 'S' {
                reindeer_node = format!("{}_{}_{}", i,j, '>');
            }

            if map[i][j] == 'E' {
                end_node = format!("{}_{}", i,j);
            }

            for (direction_facing, add_ons) in directions.iter() {
                let node_id = format!("{}_{}_{}", i,j,direction_facing);
                let mut reachable_nodes: Vec<(String, i32)> = Vec::new();

                //add neighbor if you can reach it
                let next_y = i as i32 + add_ons.0;
                let next_x = j as i32 + add_ons.1;                    
                if map[next_y as usize][next_x as usize]  == '.' || map[next_y as usize][next_x as usize]  == 'E' {
                    reachable_nodes.push((format!("{}_{}_{}", next_y,next_x,direction_facing), 1));
                }  

                if *direction_facing == '<' || *direction_facing == '>' {
                    reachable_nodes.push((format!("{}_{}_{}", i,j,'^'), 1000));
                    reachable_nodes.push((format!("{}_{}_{}", i,j,'v'), 1000));
                } else if *direction_facing == '^' || *direction_facing == 'v' {
                    reachable_nodes.push((format!("{}_{}_{}", i,j,'<'), 1000));
                    reachable_nodes.push((format!("{}_{}_{}", i,j,'>'), 1000));
                }
                           
                graph.insert(node_id, reachable_nodes);        
            }
        }
    }
    println!("reindeer starting node {}   end node {}", reindeer_node, end_node);

    //for sanity check print reindeer and 3 layers of movement from it
    println!(" nearest neighbors to reindeer are {:?}", graph[&reindeer_node]);
    //println!("graph of distances {:#?}", graph);    
    let distances: HashMap<String, (i32, Vec<String>)> = dijkstra(&graph, &reindeer_node);
    //println!(" distances from  {:#?}", distances);
    //get min distance by searching ever direction
    let mut min_distance = i32::MAX;
    

    for each_dir in directions.keys() {
        let end_node_str = format!("{}_{}",end_node, each_dir);
        let distance_this_dir = distances[&end_node_str].0;
        //println!("distance facing {} is {}", each_dir, distance_this_dir);
        if distance_this_dir < min_distance {
            min_distance = distance_this_dir;
        }
    }

    //part two right here, no separate method, just count predecessors
    let mut predecessors: HashSet<String> = HashSet::new();
    predecessors.insert(reindeer_node[..reindeer_node.len() - 2].to_string());
    predecessors.insert(end_node.clone());
    for each_dir in directions.keys() {
        let end_node_str = format!("{}_{}",end_node, each_dir);
        let distance_this_dir = distances[&end_node_str].0;

        if distance_this_dir > min_distance {
            continue;
        } //else this was a valid path
        
        let mut predecessors_tracked: Vec<String> = distances[&end_node_str].1.clone();

        while !predecessors_tracked.is_empty() {
            let predecessor = predecessors_tracked.pop().unwrap();

            let without_direction = &predecessor[..predecessor.len() - 2];
            predecessors.insert(without_direction.to_string());

            for new_pred in distances[&predecessor].1.clone() {
                if new_pred != "" {
                    predecessors_tracked.push(new_pred);
                }                
            }            
        }               
    }

    println!("Size of predecessor list (de-duped) is {}", predecessors.len());
    print_map(map, predecessors);


    min_distance
}




pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut map: Vec<Vec<char>> = value.iter().map(|s| s.chars().collect()).collect();  
            println!("Answer to part one:  {}", part_one( &mut map));
            //println!("Answer to part two:  {}", part_two( &mut map));
           
         },
         Err(e) => println!("Error: {}", e),
          
        }
        Ok(())    
    }
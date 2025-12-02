
use std::fs::File;
use std::i64;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};

fn read_lines<P>(filename: P) -> Result<Vec<String>,std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}



fn _part_one(connections:  HashMap<String, Vec<String>>) -> i64 {

    let mut all_triplets: HashSet<Vec<String>> = HashSet::new();
    let mut just_t_triplets: HashSet<Vec<String>> = HashSet::new();

    let all_nodes_list = connections.keys().clone();
    for node in all_nodes_list {
        //println!("node {} connects to {:?}", node, connections[node]);
        for left_connect in connections[node].clone() {
            for right_connect in connections[node].clone() {
                if right_connect != left_connect &&  connections[&(right_connect)].contains(&left_connect) {
                    let mut this_triple: Vec<String> = vec![node.clone(), left_connect.clone(), right_connect.clone()];
                    this_triple.sort();
                    all_triplets.insert(this_triple.clone());

                    if node.starts_with('t') || left_connect.starts_with('t') || right_connect.starts_with('t') {
                        just_t_triplets.insert(this_triple.clone());
                    }
                }
            }
        }
    }
    println!("Got {} triplets {:?}", all_triplets.len(), all_triplets);
    println!("And {} triplets that include a t {:?}", just_t_triplets.len(), just_t_triplets);

    
    just_t_triplets.len() as i64
}
          
fn part_two(connections: HashMap<String, Vec<String>>) -> i64 {
   let mut cliques: Vec<HashSet<String>> = Vec::new();
   let all_nodes_list = connections.keys().clone();
   for _i in 0..3 {
    for node in all_nodes_list.clone() {
        let has_found_clique = false;

        for clique in &mut cliques {
            let mut was_fully_connected = true;
            for clique_node in clique.iter() {
                if !connections[clique_node].contains(node) {
                    was_fully_connected = false;
                    break;
                }
            }

            if was_fully_connected {
                clique.insert(node.clone());
            }
        }

        if ! has_found_clique {
            //make new clique
            cliques.push(HashSet::from([node.clone()]));

        }
   }
   }

   
   println!("Found {} cliques - {:?}", cliques.len(), cliques);
   let mut max_size_seen: usize = 0;
   let mut max_clique: HashSet<String> = HashSet::new();

   for clique in cliques {
       if clique.len()> max_size_seen {
            max_size_seen = clique.len();
            max_clique = clique;
       }  
    }
    println!("Max clique size seen is: {} -- {:?}", max_clique.len(), max_clique);
    let mut max_as_list: Vec<String> = max_clique.into_iter().collect();
    max_as_list.sort();
   let password: String = max_as_list.join(",");
   println!("password {}", password);
   0
}


pub fn run()-> io::Result<()> {
    let input = "./src/input.txt";  


    match read_lines(input) {
        Ok(value) => {    

            let mut connections: HashMap<String, Vec<String>> = HashMap::new();
            for line in value {
                let tokens: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
                connections.entry(tokens[0].clone()).and_modify(|v| v.push(tokens[1].clone())).or_insert(vec![tokens[1].clone()]);
                connections.entry(tokens[1].clone()).and_modify(|v| v.push(tokens[0].clone())).or_insert(vec![tokens[0].clone()]);

            } 
            //println!("Answer to part one:  {}", part_one( connections));
            println!("Answer to part two:  {}", part_two(connections));
         },
         Err(e) => println!("Error: {}", e),
          
        }   
       Ok(())
    }


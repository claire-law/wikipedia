use crate::graphs::DirectedGraph;
use std::collections::VecDeque;
use std::collections::HashMap;

//calculates distance from ONE given node to all other nodes. returns hashmap.
pub fn compute_distances(node: &String, graph: &DirectedGraph) -> HashMap<String,Option<usize>> {
    let mut distance = HashMap::<String,Option<usize>>::new();
    
    if let None = graph.adj_lists.get(node) {
        println!("The page {:?} does not exist in the data.", node);
        return distance;
    } else {   
        distance.insert(node.to_string(), Some(0)); 

        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(node.to_string());

        while let Some(v) = queue.pop_front() { 
            if let Some(neighbors) = graph.adj_lists.get(&v) {
                for u in neighbors.iter() { 
                    if let None = distance.get(u) { 
                        let d = distance.get(&v).unwrap().clone().unwrap() + 1; 
                        queue.push_back(u.clone());
                        distance.insert(u.clone(), Some(d)); 
                    }
                }
            } else { // uncomment to be notified when cases where the page links to another page that is not in the dataset.
                //println!("Dataset does not have the Wikipedia page for: {:?}", v);
            }
        }

        // assigns None for nodes not connected to given node
        for i in graph.adj_lists.keys() {
            if let None = distance.get(i) {
                distance.insert(i.clone(), None);
            }
        }
        distance
    }
}

// computes distance between two given nodes
pub fn how_many_clicks(start: &String, end: &String, graph: &DirectedGraph) {
    if let None = graph.adj_lists.get(start) {
        println!("The page {:?} does not exist in the data.", start);
    } else if let None = graph.adj_lists.get(end) {
        println!("The page {:?} does not exist in the data.", end);
    } else {
        let x = compute_distances(start, &graph);
        let y = x.get(end).unwrap();
        match y {
            Some(_) => println!("It takes {:?} click(s) to get from the {:?} page to the {:?} page.", y.unwrap(), start, end),
            None => println!("There are no hyperlinks connecting the {:?} page to the {:?} page.", start, end)
        }
    }
}

//calls compute_distances for all nodes in the graph. returns hashmap.
pub fn all_distances(graph:&DirectedGraph) -> HashMap<String,HashMap::<String,Option<usize>>> {
    let mut all = HashMap::<String,HashMap<String,Option<usize>>>::new();
    for (node, _c) in graph.adj_lists.iter() {
        all.insert(String::from(node), compute_distances(node, graph));
    }
    all
}

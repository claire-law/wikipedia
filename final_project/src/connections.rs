use crate::graphs::DirectedGraph;
use std::collections::HashMap;

pub fn dfs (graph: &DirectedGraph, graph_reverse:&DirectedGraph) -> HashMap::<usize, Vec<String>> {
    let mut stack: Vec<String> = Vec::new(); 
    let mut visited = HashMap::<String, bool>::new();
    let mut cpnts = HashMap::<usize, Vec<String>>::new();

    //first DFS: iterate through each node
    for (i, _c) in graph.adj_lists.iter() {
        collect_stack(i.to_string(), &graph, &mut stack, &mut visited);
    }

    //second DFS on reversed graph
    let mut component = HashMap::<String, Option<usize>>::new();
    let mut count = 0;

    while let Some(v) = stack.pop() { 
        if let None = component.get(&v) {
            count += 1;
            mark_component(v, graph_reverse, &mut component, count); 
        }
    };

    for c in 0..=count {
        for v in graph.adj_lists.keys() {
            if component.get(&v.to_string()).unwrap().clone().unwrap() == c {
                if let Some(x) = cpnts.get_mut(&c) {
                    x.push(v.to_string());
                } else {
                    cpnts.insert(c, vec![v.to_string()]);
                }
            }
        }
    }
    cpnts
}


fn collect_stack( i:String, 
                graph: &DirectedGraph, 
                stack:&mut Vec<String>, 
                visited:&mut HashMap::<String, bool>) {
    if let None = visited.get(&i) { 
        visited.insert(i.clone(), true); 
        if let Some(neighbors) = graph.adj_lists.get(&i) { 
            for u in neighbors.iter() {
                collect_stack(u.to_string(), graph, stack, visited); 
            }
        } else {}
        stack.push(i.to_string()); // furthest neighbors will be pushed onto stack first, then closer neighbors, then v. 
    }
}


fn mark_component(v:String, graph:&DirectedGraph, component:&mut HashMap<String, Option<usize>>, component_no:usize) {
    component.insert(v.clone(), Some(component_no));
    if let Some(neighbors) = graph.adj_lists.get(&v) { 
        for u in neighbors.iter() {
            if !component.contains_key(u) {
                mark_component(u.to_string(), graph, component, component_no); 
            }
        }
    } else {
        //println!("Node {} has no neighbors", v);
    }
}
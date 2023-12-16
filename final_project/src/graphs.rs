use std::collections::HashMap;

#[derive(Debug)]
pub struct DirectedGraph {
    pub n: usize,
    pub adj_lists: HashMap::<String,Vec<String>>
}

impl DirectedGraph {
    pub fn create(edges:&Vec<(String, String)>) -> DirectedGraph {
        let mut g = DirectedGraph{n:0, adj_lists:HashMap::<String,Vec<String>>::new()};
        for (k,v) in edges.iter() {
            if let Some(x) = g.adj_lists.get_mut(k) { 
                x.push(v.to_string());
            } else {
                g.adj_lists.insert(k.to_string(), vec![v.to_string()]);
            }
        }        
        g.n = g.adj_lists.keys().len();  
        g                                    
    }
}


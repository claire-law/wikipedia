mod reader;
mod graphs;
mod distance;
use graphs::DirectedGraph;

fn main() {
    let e = reader::read_file("links.tsv", true).unwrap();
    let my_graph = DirectedGraph::create(&e);
    println!("Number of edges: {:?}", e.len()); 
    println!("Number of nodes with outgoing edges: {:?}", my_graph.n);

    //example nodes. feel free to change variables a-e to any another page/node.
    let a = "The_Beatles";
    let b = "United_Kingdom"; 
    let c = "Thomas_Edison"; 
    let d = "Human_abdomen";
    let e = "ghasldkadkfjangfjs"; //this represents a page that does not exist in the graph 

    // computes and prints the HashMap containing degree of separation from node b to all other nodes in the graph.
    let x = distance::compute_distances(&String::from(b), &my_graph);
    println!("Distance from {:?} to its connected nodes: {:?}", e, x);

    // prints degree of separation from one node to another
    distance::how_many_clicks(&String::from(a), &String::from(b), &my_graph);
    distance::how_many_clicks(&String::from(b), &String::from(c), &my_graph);
    distance::how_many_clicks(&String::from(c), &String::from(d), &my_graph);
    distance::how_many_clicks(&String::from(e), &String::from(d), &my_graph);
    
    // uncomment next line to print distances from all nodes to all nodes. should take a few minutes. Run "cargo run > output.txt" to see the full output in a separate file.
    //println!("{:?}", distance::all_distances(&my_graph)); 
}

//test to check graph was created with correct n and adjacency lists
#[test]
fn test_create() {    
    let list_of_tuples = vec![(String::from("key1"),String::from("value1")), 
                                (String::from("key2"),String::from("value2")), 
                                (String::from("key3"), String::from("value3")),
                                (String::from("key2"),String::from("hello"))];

    let test = DirectedGraph::create(&list_of_tuples);

    assert_eq!(test.adj_lists.get("key2"), Some(&vec![String::from("value2"), String::from("hello")])); // key2 should link to a vector containing the values "value2" and "hello"
    assert_eq!(test.n, 3); // n should be 3
}

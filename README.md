# DS210 Final Project 
Write-up for DS210 Final Project by Claire Law


DATASET: Navigation paths between Wikipedia pages, collected through the game "Wikispeedia," in which users are asked to navigate from one Wikipedia article to another only by clicking on other Wikipedia pages that are hyperlinked. (Citation: Robert West, Joelle Pineau, and Doina Precup: Wikispeedia: An Online Game for Inferring Semantic Distances between Concepts. 21st International Joint Conference on Artificial Intelligence (IJCAI), 2009.)
- Link to download dataset: https://snap.stanford.edu/data/wikispeedia.html. I downloaded the first link (the file that is 9.5 MB). Once you unzip the folder, you will see a file called "links.tsv", which is the dataset I used. 
- It contains the list of all edges (hyperlinks) between the nodes (articles). In each record, the first item is the node/page that contains the hyperlink to the other node/page listed in the record.
- Note: The article names are URL-encoded, so any unreadable text in the dataset is likely representing a page/node whose title contains non-English characters, such as any characters with accents.


HOW TO RUN: I recommend to run this using cargo run > output.txt because the output is quite long and the terminal will cut it off. Run test using cargo test.


WHAT THE PROJECT DOES: Conducts breadth first search on the graph. There are brief comments within my code to explain what everything does, but here is a more in-depth explanation of all the modules, functions and methods I wrote.
- reader.rs: This module reads in the "links.tsv" file. I included a line to skip all the comments at the top of the file (the citation, extra notes, etc). I also indicated the file has headers to skip the blank line between the notes and the records.
- graphs.rs: Here, I defined my Graph struct, which I called DirectedGraph (just to remind me that I was working with a directed graph). The create method creates an instance of DirectedGraph, with a given list of edges. I decided to make the adjacency list a HashMap with Strings as keys (the starting node that contains the hyperlink), linking to keys that are vectors of Strings (all the pages that the node contains a hyperlink to).
- distance.rs: This module contains all my functions related to the breadth first search algorithm.
  - compute_distances calculates the distance from one given node to all other of the nodes it is connected to, and returns a HashMap (for example, if we call compute_distances with the node "United_Kingdom", it will return a HashMap where they keys are all the nodes in the graph, and the values are the degrees of separation from "United_Kingdom". And if it's not connected to "United_Kingdom", the value is None).
  - all_distances calls compute_distances for all the nodes in the graph and returns a big HashMap containing all the smaller HashMaps computed for each individual node. If you call this function, it takes a while to run, so I have commented out the code in my main.rs file in case you want to see what everything else does when you run the project.
  - how_many_clicks is an auxiliary function that builds on compute_distances, and given a start and end node, it just calls compute_distances and unwraps and prints the degree between the two nodes. It will tell you if it is called with an argument that is not a node in the dataset. I was initially planning on making this function access the big HashMap calculated from all_distances, but after seeing how long it took to call all_distances, I decided to just call have the function call compute_distances every time. 
- main.rs and EXPECTED OUTPUT: I bring in the data and use it to create an instance of DirectedGraph. Then I print basic info about the graph, like number of edges and nodes (that contain outgoing edges. You will notice if you check the length of the HashMap returned by compute_distances, there are a few nodes that were not included in the dataset because they did not link to any pages, but other pages linked to them). I call compute_distances for an example node and print the result, and then call how_many clicks to print the degree of separation from a few example nodes (feel free to change these to other nodes that interest you). I also call all_distances at the end to generate a HashMap containing distances from all nodes to all nodes.


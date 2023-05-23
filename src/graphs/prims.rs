// Greedy algorithm to find the minimum spanning tree of a weighted graph
/*
    1. Start with a vertex, mark it as visited
    2. Find the minimum weight edge that connects the visited vertices to the unvisited vertices
    3. Mark the vertex at the other end of the edge as visited
    4. Repeat steps 2 and 3 until all vertices are visited
 */


pub fn prim(graph: &Graph) -> Vec<Edge> {
    let mut minimal_spanning_tree = Vec::new();
    
}
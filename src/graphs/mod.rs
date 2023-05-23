

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vertex(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    from: Vertex,
    to: Vertex,
    weight: usize,
} 

pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

// other general implementation details

impl Graph {
    pub fn add_vertex(&mut self) -> Vertex {
        let vertex = Vertex(self.vertices.len());
        self.vertices.push(vertex);
        vertex
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn adjacent(&self, vertex: Vertex) -> Vec<(Vertex, usize)> {
        self.edges
            .iter()
            .filter(|e| e.from == vertex)
            .map(|e| (e.to, e.weight))
            .collect()
    }
}

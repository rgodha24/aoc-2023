use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<(usize, usize)>,
}

impl<T: PartialEq + Debug> Graph<T> {
    /// adds an directed edge between 2 nodes
    pub fn add_edge(&mut self, from: &T, to: &T) {
        let from = self.nodes.iter().position(|n| n == from).unwrap();
        let to = self.nodes.iter().position(|n| n == to).unwrap();
        self.edges.push((from, to));
    }

    /// adds an undirected edge between 2 nodes
    pub fn add_undirected_edge(&mut self, from: &T, to: &T) {
        let from = self.nodes.iter().position(|n| n == from).unwrap();
        let to = self.nodes.iter().position(|n| n == to).unwrap();
        self.edges.push((from, to));
        self.edges.push((to, from));
    }

    pub fn add_node(&mut self, node: T) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    /// creates a new graph from a list of nodes
    pub fn new(data: Vec<T>) -> Self {
        Self {
            nodes: data,
            edges: Vec::new(),
        }
    }

    /// stoer-wagner algorithm https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
    pub fn min_cut(&self) -> (Graph<T>, Graph<T>)
    where
        T: Clone,
    {
        todo!()
    }

    fn min_cut_phase:
}

use std::collections::HashMap;

pub struct Graph {
    neighbours: Vec<HashMap<usize, f64>>
}

impl Graph {
    /// Creates a new graph with `n` nodes but no edges.
    ///
    /// To add edges use [add_edge](struct.Graph.html#method.add_edge).
    pub fn new(number_of_nodes: usize) -> Graph {
        Graph {
            neighbours: vec![HashMap::new(); number_of_nodes]
        }
    }

    /// Add an edge to this graph.
    ///
    /// Both `node_a` and `node_b` must be valid nodes in the graph.
    /// `node_a` and `node_b` may not be the same node.
    pub fn add_edge(&mut self, node_a: usize, node_b: usize, edge_cost: f64) {
        assert!(node_b < self.neighbours.len());
        assert!(node_a < self.neighbours.len());
        assert_ne!(node_a, node_b);

        self.neighbours[node_a].insert(node_b, edge_cost);
        self.neighbours[node_b].insert(node_a, edge_cost);
    }

    /// Get neighbours for a specific node.
    ///
    /// `node` must be a valid node in this graph.
    pub fn get_neighbours(&self, node: usize) -> &HashMap<usize, f64> {
        &self.neighbours[node]
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Graph;

    #[test]
    fn new() {
        for n in 0..10 {
            let g = Graph::new(n);
            assert_eq!(g.neighbours.len(), n);
        }
    }

    #[test]
    fn valid_add_edge() {
        let node_a = 0;
        let node_b = 1;
        let edge_cost = 5.0;

        let mut g = Graph::new(10);
        g.add_edge(node_a, node_b, edge_cost);

        assert_eq!(*g.neighbours[node_a].get(&node_b).unwrap(), edge_cost);
        assert_eq!(*g.neighbours[node_b].get(&node_a).unwrap(), edge_cost);
    }

    #[test]
    #[should_panic]
    fn add_self_edge() {
        let mut g = Graph::new(10);
        g.add_edge(0, 0, 5.0);
    }

    #[test]
    #[should_panic]
    fn add_edge_invalid_node() {
        let mut g = Graph::new(10);
        g.add_edge(10, 0, 5.0);
    }
}
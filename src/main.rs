use crate::graph::Graph;
use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

mod graph;

struct Edge {
    node_a: usize,
    node_b: usize,
    edge_cost: f64,
}

fn main() {
    let (number_of_nodes, edges, target_length) = read_input();

    let mut g = Graph::new(number_of_nodes);
    for edge in edges.iter() {
        g.add_edge(edge.node_a, edge.node_b, edge.edge_cost);
    }

    let mut paths = Vec::new();
    for start_node in 0..number_of_nodes {
        if let Some(path) = find_path(&g, start_node, target_length) {
            paths.push(path);
        }
    }

    let best_path = paths
        .into_iter()
        .min_by(|(_, ec1), (_, ec2)| float_ordering(*ec1, *ec2));

    if let Some((best_path, cost)) = best_path {
        println!("The best path with {} nodes cost {}.", target_length, cost);
        println!("The path consists of the following nodes in order:");
        for node in best_path.into_iter() {
            println!("{}", node);
        }
    }
}

/// Find a greedy path through the graph that contains `target_length` nodes that starts
/// at `start_node`.
///
/// `target_length` must be at least one. (The path needs to contain the start node).
/// `start_node` must be a valid node in the graph.
fn find_path(graph: &Graph, start_node: usize, target_length: usize) -> Option<(Vec<usize>, f64)> {
    assert_ne!(target_length, 0);

    let mut path = Vec::with_capacity(target_length);
    let mut cost = 0.0;

    path.push(start_node);

    for _ in 0..(target_length - 1) {
        let n = graph.get_neighbours(*path.last().unwrap());
        let nearest = n
            .iter()
            .filter(|(&node, _)| !path.contains(&node))
            .min_by(|(_, &ec1), (_, &ec2)| float_ordering(ec1, ec2));

        if let Some((&nearest_node, &edge_cost)) = nearest {
            path.push(nearest_node);
            cost += edge_cost;
        } else {
            return None;
        }
    }

    return Some((path, cost));
}

fn float_ordering(a: f64, b: f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn read_input() -> (usize, Vec<Edge>, usize) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let v: Vec<_> = first_line.split(' ').map(|s| s.to_string()).collect();
    let number_of_nodes: usize = v[0].parse().unwrap();
    let number_of_edges: usize = v[1].parse().unwrap();
    let target_length: usize = v[2].parse().unwrap();

    let mut edges = Vec::with_capacity(number_of_edges);
    for _ in 0..number_of_edges {
        let line = lines.next().unwrap().unwrap();
        let v: Vec<_> = line.split(' ').map(|s| s.to_string()).collect();
        let node_a: usize = v[0].parse().unwrap();
        let node_b: usize = v[1].parse().unwrap();
        let edge_cost: f64 = v[2].parse().unwrap();

        assert_ne!(node_a, node_b);
        assert!(node_a < number_of_nodes);
        assert!(node_b < number_of_nodes);

        edges.push(Edge {
            node_a,
            node_b,
            edge_cost,
        })
    }

    (number_of_nodes, edges, target_length)
}

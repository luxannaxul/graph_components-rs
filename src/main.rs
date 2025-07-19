use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use rand::{Rng, rng};
use textplots::{Chart, Plot, Shape};

fn main() {
    test_performance();
}

fn print_chart() {
    Chart::new(360, 120, 0., 10000.)
        .lineplot(&Shape::Continuous(Box::new(|x| {
            components_with_x_nodes_y_edges_per_node(x as i32, 5) as f32
        })))
        .display();
    test_performance();
}

fn components_with_x_nodes_y_edges_per_node(x: i32, y: i32) -> i32 {
    let mut rng = rng();
    let mut g = Graph::new();
    for _ in 0..x {
        g.add_node();
    }
    for _ in 0..x {
        for _ in 0..y {
            g.add_edge(rng.random_range(0..x), rng.random_range(0..x), 0);
        }
    }
    let mut c = ConnectedComponents::new();
    c.count_connected_components(&g)
}

fn test_performance() {
    let global_start_time = Instant::now();
    let mut rng = rng();
    println!("this is a rust implementation");
    let mut g = Graph::new();
    let node_count = 2_000_000;
    let edges_per_node = 12;
    let edge_count = node_count * edges_per_node;
    for _ in 0..node_count {
        g.add_node();
    }
    println!(
        "initialized {node_count} nodes in {:?}",
        Instant::elapsed(&global_start_time)
    );
    let before_edges = Instant::now();
    for _ in 0..node_count {
        for _ in 0..edges_per_node {
            g.add_edge(
                rng.random_range(0..node_count),
                rng.random_range(0..node_count),
                0,
            );
        }
    }
    println!(
        "added {edge_count} edges, equivalent to {edges_per_node} edges per node in {:?}",
        Instant::elapsed(&before_edges)
    );
    let before_counting_components = Instant::now();
    let mut c = ConnectedComponents::new();
    let components_count = c.count_connected_components(&g);
    let pretty_string = match components_count {
        1 => String::from(
            "this graph has one component ==> every node is reachable from every other node",
        ),
        count => format!("this graph has {count} components"),
    };
    println!("{}", pretty_string);
    println!(
        "counting components took {:?}",
        Instant::elapsed(&before_counting_components)
    );
    println!(
        "total run time was {:?}",
        Instant::elapsed(&global_start_time)
    );
}

struct Edge {
    to: i32,
    weight: i32,
}

struct Graph {
    edge_lists: Vec<Vec<Edge>>,
}

struct ConnectedComponents {
    bfs: BFS,
    found_indices: HashSet<i32>,
}

struct BFS {
    connected_nodes: HashSet<i32>,
    parents: HashMap<i32, i32>,
    distances: HashMap<i32, i32>,
    queue: VecDeque<i32>,
}

impl BFS {
    fn new() -> Self {
        Self {
            connected_nodes: HashSet::new(),
            parents: HashMap::new(),
            distances: HashMap::new(),
            queue: VecDeque::new(),
        }
    }
    fn sssp(&mut self, g: &Graph, s: i32) {
        self.connected_nodes.clear();
        self.parents.clear();
        self.distances.clear();
        self.queue.clear();

        self.queue.push_back(s);
        self.connected_nodes.insert(s);
        self.parents.insert(s, s);
        self.distances.insert(s, 0);
        while !self.queue.is_empty() {
            let u = self.queue.pop_front().unwrap();
            // u --> v
            g.get_all_neighbours(u).iter().for_each(|v| {
                if !self.parents.contains_key(v) {
                    self.queue.push_back(*v);
                    self.connected_nodes.insert(*v);
                    self.parents.insert(*v, u);
                    self.distances
                        .insert(*v, self.distances.get(&u).unwrap() + 1);
                }
            });
        }
    }
    fn depth(&self, node: i32) -> i32 {
        match self.distances.contains_key(&node) {
            true => *self.distances.get(&node).unwrap(),
            false => -1,
        }
    }
    fn parent(&self, node: i32) -> i32 {
        match self.parents.contains_key(&node) {
            true => *self.parents.get(&node).unwrap(),
            false => -1,
        }
    }
    fn connected_nodes(&self) -> &HashSet<i32> {
        &self.connected_nodes
    }
}

impl ConnectedComponents {
    fn new() -> Self {
        Self {
            bfs: BFS::new(),
            found_indices: HashSet::new(),
        }
    }
    fn count_connected_components(&mut self, g: &Graph) -> i32 {
        self.found_indices.clear();
        let mut components = 0;
        for i in 0..g.get_number_of_nodes() {
            if !self.found_indices.contains(&i) {
                components += 1;
                self.bfs.sssp(&g, i);
                self.found_indices.extend(self.bfs.connected_nodes());
            }
        }
        components
    }
}

impl Edge {
    fn new(to: i32, weight: i32) -> Self {
        Self { to, weight }
    }
}

impl Graph {
    fn new() -> Self {
        Self {
            edge_lists: Vec::new(),
        }
    }
    fn add_node(&mut self) {
        self.edge_lists.push(Vec::new());
    }
    fn add_edge(&mut self, from: i32, to: i32, weight: i32) {
        self.edge_lists[from as usize].push(Edge::new(to, weight));
    }
    fn add_bidirectional_edge(&mut self, a: i32, b: i32, weight: i32) {
        self.add_edge(a, b, weight);
        self.add_edge(b, a, weight);
    }
    fn get_shortest_weight_between_neighbours(&self, from: i32, to: i32) -> i32 {
        match self.edge_lists[from as usize]
            .iter()
            .filter(|edge| edge.to == to)
            .map(|edge| edge.weight)
            .min()
        {
            Some(min) => min,
            None => i32::MAX,
        }
    }
    fn get_all_neighbours(&self, from: i32) -> Vec<i32> {
        self.edge_lists[from as usize]
            .iter()
            .map(|edge| edge.to)
            .collect()
    }
    fn get_number_of_nodes(&self) -> i32 {
        self.edge_lists.len() as i32
    }
}

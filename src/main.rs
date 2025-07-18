fn main() {
    println!("Hello, world!");
}

struct Edge {
    to: i32,
    weight: i32,
}

struct Graph {
    edge_lists: Vec<Vec<Edge>>,
}

impl Graph {
    fn add_node(&mut self) {
        self.edge_lists.push(Vec::new());
    }
    fn add_edge(&mut self, from: i32, to: i32, weight: i32) {
        self.edge_lists[from as usize].push(Edge { to, weight });
    }
}

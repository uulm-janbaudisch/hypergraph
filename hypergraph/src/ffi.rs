type Hypergraph = crate::Hypergraph<usize>;

fn new_hypergraph(num_vertices: usize, num_nets: usize) -> Box<Hypergraph> {
    Box::new(Hypergraph::new_unweighted(num_vertices, num_nets))
}

#[cxx::bridge(namespace = "hypergraph")]
mod hypergraph {
    extern "Rust" {
        type Hypergraph;
        fn new_hypergraph(num_vertices: usize, num_nets: usize) -> Box<Hypergraph>;
        fn add_pin(&mut self, net: usize, vertex: usize);
        fn partition_random(&self, blocks: usize, imbalance: f32) -> Vec<usize>;
        fn partition_bfs(&self, blocks: usize) -> Vec<usize>;
        fn partition_dfs(&self, blocks: usize) -> Vec<usize>;
    }
}

use super::{Partition, PartitionManager};
use crate::Hypergraph;

impl Hypergraph<usize> {
    /// Partitions the hypergraph based on [depth-first search](https://en.wikipedia.org/wiki/Depth-first_search).
    ///
    /// Currently, no imbalance parameter is supported.
    pub fn partition_dfs(&self, blocks: usize) -> Partition {
        // Structure to keep track of the partition.
        let mut partition = PartitionManager::new(blocks, self.len(), 0f32);

        // Calculate how many vertices should be taken per block.
        // The ceiling is taken to ensure that all vertices are considered.
        let vertices_per_block = (self.len() as f32 / blocks as f32).ceil() as usize;

        // Initiate the depth-first search.
        let mut dfs = self.dfs();

        // Consider each block ...
        for block in 0..blocks {
            // ... and assign it the calculated amount of vertices from the search.
            dfs.by_ref()
                .take(vertices_per_block)
                // Get the weight of each vertex ...
                .map(|index| {
                    (
                        index,
                        self.vertices.get(&index).expect("Vertex not found.").weight,
                    )
                })
                // ... and add it to the partition.
                .for_each(|(vertex, weight)| {
                    partition.add(block, vertex, weight);
                });
        }

        partition.blocks
    }
}

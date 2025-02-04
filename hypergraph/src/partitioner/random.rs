use super::{Partition, PartitionManager};
use crate::Hypergraph;
use rand::distr::{Distribution, Uniform};
use rand::seq::SliceRandom;

impl Hypergraph<usize> {
    pub fn partition_random(&self, blocks: usize, imbalance: f32) -> Partition {
        // RNG for sampling the vertices.
        let mut rng = rand::rng();

        // Sampler for choosing a block a vertex goes into.
        let range = Uniform::new(0, blocks).expect("Invalid block range.");

        // Structure to keep track of the partition.
        let mut partition = PartitionManager::new(blocks, self.len(), imbalance);

        // Shuffle the vertices before assigning them to blocks.
        let mut vertices: Vec<usize> = (0..self.len()).collect();
        vertices.shuffle(&mut rng);

        // Consider each vertex ...
        vertices.iter().for_each(|&vertex| {
            // ... decide which block it goes to ...
            let block = range.sample(&mut rng);

            // and check whether that would conflict with the balance requirement.
            if partition.is_balanced(block, Some(1)) {
                // If not, add it.
                partition.add(block, vertex, 1);
            } else {
                // Otherwise, it goes to the next block.
                let next_block = (block + 1) % blocks;
                partition.add(next_block, vertex, 1);
            }
        });

        partition.blocks
    }
}

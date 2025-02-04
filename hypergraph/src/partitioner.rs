mod bfs;
mod dfs;
mod partition;
mod random;

pub use partition::Partition;

/// Keeps track of a partition of a hypergraph.
struct PartitionManager {
    /// The actual partition.
    pub blocks: Partition,
    /// Weights of each block.
    pub weights: Vec<usize>,
    /// The amount of blocks in the partition.
    pub num_blocks: usize,
    /// The allowed imbalance parameter.
    pub imbalance: f32,
}

impl PartitionManager {
    /// Initializes a new partition manager.
    pub fn new(num_blocks: usize, num_vertices: usize, imbalance: f32) -> PartitionManager {
        Self {
            blocks: Partition::from(vec![0; num_vertices]),
            weights: vec![0; num_blocks],
            num_blocks,
            imbalance,
        }
    }

    /// Adds the given vertex with a weight to the specified block.
    pub fn add(&mut self, block: usize, vertex: usize, weight: usize) {
        self.blocks[vertex] = block;
        self.weights[block] += weight;
    }

    /// Checks whether the imbalance of the given block is below the configured tolerance.
    ///
    /// Optionally, the weight of a candidate to add to the given block can be given to include it
    /// in the calculation.
    pub fn is_balanced(&self, block: usize, candidate: Option<usize>) -> bool {
        self.imbalance(block, candidate) <= self.imbalance
    }

    /// Calculates the imbalance of a given block.
    ///
    /// It is calculated as the difference between ideal weight of a block and the actual weight of
    /// the specified one.
    ///
    /// Optionally, the weight of a candidate to add to the given block can be given to include it
    /// in the calculation.
    ///
    /// The returned balance is non-negative.
    pub fn imbalance(&self, block: usize, candidate: Option<usize>) -> f32 {
        // The weight of the whole (current) partition.
        let total = self.weights.iter().sum::<usize>() as f32;

        // The weight of the specified block.
        let mut partial = self.weights[block] as f32;

        // Optionally add a candidate to the block weight.
        if let Some(candidate_weight) = candidate {
            partial += candidate_weight as f32;
        }

        // The weight per block we would expect for a balanced partition.
        let expected_partial = total / self.num_blocks as f32;

        // The result is the difference between what is expected and the actual value.
        (expected_partial - partial).abs()
    }
}

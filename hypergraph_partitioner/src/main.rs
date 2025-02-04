mod mode;

use clap::Parser;
use hypergraph::Hypergraph;
use hypergraph_formats::{hmetis::FromStringHMETIS, Graph};
use mode::Mode;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// hMETIS input file
    #[arg(short, long, env)]
    input: PathBuf,

    /// Where to write the partition to
    #[arg(short, long, env)]
    output: PathBuf,

    /// Which partitioning algorithm to use
    #[arg(short, long, env)]
    mode: Mode,

    /// How many partition blocks to create
    #[arg(short, long, env, default_value_t = 2)]
    blocks: usize,

    /// Block-imbalance tolerance
    #[arg(short, long, env)]
    epsilon: Option<f32>,
}

pub fn main() {
    // Parse the arguments.
    let args = Args::parse();

    // Read the input file.
    let input = fs::read_to_string(&args.input).expect("Failed to read input file");

    // Parse the input as a hMETIS graph.
    let hmetis_graph = Graph::from_string_hmetis(&input).expect("Failed to parse hMETIS graph.");

    // Convert the hMETIS graph into the actual hypergraph used for partitioning.
    let hypergraph = Hypergraph::from(&hmetis_graph);

    // Partition the hypergraph based on the chosen mode.
    let partition = match args.mode {
        Mode::Bfs => hypergraph.partition_bfs(args.blocks),
        Mode::Dfs => hypergraph.partition_dfs(args.blocks),
        Mode::Random => hypergraph.partition_random(
            args.blocks,
            args.epsilon
                .expect("Random partitioning requires the `epsilon` imbalance parameter."),
        ),
    };

    // Write the buffer into the output file.
    let mut output = File::create(args.output).expect("Failed to create output file.");
    output
        .write_all(partition.to_string().as_bytes())
        .expect("Failed to write to output file.");
}

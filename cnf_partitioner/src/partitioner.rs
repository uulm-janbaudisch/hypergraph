pub mod kahypar;
pub mod mtkahypar;
pub mod patoh;
pub mod random;

use crate::partitioner::kahypar::KaHyPar;
use crate::partitioner::random::Random;
use crate::{Args, Partition};
use hypergraph_formats::Graph;
use log::{error, info, trace};
use mtkahypar::MtKaHyPar;
use patoh::PaToH;
use std::io::Write;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use tempfile::NamedTempFile;

pub trait Partitioner {
    /// Returns the name of this partitioner.
    fn name_full(&self) -> String;

    /// Returns the short name of this partitioner.
    fn name_short(&self) -> &'static str;

    fn blocks(&self) -> usize;

    /// Creates a string representation for the given graph.
    fn serialize_graph(&self, graph: &Graph) -> String;

    /// Creates command for starting the partitioning process for the given hypergraph file.
    fn create_run(&self, graph: &Path) -> Command;

    /// Calculates the path of the resulting partiton output file.
    fn output_file(&self, graph: &Path) -> PathBuf;

    /// Runs the partitioner.
    fn run(&self, graph: &Graph) -> Result<(u128, Partition), ()> {
        // Write the graph to a temporary file.
        let mut graph_file =
            NamedTempFile::new().expect("Failed to create temporary file for graph.");

        graph_file
            .write_all(self.serialize_graph(graph).as_bytes())
            .expect("Failed to write graph to temporary file.");

        // Close it and keep a handle for the (temporary) path.
        let graph_file = graph_file.into_temp_path();

        // Create the command.
        let mut run = self.create_run(&graph_file);

        info!("Running {} ...", self.name_full());
        trace!("{:?}", run);

        // Run the partitioner while tracking the time taken.
        let start = Instant::now();
        let output = run.output();
        let duration = start.elapsed().as_millis();

        // Simply abort this function in case of error.
        if let Err(error) = output {
            error!("Failed to run {}: {}", self.name_full(), error);
            return Err(());
        }

        let output = output.unwrap();

        // Also abort in case the process did not finish successfully.
        if !output.status.success() {
            error!("Failed to run {}:", self.name_full());
            error!("stderr: {}", String::from_utf8(output.stderr).unwrap());
            error!("stdout: {}", String::from_utf8(output.stdout).unwrap());
            return Err(());
        }

        // Inform about the time taken and where the output file is.
        info!("{} took {} ms.", self.name_full(), duration);

        Ok((duration, self.output_file(&graph_file).into()))
    }
}

/// Creates instances of all available hypergraph partitioners possible with the given arguments.
pub fn instantiate(args: &Args) -> Vec<Box<dyn Partitioner>> {
    let mut instances: Vec<Box<dyn Partitioner>> = Vec::new();

    for blocks in calculate_blocks(args) {
        if args.kahypar_enable {
            if let Some(path) = args.kahypar_path.clone() {
                instances.push(Box::new(KaHyPar::new(
                    path,
                    blocks,
                    args.kahypar_metric,
                    args.kahypar_epsilon.clone(),
                )));
            }
        }

        if args.mtkahypar_enable {
            if let Some(path) = args.mtkahypar_path.clone() {
                instances.push(Box::new(MtKaHyPar::new(
                    path,
                    blocks,
                    args.mtkahypar_metric,
                    args.mtkahypar_epsilon.clone(),
                )));
            }
        }

        if args.patoh_enable {
            if let Some(path) = args.patoh_path.clone() {
                instances.push(Box::new(PaToH::new(
                    path,
                    blocks,
                    args.patoh_metric,
                    args.patoh_preset,
                )));
            }
        }

        if args.random_enable {
            if let Some(path) = args.rust_path.clone() {
                instances.push(Box::new(Random::new(path, blocks)));
            }
        }
    }

    instances
}

/// Calculates the range of blocks to consider based on the given arguments.
pub fn calculate_blocks(args: &Args) -> RangeInclusive<usize> {
    if args.blocks.is_some() {
        assert!(
            args.blocks_start.is_none() && args.blocks_end.is_none(),
            "Either blocks or blocks_start and blocks_end must be specified."
        );

        assert!(args.blocks.unwrap() >= 2);
    }

    if args.blocks.is_none() {
        assert!(
            args.blocks_start.is_some() && args.blocks_end.is_some(),
            "Either blocks or blocks_start and blocks_end must be specified."
        );

        assert!(args.blocks_start.unwrap() <= args.blocks_end.unwrap());
    }

    if args.blocks.is_some() {
        args.blocks.unwrap()..=args.blocks.unwrap()
    } else {
        args.blocks_start.unwrap()..=args.blocks_end.unwrap()
    }
}

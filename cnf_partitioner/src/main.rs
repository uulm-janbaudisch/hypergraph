mod cnf;
mod d4;
mod output;
mod partitioner;

use crate::cnf::{
    condition_instance, find_assignment, get_cut_variables, serialize_cnf, split_cnf,
};
use crate::output::{Output, Run};
use clap::Parser;
use d4::D4;
use dimacs::parse_dimacs;
use hypergraph::Partition;
use hypergraph_formats::Graph;
use log::{info, LevelFilter};
use num::BigInt;
use partitioner::{kahypar, mtkahypar, patoh};
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::{fs, io};
use tempfile::NamedTempFile;

#[derive(Parser, Debug)]
struct Args {
    /// CNF DIMACS input file.
    input: PathBuf,

    /// How many blocks to create
    #[arg(short, long, env)]
    blocks: Option<usize>,

    /// Lower bound of how many blocks to create
    #[arg(long, env)]
    blocks_start: Option<usize>,

    /// Upper bound of how many blocks to create
    #[arg(long, env)]
    blocks_end: Option<usize>,

    /// Maximum allowed time in seconds for a d4 run
    #[arg(short, long, env)]
    timeout: Option<u64>,

    /// Path where to write the preprocessed CNF to.
    #[arg(long, env)]
    save_cnf: Option<PathBuf>,

    /// Path where to write partition outputs to.
    /// Will get the name of the partitioner appended.
    /// `file.txt` becomes `file.txt.<partitioner>`
    #[arg(long, env)]
    save_partitions: Option<PathBuf>,

    /// Whether to skip running d4 on the original instance.
    /// If specified, the output will contain 0s for values on the original instance.
    #[arg(long, env)]
    skip_original: bool,

    /// Path to the d4 executable
    #[arg(short, long, env)]
    d4_path: PathBuf,

    /// Which logging level to use
    #[arg(short, long, env, default_value_t = LevelFilter::Info)]
    logging: LevelFilter,

    /// Whether to use KaHyPar
    #[arg(long, env)]
    kahypar_enable: bool,

    /// Path to the KaHyPar executable
    #[arg(long, env)]
    kahypar_path: Option<PathBuf>,

    /// Which KaHyPar metric to use
    #[arg(long, env, default_value_t = kahypar::Metric::Cut)]
    kahypar_metric: kahypar::Metric,

    /// Which KaHyPar epsilon (imbalance parameter) to use
    #[arg(long, env, default_value = "0.1")]
    kahypar_epsilon: String,

    /// Whether to use Mt-KaHyPar
    #[arg(long, env)]
    mtkahypar_enable: bool,

    /// Path to the Mt-KaHyPar executable
    #[arg(long, env)]
    mtkahypar_path: Option<PathBuf>,

    /// Which Mt-KaHyPar metric to use
    #[arg(long, env, default_value_t = mtkahypar::Metric::Cut)]
    mtkahypar_metric: mtkahypar::Metric,

    /// Which Mt-KaHyPar epsilon (imbalance parameter) to use
    #[arg(long, env, default_value = "0.1")]
    mtkahypar_epsilon: String,

    /// Whether to use PaToH
    #[arg(long, env)]
    patoh_enable: bool,

    /// Path to the PaToH executable
    #[arg(long, env)]
    patoh_path: Option<PathBuf>,

    /// Which PaToH metric to use
    #[arg(long, env, default_value_t = patoh::Metric::Cut)]
    patoh_metric: patoh::Metric,

    /// Which PaToH preset to use
    #[arg(long, env, default_value_t = patoh::Preset::Default)]
    patoh_preset: patoh::Preset,

    /// Whether to enable the random partitioner
    #[arg(long, env)]
    random_enable: bool,

    /// Path to the Rust partitioner executable
    #[arg(long, env)]
    rust_path: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    pretty_env_logger::formatted_builder()
        .filter_level(args.logging)
        .init();

    let d4 = D4::new(args.d4_path.clone());

    // Initially print the CSV header.
    print!("{}", Run::csv_header());

    // Instantiate all possible partitioners.
    let partitioners = partitioner::instantiate(&args);

    // Initialize the collection of partitioner results.
    let mut output = Output::new();

    info!("Running on {}", args.input.display());

    // Create a temporary file for the preprocessed CNF.
    let preprocessed = NamedTempFile::new()
        .expect("Failed to create temporary file for preprocessed CNF.")
        .into_temp_path();

    info!("Preprocessing the input.");

    // Preprocess the CNF using d4.
    let _time = d4
        .preprocess(args.input.clone(), &preprocessed)
        .expect("Failed to preprocess input.");

    let input = fs::read_to_string(preprocessed);

    // Read the preprocessed DIMACS file.
    let content = match input {
        Ok(content) => content,
        Err(ref error) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Failed to read CNF file {:?}: {}", input, error),
            ));
        }
    };

    // Save the CNF if requested.
    if let Some(path) = args.save_cnf {
        fs::write(path, content.clone()).expect("Failed to save CNF.");
    };

    // Parse it as a CNF instance.
    let cnf = match parse_dimacs(&content) {
        Ok(instance) => instance,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse input DIMACS: {:?}", error),
            ));
        }
    };

    // Write it to a temporary file.
    let mut file = NamedTempFile::new().expect("Failed to create temporary file for CNF.");
    file.write_all(serialize_cnf(&cnf).as_bytes())
        .expect("Failed to write CNF.");

    info!("Running d4 on the original CNF.");

    // Compile the CNF using d4.
    let (original_time, original_count) = if args.skip_original {
        (0, BigInt::ZERO)
    } else {
        d4.compile(file.into_temp_path().to_path_buf(), args.timeout)
            .expect("Failed to compile CNF.")
    };

    // Transform it into a dual hypergraph.
    let graph = Graph::from(&cnf).dual();

    for partitioner in &partitioners {
        // Generate the partition.
        let (partitioning_time, partition) =
            partitioner.run(&graph).expect("Failed to run partitioner");

        // Save the partition if requested.
        if let Some(path) = &args.save_partitions.clone().map(|path| {
            let mut path = path.into_os_string();
            path.push(".");
            path.push(partitioner.name_short());
            path
        }) {
            fs::write(path, partition.to_string()).expect("Failed to save partition.");
        };

        // Split the original CNF into the respective CNFs as defined by the partition.
        let cnfs = split_cnf(&partition, &cnf);

        // Calculate the cut set.
        let cut = get_cut_variables(&cnfs);
        info!("{} cut size: {}", partitioner.name_full(), cut.len());

        // Find an assignment that splits the CNFs.
        let assignment = find_assignment(&cnf, &cut);

        // Run d4 on the original instance with the assignment.
        let (conditioned_time, conditioned_count) = {
            let conditioned_cnf = condition_instance(&cnf, &assignment);

            // Write it to a temporary file.
            let mut file = NamedTempFile::new().expect("Failed to create temporary file for CNF.");
            file.write_all(serialize_cnf(&conditioned_cnf).as_bytes())
                .expect("Failed to write CNF.");

            info!("Running d4 on the original CNF with the split assignment.");

            // Compile the CNF using d4.
            d4.compile(file.into_temp_path().to_path_buf(), None)
                .expect("Failed to compile CNF.")
        };

        // Condition the CNFs on the assignment.
        let cnfs = cnfs.iter().map(|cnf| condition_instance(cnf, &assignment));

        let mut run = Run::new(
            args.input
                .file_stem()
                .expect("Failed to extract input file stem.")
                .to_str()
                .expect("Failed to convert input file name to string.")
                .to_string(),
            partitioner.name_short(),
            partitioner.blocks(),
            cut.len(),
            original_time,
            original_count.clone(),
            conditioned_time,
            conditioned_count,
            partitioning_time,
        );

        info!("Running d4 on each split CNF.");

        // Solve each split CNF.
        cnfs.for_each(|cnf| {
            // Write it to a temporary file.
            let mut file = NamedTempFile::new().expect("Failed to create temporary file for CNF.");
            file.write_all(serialize_cnf(&cnf).as_bytes())
                .expect("Failed to write CNF.");

            // Compile the CNF using d4.
            let (time, count) = d4
                .compile(file.into_temp_path().to_path_buf(), args.timeout)
                .expect("Failed to compile CNF.");

            run.add_part(time, count);
        });

        run.check();
        output.add(run);
    }

    // Print the results of this file.
    print!("{}", output.csv());

    Ok(())
}

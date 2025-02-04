mod cnf;
mod csv;
mod dimension;
mod utils;
mod width;

use crate::cnf::serialize_cnf;
use clap::Parser;
use cnf::{get_cut_variables, split_cnf};
use csv::to_csv;
use dimacs::parse_dimacs;
use dimension::{num_clauses, num_literals, num_variables};
use hypergraph::partitioner::Partition;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use width::{clause_density, clause_width};

#[derive(Parser, Debug)]
struct Args {
    /// CNF DIMACS input file.
    #[arg(short, long, env)]
    cnf: PathBuf,

    /// Partition input file.
    #[arg(short, long, env)]
    partition: PathBuf,

    /// Where to write split CNFs to.
    /// Will get the block numbers appended.
    #[arg(short, long, env)]
    output_cnfs: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read the input CNF ...
    let cnf = fs::read_to_string(args.cnf)?;

    // and the partition.
    let partition = Partition::from(args.partition);

    // Parse the CNF instance.
    let original = match parse_dimacs(&cnf) {
        Ok(instance) => instance,
        Err(error) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse input DIMACS: {:?}", error),
            ));
        }
    };

    let split = split_cnf(&partition, &original);

    let output_path = args.output_cnfs.into_os_string();

    for (i, instance) in split.iter().enumerate() {
        let mut path = output_path.clone();
        path.push(".");
        path.push(i.to_string());
        path.push(".cnf");
        fs::write(path, serialize_cnf(instance)).expect("Failed to save split CNF.");
    }

    let cut = get_cut_variables(&split);

    println!("cut_size,num_clauses_original,num_clauses_split,num_variables_original,num_variables_split,num_literals_original,num_literals_split,width_original,width_split,density_original,density_split");
    println!(
        "{},{},{},{},{},{}",
        cut.len(),
        to_csv(num_clauses(&original, &split)),
        to_csv(num_variables(&original, &split)),
        to_csv(num_literals(&original, &split)),
        to_csv(clause_width(&original, &split)),
        to_csv(clause_density(&original, &split))
    );

    Ok(())
}

use super::Partitioner;
use hypergraph_formats::hmetis::ToStringHMETIS;
use hypergraph_formats::Graph;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{NamedTempFile, TempPath};

pub struct Random {
    path: PathBuf,
    blocks: usize,
    output_path: TempPath,
}

impl Random {
    pub fn new(path: PathBuf, blocks: usize) -> Self {
        Self {
            path,
            blocks,
            output_path: NamedTempFile::new()
                .expect("Failed to create temporary file for partition.")
                .into_temp_path(),
        }
    }
}

impl Partitioner for Random {
    fn name_full(&self) -> String {
        "Random".to_string()
    }
    fn name_short(&self) -> &'static str {
        "Random"
    }

    fn blocks(&self) -> usize {
        self.blocks
    }

    fn serialize_graph(&self, graph: &Graph) -> String {
        graph.to_string_hmetis()
    }

    fn create_run(&self, graph: &Path) -> Command {
        let mut run = Command::new(&self.path);

        run.args([
            "--input",
            graph.to_str().unwrap(),
            "--blocks",
            &self.blocks.to_string(),
            "--mode",
            "random",
            "--epsilon",
            "0.1",
            "--output",
            self.output_path.to_str().unwrap(),
        ]);

        run
    }

    fn output_file(&self, _: &Path) -> PathBuf {
        self.output_path.to_owned()
    }
}

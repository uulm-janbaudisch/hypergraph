use super::Partitioner;
use clap::ValueEnum;
use hypergraph_formats::hmetis::ToStringHMETIS;
use hypergraph_formats::Graph;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use strum::Display;
use tempfile::{NamedTempFile, TempPath};

pub struct KaHyPar {
    path: PathBuf,
    blocks: usize,
    metric: Metric,
    epsilon: String,
    preset: Preset,
    preset_file: TempPath,
}

#[derive(Debug, Display, Copy, Clone, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Metric {
    Cut,
    Km1,
}

#[derive(Debug, Display, Copy, Clone, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Preset {
    Default,
}

impl Preset {
    fn load(&self) -> &[u8] {
        match self {
            Preset::Default => include_bytes!("kahypar/default.ini"),
        }
    }
}

impl KaHyPar {
    pub fn new(path: PathBuf, blocks: usize, metric: Metric, epsilon: String) -> Self {
        let preset = Preset::Default;

        // Write the preset to a temporary file.
        let mut preset_file = NamedTempFile::new().expect("Failed to create temporary file.");
        preset_file
            .write_all(preset.load())
            .expect("Failed to write graph to temporary file.");

        Self {
            path,
            blocks,
            metric,
            epsilon,
            preset,
            preset_file: preset_file.into_temp_path(),
        }
    }
}

impl Partitioner for KaHyPar {
    fn name_full(&self) -> String {
        format!("KaHyPar (preset: {}, metric: {})", self.preset, self.metric)
    }

    fn name_short(&self) -> &'static str {
        "KaHyPar"
    }

    fn blocks(&self) -> usize {
        self.blocks
    }

    fn serialize_graph(&self, graph: &Graph) -> String {
        let mut graph = graph.clone();
        graph.trim();
        graph.to_string_hmetis().trim_end().into()
    }

    fn create_run(&self, graph: &Path) -> Command {
        let mut run = Command::new(&self.path);

        run.args([
            "--hypergraph",
            graph.to_str().unwrap(),
            "--blocks",
            &self.blocks.to_string(),
            "--mode",
            "direct",
            "--objective",
            "cut",
            "--epsilon",
            &self.epsilon.clone(),
            "--preset",
            self.preset_file.to_str().unwrap(),
            "--write-partition",
            "1",
        ]);

        run
    }

    fn output_file(&self, input: &Path) -> PathBuf {
        let filename = input
            .file_name()
            .expect("Failed to resolve input file name.")
            .to_str()
            .expect("Failed to convert input file name.");

        input
            .parent()
            .expect("Failed to resolve directory.")
            .join(format!(
                "{filename}.part{}.epsilon{}.seed-1.KaHyPar",
                self.blocks, self.epsilon
            ))
    }
}

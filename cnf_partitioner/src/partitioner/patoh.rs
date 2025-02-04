use super::Partitioner;
use clap::ValueEnum;
use hypergraph_formats::patoh::ToStringPATOH;
use hypergraph_formats::Graph;
use std::path::{Path, PathBuf};
use std::process::Command;
use strum::Display;

pub struct PaToH {
    path: PathBuf,
    blocks: usize,
    metric: Metric,
    preset: Preset,
}

#[derive(Debug, Copy, Clone, Display, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Metric {
    Cut,
    Connectivity,
}

impl Metric {
    pub fn as_arg(&self) -> &str {
        match self {
            Metric::Cut => "U",
            Metric::Connectivity => "O",
        }
    }
}

#[derive(Debug, Copy, Clone, Display, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Preset {
    Quality,
    Default,
    Speed,
}

impl Preset {
    pub fn as_arg(&self) -> &str {
        match self {
            Preset::Quality => "Q",
            Preset::Default => "D",
            Preset::Speed => "S",
        }
    }
}

impl PaToH {
    pub fn new(path: PathBuf, blocks: usize, metric: Metric, preset: Preset) -> Self {
        Self {
            path,
            blocks,
            metric,
            preset,
        }
    }
}

impl Partitioner for PaToH {
    fn name_full(&self) -> String {
        format!(
            "PaToH (preset: {:?}, metric: {:?})",
            self.preset, self.metric
        )
    }

    fn name_short(&self) -> &'static str {
        "PaToH"
    }

    fn blocks(&self) -> usize {
        self.blocks
    }

    fn serialize_graph(&self, graph: &Graph) -> String {
        graph.to_string_patoh()
    }

    fn create_run(&self, graph: &Path) -> Command {
        let mut run = Command::new(&self.path);

        run.args([
            graph.to_str().unwrap(),
            &self.blocks.to_string(),
            format!("UM={}", &self.metric.as_arg()).as_str(),
            format!("PQ={}", &self.preset.as_arg()).as_str(),
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
            .join(format!("{filename}.part.{}", self.blocks))
    }
}

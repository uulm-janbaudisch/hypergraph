use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

/// The result of a partitioner run mapping each vertex to a block.
#[derive(Debug, Clone)]
pub struct Partition(pub Vec<usize>);

impl From<PathBuf> for Partition {
    fn from(value: PathBuf) -> Self {
        Partition(
            fs::read_to_string(value)
                .expect("Failed to read partition file.")
                .lines()
                .map(usize::from_str)
                .map(Result::unwrap)
                .collect(),
        )
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|block| writeln!(f, "{}", block))
    }
}

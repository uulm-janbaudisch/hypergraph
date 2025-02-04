use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Index, IndexMut};
use std::path::PathBuf;
use std::str::FromStr;

/// The result of a partitioner, mapping of vertices to a block.
///
/// Entry `i` indicates the block that vertex `i` is in.
pub struct Partition(Vec<usize>);

impl Partition {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }
}

impl Default for Partition {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<usize>> for Partition {
    fn from(value: Vec<usize>) -> Self {
        Self(value)
    }
}

impl From<&str> for Partition {
    fn from(value: &str) -> Self {
        value
            .lines()
            .map(|block| usize::from_str(block).expect("Failed to parse block number."))
            .collect::<Vec<usize>>()
            .into()
    }
}

impl From<PathBuf> for Partition {
    fn from(value: PathBuf) -> Self {
        Self::from(
            fs::read_to_string(value)
                .expect("Failed to read partition file.")
                .as_str(),
        )
    }
}

impl Index<usize> for Partition {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl IndexMut<usize> for Partition {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter().try_for_each(|block| writeln!(f, "{block}"))
    }
}

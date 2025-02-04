use clap::ValueEnum;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Mode {
    Bfs,
    Dfs,
    Random,
}

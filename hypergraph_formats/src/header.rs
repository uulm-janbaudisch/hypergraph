use crate::Format;

/// The header of a hypergraph file describing the graph following it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// The number of nets in the hypergraph.
    pub num_nets: usize,
    /// The total number of vertices in the hypergraph.
    pub num_vertices: usize,
    /// The type of hypergraph in regard to it having net and/or vertex weights.
    pub format: Format,
}

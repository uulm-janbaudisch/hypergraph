/// The type of hypergraph in regard to it having net and/or vertex weights.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Neither the vertices nor the nets are weighted.
    Unweighted,
    /// Nets have weights, but vertecies do not.
    NetWeights,
    /// Vertices have weights, but nets do not.
    VertexWeights,
    /// Vertices and nets are weighted.
    Weighted,
}

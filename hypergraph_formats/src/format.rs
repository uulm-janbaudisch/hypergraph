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

impl Format {
    /// Inverts the format for creating a dual hypergraph.
    pub fn dual(&self) -> Self {
        match self {
            Self::Unweighted => Self::Unweighted,
            Self::NetWeights => Self::VertexWeights,
            Self::VertexWeights => Self::NetWeights,
            Self::Weighted => Self::Weighted,
        }
    }

    pub fn contains_vertex_weights(&self) -> bool {
        self == &Self::VertexWeights || self == &Self::Weighted
    }

    pub fn contains_net_weights(&self) -> bool {
        self == &Self::NetWeights || self == &Self::Weighted
    }
}

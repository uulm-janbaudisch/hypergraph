use crate::Header;
use std::collections::{BTreeMap, BTreeSet};
use std::iter;

/// The identification of a single vertex.
pub type Vertex = usize;

/// A collection of vertices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Net(pub Vec<Vertex>);

impl Net {
    /// Returns an iterator over the vertices of this net.
    pub fn iter(&self) -> impl Iterator<Item = &Vertex> {
        self.0.iter()
    }

    /// Checks whether this net contains no vertices.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A hypergraph structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub header: Header,
    pub nets: Vec<Net>,
}

impl Graph {
    /// Creates the corresponding dual hypergraph.
    ///
    /// Vertices become nets containing their pins.
    pub fn dual(&self) -> Self {
        let mut pins: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        self.nets.iter().enumerate().for_each(|(net_index, net)| {
            net.iter().for_each(|vertex| {
                if pins
                    .get_mut(vertex)
                    .map(|vertex_pins| vertex_pins.insert(net_index))
                    .is_none()
                {
                    pins.insert(*vertex, BTreeSet::new());
                    pins.get_mut(vertex)
                        .map(|vertex_pins| vertex_pins.insert(net_index));
                }
            })
        });

        // Each vertex of the primal graph becomes a net containing all the original vertex pins.
        let nets = pins
            .values()
            .map(BTreeSet::iter)
            .map(Iterator::copied)
            .map(Iterator::collect)
            .map(Net)
            .collect();

        Self {
            header: Header {
                num_nets: self.header.num_vertices,
                num_vertices: self.header.num_nets,
                // TODO: actually invert the format if necessary
                format: self.header.format,
            },
            nets,
        }
    }

    /// Iterates over all pins (vertex <-> net pair) of the hypergraph.
    pub fn pins(&self) -> impl Iterator<Item = (usize, Vertex)> + '_ {
        // Consider each net ...
        self.nets
            .iter()
            .enumerate()
            // ... creating an iterator per net for all vertices in it with the corresponding index.
            .flat_map(|(net_index, net)| iter::repeat(net_index).zip(net.iter().copied()))
    }

    /// Calculates the number of pins in this hypergraph.
    pub fn pin_count(&self) -> usize {
        self.nets.iter().flat_map(Net::iter).count()
    }

    /// Iterates over each vertex in the hypergraph exactly once.
    pub fn vertices(&self) -> impl Iterator<Item = &Vertex> {
        // Set tracking which vertices were already seen.
        let mut vertices = BTreeSet::new();

        // Iterate over each net ...
        self.nets
            .iter()
            // ... and each vertex within ...
            .flat_map(Net::iter)
            // ... only taking those that were not seen before.
            .filter(move |&&vertex| vertices.insert(vertex))
    }

    /// Move all vertex indices of the hypergraph by the given amount.
    pub fn move_indices(&mut self, step: isize) {
        self.nets.iter_mut().for_each(|net| {
            net.0.iter_mut().for_each(|vertex| {
                *vertex = vertex.saturating_add_signed(step);
            })
        })
    }
}

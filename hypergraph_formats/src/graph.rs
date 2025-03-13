use crate::Header;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::Display;
use std::iter;

pub type Weight = u16;
pub type VertexIndex = usize;
pub type NetIndex = usize;
pub type Pin = (VertexIndex, NetIndex);

/// A collection of vertices.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Net(pub Vec<VertexIndex>);

impl Net {
    /// Returns an iterator over the vertices of this net.
    pub fn iter(&self) -> impl Iterator<Item = &VertexIndex> {
        self.0.iter()
    }

    /// Checks whether this net contains no vertices.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Net {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let last = self.0.len() - 1;
        self.0.iter().enumerate().try_for_each(|(i, vertex)| {
            write!(f, "{}", vertex)?;

            // All but the last vertex should have a space appended as a separator.
            if i != last {
                write!(f, " ")?;
            }

            Ok(())
        })
    }
}

/// A hypergraph structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    /// Information on the type of hypergraph.
    pub header: Header,
    /// The nets of the hypergraph.
    pub nets: Vec<Net>,
    /// The weight for vertices.
    pub vertex_weights: Vec<Weight>,
    /// The weight for nets.
    pub net_weights: Vec<Weight>,
}

impl Graph {
    /// Removes empty nets and their weights.
    pub fn trim(&mut self) {
        let empty_nets: BTreeSet<usize> = self
            .nets
            .iter()
            .enumerate()
            .filter_map(|(index, net)| if net.is_empty() { Some(index) } else { None })
            .collect();

        self.nets = self
            .nets
            .iter()
            .enumerate()
            .filter_map(|(index, net)| {
                if empty_nets.contains(&index) {
                    None
                } else {
                    Some(net)
                }
            })
            .cloned()
            .collect();

        if !self.header.format.contains_net_weights() {
            return;
        }

        self.net_weights = self
            .net_weights
            .iter()
            .enumerate()
            .filter_map(|(index, weight)| {
                if empty_nets.contains(&index) {
                    None
                } else {
                    Some(weight)
                }
            })
            .copied()
            .collect();
    }

    /// Creates the corresponding dual hypergraph.
    ///
    /// Vertices become nets containing their pins.
    pub fn dual(&self) -> Self {
        let mut nets = vec![Net::default(); self.header.num_vertices];

        // Consider each pin.
        self.pins().for_each(|(vertex, net)| {
            // `1`-indexed graphs need their vertex and net indices adapted accordingly.
            let (vertex, net) = if self.header.one_indexed {
                (vertex - 1, net + 1)
            } else {
                (vertex, net)
            };

            // Add the net this pin is referencing as a vertex to the net representing the original vertex.
            nets.get_mut(vertex).unwrap().0.push(net);
        });

        Self {
            header: Header {
                num_nets: self.header.num_vertices,
                num_vertices: self.header.num_nets,
                format: self.header.format.dual(),
                one_indexed: self.header.one_indexed,
            },
            nets,
            vertex_weights: self.net_weights.clone(),
            net_weights: self.vertex_weights.clone(),
        }
    }

    /// Iterates over all pins (vertex <-> net pair) of the hypergraph.
    pub fn pins(&self) -> impl Iterator<Item = Pin> + '_ {
        // Consider each net ...
        self.nets
            .iter()
            .enumerate()
            // ... creating an iterator per net for all vertices in it with the corresponding index.
            .flat_map(|(net_index, net)| {
                iter::repeat(net_index)
                    .zip(net.iter())
                    .map(|(net_index, &vertex)| (vertex, net_index))
            })
    }

    /// Calculates the number of pins in this hypergraph.
    pub fn pin_count(&self) -> usize {
        self.nets.iter().flat_map(Net::iter).count()
    }

    /// Iterates over each vertex in the hypergraph exactly once.
    pub fn vertices(&self) -> impl Iterator<Item = &VertexIndex> {
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
    ///
    /// Also increases the number of vertices and shifts the vertex weights according if present.
    pub fn move_indices(&mut self, step: isize) {
        self.nets.iter_mut().for_each(|net| {
            net.0.iter_mut().for_each(|vertex| {
                *vertex = vertex.saturating_add_signed(step);
            })
        });

        self.header.num_vertices += step as usize;

        // Insert vertex weights (0) for the vertices created.
        if self.header.format.contains_vertex_weights() {
            let mut queue = VecDeque::from(self.vertex_weights.clone());
            (0..step).for_each(|_| queue.push_front(0));
            self.vertex_weights = Vec::from(queue);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Graph;
    use crate::{Format, Header, Net};

    fn graph() -> Graph {
        Graph {
            header: Header {
                num_nets: 3,
                num_vertices: 4,
                format: Format::Unweighted,
                one_indexed: false,
            },
            nets: vec![Net(vec![0, 1]), Net(vec![1, 2, 3]), Net(vec![0, 3])],
            vertex_weights: vec![],
            net_weights: vec![],
        }
    }

    #[test]
    fn dual_unweighted() {
        let primal = graph();

        let dual = Graph {
            header: Header {
                num_nets: 4,
                num_vertices: 3,
                format: Format::Unweighted,
                one_indexed: false,
            },
            nets: vec![
                Net(vec![0, 2]),
                Net(vec![0, 1]),
                Net(vec![1]),
                Net(vec![1, 2]),
            ],
            vertex_weights: vec![],
            net_weights: vec![],
        };

        assert_eq!(primal.dual(), dual);
        assert_eq!(primal.dual().dual(), primal);
    }

    #[test]
    fn move_indices_unweighted() {
        let mut moved = graph();
        moved.move_indices(1);

        let mut expected = graph();
        expected.header.num_vertices = 5;
        expected.nets = vec![Net(vec![1, 2]), Net(vec![2, 3, 4]), Net(vec![1, 4])];

        assert_eq!(moved, expected);
    }
}

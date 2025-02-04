mod search;

use search::SearchIterator;
use std::collections::{BTreeMap, VecDeque};
use std::iter::Sum;

#[cfg(feature = "formats")]
use hypergraph_formats::Graph;

pub trait Weight: Copy + Sum {}

impl<W: Copy + Sum> Weight for W {}

#[derive(Clone, Debug)]
pub struct Hypergraph<W: Weight> {
    pub(crate) vertices: BTreeMap<usize, Vertex<W>>,
    pub(crate) nets: Vec<Vec<usize>>,
}

#[derive(Clone, Debug)]
pub struct Vertex<W: Weight> {
    pub(crate) nets: Vec<usize>,
    pub(crate) weight: W,
}

impl<W: Weight> Vertex<W> {
    pub fn new(weight: W) -> Vertex<W> {
        Self {
            nets: Vec::new(),
            weight,
        }
    }
}

impl<W: Weight> Hypergraph<W> {
    // /// Creates a new hypergraph with the given vertex weights and amount of nets.
    // ///
    // /// It is not possible to add more nets later.
    /*pub fn new(vertex_weights: &[W], num_nets: usize) -> Self {
        Self {
            vertices: vertex_weights
                .iter()
                .map(|&weight| Vertex::new(weight))
                .collect(),
            nets: vec![Vec::new(); num_nets],
        }
    }*/

    pub fn new(num_nets: usize) -> Self {
        Self {
            vertices: BTreeMap::new(),
            nets: vec![Vec::new(); num_nets],
        }
    }

    // /// Adds a pin placing a vertex in a net.
    /*pub fn add_pin(&mut self, net: usize, vertex: usize) {
        self.vertices[vertex].nets.push(net);
        self.nets[net].push(vertex);
    }*/

    /// The amount of vertices in the hypergraph.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Determines whether the hypergraph contains vertices.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Creates an iterator over all vertices adjacent to the specified one.
    pub fn neighbors(&self, vertex: usize) -> impl Iterator<Item = &usize> {
        self.vertices
            .get(&vertex)
            .expect("Vertex not found.")
            .nets
            .iter()
            .flat_map(|&net| self.nets[net].iter())
    }

    /// The sum of all vertex weights.
    pub fn weight(&self) -> W {
        self.vertices.values().map(|vertex| vertex.weight).sum()
    }

    /// Iterates over the vertices via [breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search).
    pub fn bfs(&self) -> SearchIterator<W, VecDeque<usize>> {
        SearchIterator::new(self)
    }

    /// Iterates over the vertices via [depth-first search](https://en.wikipedia.org/wiki/Depth-first_search).
    pub fn dfs(&self) -> SearchIterator<W, Vec<usize>> {
        SearchIterator::new(self)
    }
}

impl Hypergraph<usize> {
    pub fn add_pin(&mut self, net: usize, vertex: usize) {
        self.vertices
            .entry(vertex)
            .or_insert_with(|| Vertex::new(1))
            .nets
            .push(net);

        self.nets[net].push(vertex);
    }
}

#[cfg(feature = "formats")]
impl From<&Graph> for Hypergraph<usize> {
    fn from(value: &Graph) -> Self {
        let mut graph = Self::new(value.nets.len());

        value
            .pins()
            .for_each(|(net_index, vertex)| graph.add_pin(net_index, vertex));

        graph
    }
}

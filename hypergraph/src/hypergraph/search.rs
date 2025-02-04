mod search_collection;

use crate::hypergraph::Weight;
use crate::Hypergraph;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use search_collection::SearchCollection;

/// Structure for doing a hypergraph search.
pub struct SearchIterator<'a, W: Weight, C: SearchCollection<usize>> {
    /// The graph this search is operating on.
    graph: &'a Hypergraph<W>,
    /// Markers for the vertices already put into the stack.
    seen: BitVec,
    /// The amount of vertices already visited.
    visited: usize,
    /// The vertices left to visit.
    collection: C,
    /// The last considered root when searching for disconnected components.
    last_root: usize,
}

impl<'a, W: Weight, C: SearchCollection<usize>> SearchIterator<'a, W, C> {
    /// Creates a new search iterator.
    pub fn new(graph: &'a Hypergraph<W>) -> Self {
        let mut collection = C::new();
        let mut seen = bitvec![0; graph.len()];

        // Start with the first vertex in case the graph is not empty.
        if !graph.is_empty() {
            collection.extend([0]);
            seen.set(0, true);
        }

        Self {
            graph,
            seen,
            visited: 0,
            collection,
            last_root: 0,
        }
    }

    /// Finds the root of the next unvisited disconnected component of the hypergraph.
    fn find_unvisited(&mut self) -> usize {
        // Consider all vertices from the last root on.
        let next = (self.last_root..self.graph.len())
            // Take the first unvisited vertex.
            .find(|&vertex| !self.seen[vertex])
            .expect("Failed to find unvisited vertex.");

        // Save it for the next run.
        self.last_root = next;

        next
    }
}

impl<W: Weight, C: SearchCollection<usize>> Iterator for SearchIterator<'_, W, C> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // End of iteration reached when all vertices were visited.
        if self.visited == self.graph.len() {
            return None;
        }

        // Special case for disconnected graphs:
        // Unvisited parts of the graph should be considered next.
        if self.collection.is_empty() {
            let next = self.find_unvisited();
            self.collection.put(next);
            self.seen.set(next, true);
        }

        // Take the next vertex.
        let current = self.collection.get().unwrap();
        self.visited += 1;

        // Get the unvisited neighbors of the current node ...
        let neighbors = self
            .graph
            .neighbors(current)
            .filter(|&&neighbor| {
                // Mark each neighbor as seen.
                let visited = self.seen[neighbor];

                if !visited {
                    self.seen.set(neighbor, true);
                }

                !visited
            })
            .copied();

        // ... and insert them to be considered next.
        self.collection.extend(neighbors);

        Some(current)
    }
}

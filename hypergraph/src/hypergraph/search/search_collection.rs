use std::collections::VecDeque;

/// An abstraction over different collections for search algorithms such as stacks or queues.
pub trait SearchCollection<I>: Extend<I> {
    /// Creates a new collection.
    fn new() -> Self;

    /// Creates a new collection with a pre-allocated capacity.
    fn with_capacity(capacity: usize) -> Self;

    /// Checks whether this collection is empty.
    fn is_empty(&self) -> bool;

    /// Returns the next element.
    fn get(&mut self) -> Option<I>;

    /// Puts an element into the collection.
    fn put(&mut self, value: I);
}

impl<I> SearchCollection<I> for Vec<I> {
    fn new() -> Self {
        Vec::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn get(&mut self) -> Option<I> {
        self.pop()
    }

    fn put(&mut self, value: I) {
        self.push(value);
    }
}

impl<I> SearchCollection<I> for VecDeque<I> {
    fn new() -> Self {
        VecDeque::new()
    }

    fn with_capacity(capacity: usize) -> Self {
        VecDeque::with_capacity(capacity)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn get(&mut self) -> Option<I> {
        self.pop_front()
    }

    fn put(&mut self, value: I) {
        self.push_back(value);
    }
}

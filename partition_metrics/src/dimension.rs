use crate::utils::{extract_clauses, extract_literals, extract_variables};
use dimacs::Instance;

pub fn num_clauses(original: &Instance, split: &[Instance]) -> (usize, Vec<usize>) {
    (
        extract_clauses(original).len(),
        split
            .iter()
            .map(extract_clauses)
            .map(|clauses| clauses.len())
            .collect(),
    )
}

pub fn num_variables(original: &Instance, split: &[Instance]) -> (usize, Vec<usize>) {
    (
        extract_variables(original).count(),
        split
            .iter()
            .map(extract_variables)
            .map(Iterator::count)
            .collect(),
    )
}

pub fn num_literals(original: &Instance, split: &[Instance]) -> (usize, Vec<usize>) {
    (
        extract_literals(original).count(),
        split
            .iter()
            .map(extract_literals)
            .map(Iterator::count)
            .collect(),
    )
}

use crate::dimension::{num_clauses, num_variables};
use crate::utils::extract_clauses;
use dimacs::{Clause, Instance};

pub fn clause_width(original: &Instance, split: &[Instance]) -> (usize, Vec<usize>) {
    (
        width(extract_clauses(original)),
        split.iter().map(extract_clauses).map(width).collect(),
    )
}

pub fn clause_density(original: &Instance, split: &[Instance]) -> (usize, Vec<usize>) {
    let (clauses_original, clauses_split) = num_clauses(original, split);
    let (variables_original, variables_split) = num_variables(original, split);

    assert_eq!(clauses_split.len(), variables_split.len());

    (
        clauses_original / variables_original,
        clauses_split
            .iter()
            .zip(variables_split)
            .map(|(clauses, variables)| clauses / variables)
            .collect(),
    )
}

fn width(clauses: &[Clause]) -> usize {
    clauses
        .iter()
        .map(Clause::len)
        .max()
        .expect("There should be at least one clause.")
}

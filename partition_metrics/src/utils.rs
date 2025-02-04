use dimacs::{Clause, Instance, Lit, Sign, Var};
use std::collections::BTreeSet;

pub fn extract_clauses(instance: &Instance) -> &[Clause] {
    match instance {
        Instance::Cnf { clauses, .. } => clauses,
        Instance::Sat { .. } => panic!("Expected CNF instance, found SAT."),
    }
}

pub fn extract_variables(instance: &Instance) -> impl Iterator<Item = u64> {
    let variables: BTreeSet<u64> = extract_clauses(instance)
        .iter()
        .flat_map(|clause| clause.lits())
        .copied()
        .map(Lit::var)
        .map(Var::to_u64)
        .collect();

    variables.into_iter()
}

pub fn extract_literals(instance: &Instance) -> impl Iterator<Item = i64> {
    let literals: BTreeSet<i64> = extract_clauses(instance)
        .iter()
        .flat_map(|clause| clause.lits())
        .copied()
        .map(|literal| match literal.sign() {
            Sign::Pos => literal.var().to_u64() as i64,
            Sign::Neg => -(literal.var().to_u64() as i64),
        })
        .collect();

    literals.into_iter()
}

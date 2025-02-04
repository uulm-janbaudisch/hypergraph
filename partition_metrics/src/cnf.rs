use dimacs::{Clause, Instance, Lit, Sign, Var};
use hypergraph::Partition;
use std::collections::{BTreeSet, HashMap};

pub fn split_cnf(partition: &Partition, original: &Instance) -> Vec<Instance> {
    let (clauses, &num_vars) = match original {
        Instance::Cnf { clauses, num_vars } => (clauses, num_vars),
        Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
    };

    // Helper instance to collect all clauses for their respective CNF.
    let mut instances: HashMap<usize, Vec<Clause>> = HashMap::new();

    // Iterate over all clause -> block pairs.
    partition
        .iter()
        .enumerate()
        // Get the clauses content.
        .map(|(clause_index, block)| (clauses[clause_index].clone(), block))
        // Add the clause to the correct block (CNF).
        .for_each(|(clause, block)| match instances.get_mut(block) {
            Some(instance) => instance.push(clause),
            None => {
                // Create a new set of clauses in case there is none.
                instances.insert(*block, vec![clause]);
            }
        });

    instances
        .into_values()
        .map(|clauses| Instance::cnf(num_vars, clauses))
        .collect()
}

/// Calculates the cut between multiple CNF instances.
///
/// The cut consists of those variables that are shared between instances.
pub fn get_cut_variables(instances: &[Instance]) -> BTreeSet<u64> {
    let mut cut = BTreeSet::new();

    // Collect the variables beforehand.
    let instance_variables: Vec<BTreeSet<u64>> =
        instances.iter().map(get_instance_variables).collect();

    // For each instance ...
    instance_variables
        .iter()
        .enumerate()
        .for_each(|(index, a)| {
            // ... compare it with each other instance not considered yet ...
            instance_variables.iter().skip(index + 1).for_each(|b| {
                // ... and add the cut of these.
                cut.extend(a.intersection(b));
            });
        });

    cut
}

/// Creates the set of variables in a CNF instance.
fn get_instance_variables(instance: &Instance) -> BTreeSet<u64> {
    match instance {
        Instance::Cnf { clauses, .. } => clauses
            .iter()
            .flat_map(Clause::lits)
            .copied()
            .map(Lit::var)
            .map(Var::to_u64)
            .collect(),

        Instance::Sat { .. } => panic!("Expected SAT but found CNF DIMACS."),
    }
}

/// Serializes a CNF into the DIMACS format.
pub fn serialize_cnf(cnf: &Instance) -> String {
    let (clauses, &num_vars) = match cnf {
        Instance::Cnf { clauses, num_vars } => (clauses, num_vars),
        Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
    };

    let mut buffer = String::new();

    // Start with the header.
    let num_clauses = clauses.len();
    buffer.push_str(format!("p cnf {num_vars} {num_clauses}\n").as_str());

    // For each clause and its literals ...
    clauses.iter().map(Clause::lits).for_each(|literals| {
        // ... add them to the output buffer.
        literals.iter().for_each(|literal| {
            buffer.push_str(format!("{} ", serialize_literal(literal)).as_str())
        });

        // Each clause end with a `0`.
        buffer.push_str("0\n");
    });

    buffer
}

/// Serializes a CNF literal.
fn serialize_literal(literal: &Lit) -> String {
    let mut buffer = String::new();

    // Add the sign if necessary.
    match literal.sign() {
        Sign::Pos => {}
        Sign::Neg => buffer.push('-'),
    }

    // Add the actual value.
    buffer.push_str(literal.var().to_u64().to_string().as_str());

    buffer
}

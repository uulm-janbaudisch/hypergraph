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

/// Searches for an assignment of the given cut set satisfying the CNF instance.
pub fn find_assignment(instance: &Instance, cut_set: &BTreeSet<u64>) -> Vec<i32> {
    // Initialize the SAT solver.
    let mut solver: cadical::Solver = Default::default();

    // Transform the clauses into the required format for the solver.
    match instance {
        Instance::Cnf { clauses, .. } => {
            clauses.iter().for_each(|clause| {
                solver.add_clause(clause.lits().iter().map(|literal| {
                    let variable = literal.var().to_u64() as i32;
                    match literal.sign() {
                        Sign::Pos => variable,
                        Sign::Neg => -variable,
                    }
                }))
            });
        }
        Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
    }

    // Solve the formula.
    assert!(solver.solve().expect("Failed to solve CNF."));

    // For each variable in the cut set ...
    cut_set
        .iter()
        .map(|&variable| variable as i32)
        // ... find the value in the solved instance.
        .map(|variable| match solver.value(variable) {
            // Return it as a positive literal in case it was not decided or done so positively ...
            None | Some(true) => variable,
            // ... and negative otherwise.
            Some(false) => -variable,
        })
        .collect()
}

/// Conditions a CNF instance on the provided assignment by adding each literal as a unit clause.
/// Additionally adds each variable that is neither part of the assignment nor occurring in the formula.
pub fn condition_instance(instance: &Instance, assignment: &[i32]) -> Instance {
    let (mut clauses, &num_vars) = match instance {
        Instance::Cnf { clauses, num_vars } => (clauses.to_vec(), num_vars),
        Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
    };

    // Add each assigned literal to the formula as a unit clause.
    clauses.extend(
        assignment
            .iter()
            .map(|&literal| Clause::from_vec(vec![Lit::from_i64(literal as i64)])),
    );

    // Generate a CNF instance to be further extended.
    let instance = Instance::cnf(num_vars, clauses.clone());

    // Get all variables in the CNF, including the ones just added.
    let variables = get_instance_variables(&instance);

    // Add each variable that is not part of the assignment and the formula.
    clauses.extend(
        (1..=num_vars)
            .filter(|variable| !variables.contains(variable))
            .map(|variable| Clause::from_vec(vec![Lit::from_i64(variable as i64)])),
    );

    Instance::cnf(num_vars, clauses)
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
pub fn serialize_literal(literal: &Lit) -> String {
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

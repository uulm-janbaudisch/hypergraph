use crate::{Format, Graph, Header, Net};
use dimacs::{Instance, Lit, Var};

impl From<&Instance> for Graph {
    /// Converts a CNF into an unweighted hypergraph.
    ///
    /// Only works for a CNF instance, will panic otherwise.
    fn from(value: &Instance) -> Self {
        let (clauses, num_vars) = match value {
            Instance::Cnf { clauses, num_vars } => (clauses, num_vars),
            Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
        };
        // Transform the CNF into a hypergraph ...
        let nets = clauses
            .iter()
            // by transforming every clause into a net.
            .map(|clause| {
                clause
                    .lits()
                    .iter()
                    .cloned()
                    .map(Lit::var)
                    .map(Var::to_u64)
                    .map(|vertex| vertex as usize)
                    .collect()
            })
            .map(Net)
            .collect();

        Graph {
            header: Header {
                num_nets: clauses.len(),
                num_vertices: *num_vars as usize,
                format: Format::Unweighted,
            },
            nets,
        }
    }
}

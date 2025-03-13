use crate::{Format, Graph, Header, Net, Weight};
use clap::ValueEnum;
use dimacs::{Clause, Instance, Lit, Sign, Var};
use std::cmp::PartialEq;
use strum::Display;

/// A single clause of a CNF.
struct CnfClause(Vec<i64>);

impl CnfClause {
    /// Iterates of all literals of this clause.
    pub fn literals(&self) -> impl Iterator<Item = &i64> {
        self.0.iter()
    }

    /// Iterates over all variables of this clause.
    pub fn variables(&self) -> impl Iterator<Item = u64> + '_ {
        self.literals().map(|literal| literal.unsigned_abs())
    }

    /// Returns the amount of literals in this clause.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks whether this clause contains the positive literal of the given variable.
    pub fn contains_positive(&self, var: u64) -> bool {
        self.0.contains(&(var as i64))
    }

    /// Checks whether this clause contains the negative literal of the given variable.
    pub fn contains_negative(&self, var: u64) -> bool {
        self.0.contains(&-(var as i64))
    }

    /// Checks whether this clause contains the given variable.
    pub fn contains(&self, var: u64) -> bool {
        self.contains_positive(var) || self.contains_negative(var)
    }

    /// Calculates how many times the given variable is found in this clause. Can be `0`, `1` or `2`.
    pub fn occurrence(&self, var: u64) -> usize {
        let positive = self.contains_positive(var);
        let negative = self.contains_negative(var);

        if positive && negative {
            return 2;
        }

        if positive || negative {
            return 1;
        }

        0
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, ValueEnum, Display)]
pub enum VariableHeuristic {
    #[default]
    None,
    MAXO,
    MOMS,
    MAMS,
}

/// An alternative CNF representation for calculating heuristics.
struct Cnf(Vec<CnfClause>);

impl Cnf {
    /// Iterates over all clauses of this CNF.
    pub fn iter(&self) -> impl Iterator<Item = &CnfClause> {
        self.0.iter()
    }

    /// Iterates over all clauses of minimum size.
    fn minimum_size_clauses(&self) -> impl Iterator<Item = &CnfClause> {
        let minimum_size = self.0.iter().map(CnfClause::len).min().unwrap();
        self.iter()
            .filter(move |clause| clause.len() == minimum_size)
    }

    pub fn maxo(&self, var: u64) -> usize {
        self.iter().map(|clause| clause.occurrence(var)).sum()
    }

    pub fn moms(&self, var: u64) -> usize {
        self.minimum_size_clauses()
            .map(|clause| clause.occurrence(var))
            .sum()
    }

    pub fn mams(&self, var: u64) -> usize {
        self.maxo(var) + self.moms(var)
    }

    pub fn jeroslaw_wang(&self, var: u64) -> f64 {
        self.iter()
            .filter(|clause| clause.contains(var))
            .map(|clause| 2f64.powi(-(clause.len() as i32)))
            .sum()
    }
}

impl From<&Clause> for CnfClause {
    fn from(value: &Clause) -> Self {
        CnfClause(
            value
                .lits()
                .iter()
                .map(|lit| match lit.sign() {
                    Sign::Pos => lit.var().to_u64() as i64,
                    Sign::Neg => -(lit.var().to_u64() as i64),
                })
                .collect(),
        )
    }
}

impl From<&Instance> for Cnf {
    fn from(value: &Instance) -> Self {
        match value {
            Instance::Cnf { clauses, .. } => Cnf(clauses.iter().map(CnfClause::from).collect()),
            Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
        }
    }
}

impl From<(&Instance, VariableHeuristic)> for Graph {
    fn from((instance, heuristic): (&Instance, VariableHeuristic)) -> Self {
        let cnf = Cnf::from(instance);

        // Transform the CNF into a hypergraph ...
        let nets: Vec<Net> = cnf
            .iter()
            // by transforming every clause into a net.
            .map(|clause| {
                clause
                    .variables()
                    .map(|variable| variable as usize)
                    .collect()
            })
            .map(Net)
            .collect();

        let num_nets = nets.len();
        let num_vertices = match instance {
            Instance::Cnf { num_vars, .. } => *num_vars as usize,
            Instance::Sat { .. } => panic!("Expected CNF but found SAT DIMACS."),
        };

        if heuristic == VariableHeuristic::None {
            return Graph {
                header: Header {
                    num_nets,
                    num_vertices,
                    format: Format::Unweighted,
                    one_indexed: true,
                },
                vertex_weights: Vec::new(),
                net_weights: Vec::new(),
                nets,
            };
        }

        // Add the vertex weights as calculated by the chosen heuristic.
        let mut weights = Vec::with_capacity(num_vertices);

        // CNF variables are `1`-index which has to be taken into account for calculating the weights.
        (1..=num_vertices).for_each(|vertex| {
            // Calculate the weight using the chosen heuristic.
            let weight = match heuristic {
                VariableHeuristic::None => unreachable!(),
                VariableHeuristic::MAXO => cnf.maxo(vertex as u64),
                VariableHeuristic::MOMS => cnf.moms(vertex as u64),
                VariableHeuristic::MAMS => cnf.mams(vertex as u64),
            };

            weights.push(weight);
        });

        // Find the maximum weight.
        let max_weight = weights
            .iter()
            .max()
            .expect("There should be at least one vertex weight.");

        // Scale all weights relative to the maximum one.
        let vertex_weights = weights
            .iter()
            // Calculate the ratio of any given weight to the maximum one.
            /*            .map(|&weight| weight as f64 / *max_weight as f64)
            // Scale the weight using this ratio relative to the maximum possible weight.
            .map(|ratio| {
                let weight = (Weight::MAX as f64 * ratio) as Weight;
                weight
            })*/
            .map(|&weight| {
                let weight_adapted = Weight::MAX - weight as Weight;
                weight_adapted
            })
            .map(|weight| if weight == 0 { 1 } else { weight })
            .collect();

        Graph {
            header: Header {
                num_nets,
                num_vertices,
                format: Format::VertexWeights,
                one_indexed: true,
            },
            vertex_weights,
            net_weights: Vec::new(),
            nets,
        }
    }
}

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

        let num_nets = clauses.len();
        let num_vertices = *num_vars as usize;

        Graph {
            header: Header {
                num_nets,
                num_vertices,
                format: Format::Unweighted,
                one_indexed: true,
            },
            vertex_weights: Vec::new(),
            net_weights: Vec::new(),
            nets,
        }
    }
}

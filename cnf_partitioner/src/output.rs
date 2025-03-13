use hypergraph_formats::cnf::VariableHeuristic;
use num::BigInt;

/// A collection of run results being this programs output.
#[derive(Debug)]
pub struct Output(Vec<Run>);

impl Output {
    /// Creates an empty output.
    pub fn new() -> Output {
        Self(Vec::new())
    }

    /// Adds a run.
    pub fn add(&mut self, run: Run) {
        self.0.push(run);
    }

    /// Serializes the output as CSV.
    pub fn csv(&self) -> String {
        let mut output = String::new();

        self.0.iter().for_each(|run| {
            output.push('\n');
            run.csv(&mut output)
        });

        output
    }
}

/// A run of a partitioner
#[derive(Default, Debug)]
pub struct Run {
    instance: String,
    partitioner: String,
    variable_heuristic: VariableHeuristic,
    blocks: usize,
    cut_size: usize,
    time_partitioning: u128,
    time_solving: Vec<u128>,
    time_original: u128,
    time_conditioned: u128,
    count: BigInt,
    count_conditioned: BigInt,
    count_original: BigInt,
}

impl Run {
    /// Creates a new run.
    pub fn new(
        instance: String,
        partitioner: &str,
        variable_heuristic: VariableHeuristic,
        blocks: usize,
        cut_size: usize,
        time_original: u128,
        count_original: BigInt,
        time_conditioned: u128,
        count_conditioned: BigInt,
        time_partitioning: u128,
    ) -> Self {
        Self {
            instance,
            partitioner: String::from(partitioner),
            variable_heuristic,
            blocks,
            cut_size,
            time_original,
            count_original,
            time_conditioned,
            count_conditioned,
            time_partitioning,
            count: BigInt::from(1),
            ..Default::default()
        }
    }

    /// Adds the result of solving a part of the split CNF.
    pub fn add_part(&mut self, time: u128, count: BigInt) {
        self.time_solving.push(time);
        self.count *= count;
    }

    /// Checks the result after all partial runs for correctness.
    pub fn check(&self) {
        assert_eq!(
            self.count_conditioned, self.count,
            "The counts between the conditioned and split CNFs (collectively) should be equal."
        );
    }

    /// The output CSV header describing the contents of runs.
    pub const fn csv_header() -> &'static str {
        "instance,partitioner,heuristic,blocks,cut_size,time_original,time_conditioned,count_original,count_conditioned,time_split,time_sum,time_partitioning"
    }

    /// Serializes a run into a CSV row.
    pub fn csv(&self, output: &mut String) {
        output.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            self.instance,
            self.partitioner,
            self.variable_heuristic,
            self.blocks,
            self.cut_size,
            self.time_original,
            self.time_conditioned,
            self.count_original,
            self.count_conditioned,
            self.time_solving
                .iter()
                .map(u128::to_string)
                .collect::<Vec<String>>()
                .join(";"),
            self.time_solving.iter().sum::<u128>(),
            self.time_partitioning,
        ));
    }
}

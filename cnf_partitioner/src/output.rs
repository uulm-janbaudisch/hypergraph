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
    blocks: usize,
    cut_size: usize,
    time_partitioning: u128,
    time_solving: u128,
    time_original_solving: u128,
    count: BigInt,
    original_count: BigInt,
}

impl Run {
    /// Creates a new run.
    pub fn new(
        instance: String,
        partitioner: &str,
        blocks: usize,
        cut_size: usize,
        time_original_solving: u128,
        original_count: BigInt,
        time_partitioning: u128,
    ) -> Self {
        Self {
            instance,
            partitioner: String::from(partitioner),
            blocks,
            cut_size,
            time_original_solving,
            original_count,
            time_partitioning,
            count: BigInt::from(1),
            ..Default::default()
        }
    }

    /// Adds the result of solving a part of the split CNF.
    pub fn add_part(&mut self, time: u128, count: BigInt) {
        self.time_solving += time;
        self.count *= count;
    }

    /// Checks the result after all partial runs for correctness.
    pub fn check(&self) {
        assert_eq!(
            self.original_count, self.count,
            "The counts between the original and split CNFs (collectively) should be equal."
        );
    }

    /// The output CSV header describing the contents of runs.
    pub const fn csv_header() -> &'static str {
        "instance,partitioner,blocks,cut_size,count,inclusive_diff_abs,inclusive_diff_rel,exclusive_diff_abs,exclusive_diff_rel"
    }

    /// Serializes a run into a CSV row.
    pub fn csv(&self, output: &mut String) {
        // Calculate the time difference both absolut and relative to the original instance.
        // Once not taking partitioning into account.
        let total_time_exclusive = self.time_solving;
        let exclusive_difference_absolut =
            total_time_exclusive as i128 - self.time_original_solving as i128;
        let exclusive_difference_relative =
            (exclusive_difference_absolut as f32 / total_time_exclusive as f32) * 100f32;

        // And once taking partitioning into account.
        let total_time_inclusive = self.time_solving + self.time_partitioning;
        let inclusive_difference_absolut =
            total_time_inclusive as i128 - self.time_original_solving as i128;
        let inclusive_difference_relative =
            (inclusive_difference_absolut as f32 / total_time_inclusive as f32) * 100f32;

        output.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}",
            self.instance,
            self.partitioner,
            self.blocks,
            self.cut_size,
            self.count,
            exclusive_difference_absolut,
            exclusive_difference_relative,
            inclusive_difference_absolut,
            inclusive_difference_relative
        ));
    }
}

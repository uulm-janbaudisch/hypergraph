use log::{debug, info, trace, warn};
use num::BigInt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::{from_utf8, FromStr};
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::thread::{sleep, yield_now};
use std::time::{Duration, Instant};

/// Wrapper around `d4` for preprocessing and compiling a CNF.
pub struct D4(PathBuf);

impl D4 {
    /// Creates a new d4 instance using the binary at the given path.
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    /// Processes a CNF with `d4` while writing the preprocessed input to the specified file
    /// and returning the time taken and the model count.
    pub fn preprocess(&self, cnf: PathBuf, out_file: &Path) -> Result<u128, ()> {
        // Create the d4 process and run it.
        let mut run = Command::new(&self.0);

        run.args([
            "--input",
            cnf.to_str().expect("Failed to serialize cnf path."),
            "--method",
            "ddnnf-compiler",
            "--only-preproc",
            "1",
            "--dump-preproc",
            out_file
                .to_str()
                .expect("Failed to serialize preprocessed output file."),
        ]);

        trace!("{:?}", run);

        let start = Instant::now();
        run.output().expect("Failed to run d4 preprocessing.");
        Ok(start.elapsed().as_millis())
    }

    /// Compiles a CNF into a d-DNNF, returning the time taken and the model count.
    pub fn compile(&self, cnf: PathBuf, timeout: Option<u64>) -> Result<(u128, BigInt), ()> {
        debug!("Running d4 on: {:?}", cnf);

        if let Some(timeout) = timeout {
            debug!("Timeout at: {:?} s", timeout);
        }

        // Create the d4 process and run it.
        let mut run = Command::new(&self.0);
        run.args([
            "--input",
            cnf.to_str().expect("Failed to serialize cnf path."),
            "--method",
            "ddnnf-compiler",
            "--partitioning-heuristic",
            "none",
        ])
        .stdout(Stdio::piped());

        trace!("{:?}", run);

        // Start the d4 process and track the time.
        let start = Instant::now();
        let mut process = run.spawn().expect("Failed to spawn d4 process.");

        if let Some(timeout) = timeout {
            // Spawn a thread for the timeout.
            let (timeout_sender, timeout_receiver) = channel();
            thread::spawn(move || {
                sleep(Duration::from_secs(timeout));
                let _ = timeout_sender.send(());
            });

            // Wait for either the d4 process or the timeout to finish.
            loop {
                // In case the process finises, continue.
                if process
                    .try_wait()
                    .expect("Failed to check for d4 process.")
                    .is_some()
                {
                    break;
                }

                let timeout_status = timeout_receiver.try_recv();

                // In case the timeout is reached, indicate a not finished operation.
                if let Ok(()) = timeout_status {
                    warn!("Timeout reached.");
                    process.kill().expect("Failed to kill d4 process.");
                    return Ok((u128::MAX, BigInt::ZERO));
                }

                // Panic in case the timeout thread disconnected.
                if let Err(TryRecvError::Disconnected) = timeout_status {
                    panic!("Timeout process disconnected.");
                }

                yield_now();
            }
        }

        let output = process
            .wait_with_output()
            .expect("Failed to wait for d4 process.");

        let duration = start.elapsed().as_millis();

        let mut last_line = from_utf8(&output.stdout)
            .expect("Failed to read output from d4.")
            .lines()
            .last()
            .expect("No output from d4.")
            .split_whitespace();

        assert_eq!(
            last_line.next(),
            Some("s"),
            "Last line of output should start with 's'."
        );

        // Extract the model count.
        let count = BigInt::from_str(
            last_line
                .next()
                .expect("Last line of output should contain the model count."),
        )
        .expect("Failed to read model count.");

        info!("d4 took {} ms, model count: {}", duration, count);

        Ok((duration, count))
    }
}

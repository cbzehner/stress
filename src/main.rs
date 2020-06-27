use structopt::StructOpt;

mod stress;

/// Put your programs to the test. Run a command in a loop and collect failures.
#[derive(Clone, Debug, StructOpt)]
pub struct Cli {
    /// The command to run. Precede this command with -- in order to pass in flags.
    /// Usage:
    ///   stress --count 10 -- echo "hello world"
    ///   stress -- ls -a
    #[structopt(required = true, min_values = 1, verbatim_doc_comment)]
    cmd: Vec<String>,
    /// Exit immediately upon the first non-zero exit code.
    #[structopt(short, long)]
    bail: bool,
    /// Display the output (stdout) from runs. The output is grouped together by exit code.
    /// Each exit code will only show output for one run, even if there were several runs with that exit code.
    #[structopt(short, long)]
    output: bool,
    /// The number of times to run the command.
    #[structopt(short, long, default_value = "10")]
    runs: usize,
    // Turn off parallelization of runs (default).
    // #[structopt(short, long)]
    // serial: bool,
}

fn main() {
    let cli = Cli::from_args();
    let stress = stress::Stress::new(cli);
    stress.run()
}

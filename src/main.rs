use structopt::StructOpt;

mod stress;

/// Put your programs to the test. Run a command in a loop and collect failures.
#[derive(Clone, Debug, StructOpt)]
pub struct Cli {
    #[structopt(required = true, min_values = 1, verbatim_doc_comment)]
    /// The command to run. Precede this command with -- in order to pass in flags.
    /// Usage:
    ///   stress --count 10 -- echo "hello world"
    ///   stress -- ls -a
    cmd: Vec<String>,
    /// Exit immediately upon the first non-zero exit code.
    #[structopt(short, long)]
    bail: Option<bool>,
    /// The number of times to run the command.
    #[structopt(short, long, default_value = "10")]
    count: usize,
    /// Turn off parallelization of runs (default).
    #[structopt(short, long)]
    serial: Option<bool>,
}

fn main() {
    let cli = Cli::from_args();
    let stress = stress::Stress::new(cli);
    stress.run()
}

use std::collections::HashMap;
use std::fmt;
use std::process::{Command, Output};

use structopt::StructOpt;

/// Put your programs to the test. Run a command in a loop and collect failures.
#[derive(StructOpt)]
struct Cli {
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

const SUCCESS: i32 = 0;

fn main() {
    let cli = Cli::from_args();
    let stress = Stress::new(cli);
    stress.run()
}

/// The primary control structure for this utility.
struct Stress {
    cmd: Cmd,
    results: HashMap<i32, Outcome>,
    runs: usize,
}

impl Stress {
    pub fn new(cli: Cli) -> Self {
        let runs = cli.count;
        let cmd = Cmd::new(cli);
        let results = HashMap::new();

        Stress { cmd, runs, results }
    }

    pub fn run(mut self) -> () {
        // Get ready to rumble.
        println!("Running \"{}\" {} times...", self.cmd, self.runs);

        // Run the command the specified number of times.
        for _ in 0..self.runs {
            let output = self.cmd.clone().execute();
            let exit_code = output.status.code().expect("failed to exit cleanly");

            // Store the results.
            // TODO: Optionally include output from failures.
            let outcome = self
                .results
                .entry(exit_code)
                .or_insert(Outcome { exit_code, runs: 0 });
            outcome.runs += 1;
        }

        // Print out the results.
        println!("Over the course of {} runs of \"{}\"", self.runs, self.cmd);
        if self.results.contains_key(&SUCCESS) {
            println!("[Success]");
            println!("Exit Code\tOccurrences");
            println!("{}", self.results.get(&SUCCESS).unwrap());
            println!("");
            self.results.remove(&0);
        } else {
            println!("[No Successes]");
            println!("");
        }
        if self.results.len() > 0 {
            println!("[Failure]");
            println!("Exit Code\tOccurrences");
            for (_, outcome) in self.results.iter() {
                println!("{}", outcome);
            }
        } else {
            println!("[No Failures]");
        }
    }
}

/// The command to be executed.
#[derive(Clone)]
struct Cmd {
    program: String,
    args: Args,
}

impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.program, self.args)
    }
}

impl Cmd {
    pub fn new(cli: Cli) -> Self {
        let program = String::from(&cli.cmd[0]);
        let args = Args {
            list: Vec::from(&cli.cmd[1..cli.cmd.len()]),
        };

        Cmd { program, args }
    }

    pub fn execute(self) -> Output {
        Command::new(&self.program[..])
            .args(&self.args.list)
            .output()
            .expect("failed to execute process")
    }
}

/// The arguments passed to the subcommand.
#[derive(Clone)]
struct Args {
    list: Vec<String>,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = self
            .list
            .iter()
            .map(|s| s.chars())
            .flatten()
            .collect::<String>();
        write!(f, "{}", display)
    }
}

/// Store the outcome of each run for analysis.
struct Outcome {
    exit_code: i32,
    runs: i32,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t\t{}", self.exit_code, self.runs)
    }
}

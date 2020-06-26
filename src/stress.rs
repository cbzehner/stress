use std::collections::HashMap;
use std::fmt;
use std::process::{Command, Output};

use crate::Cli;

const SUCCESS: i32 = 0;

/// The primary control structure for this utility.
pub struct Stress {
    cmd: Cmd,
    config: Config,
    results: HashMap<i32, Outcome>,
}

impl Stress {
    pub fn new(cli: Cli) -> Self {
        let cmd = Cmd::new(cli.clone());
        let config = Config::new(cli);
        let results = HashMap::new();

        Stress {
            cmd,
            config,
            results,
        }
    }

    pub fn run(mut self) -> () {
        // Get ready to rumble.
        println!("Running \"{}\" {} times...", self.cmd, self.config.runs);

        // Run the command the specified number of times.
        for _ in 0..self.config.runs {
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
        println!(
            "Over the course of {} runs of \"{}\"",
            self.config.runs, self.cmd
        );
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
#[derive(Clone, Debug)]
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
    fn new(cli: Cli) -> Self {
        let program = String::from(&cli.cmd[0]);
        let args = Args {
            list: Vec::from(&cli.cmd[1..cli.cmd.len()]),
        };

        Cmd { program, args }
    }

    fn execute(self) -> Output {
        Command::new(&self.program[..])
            .args(&self.args.list)
            .output()
            .expect("failed to execute process")
    }
}

/// The arguments passed to the subcommand.
#[derive(Clone, Debug)]
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

/// Configuration passed in from the commandline.
#[derive(Clone, Debug)]
struct Config {
    bail: Option<bool>,
    runs: usize,
    serial: Option<bool>,
}

impl Config {
    fn new(cli: Cli) -> Self {
        Config {
            runs: cli.count,
            bail: cli.bail,
            serial: cli.serial,
        }
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

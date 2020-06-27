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
        println!("Starting {} runs of \"{}\"...", self.config.runs, self.cmd);
        // If the --bail flag is set, the program may halt early. Record the actual run count for display later.
        let mut cmd_runs = 0;

        // Run the command the specified number of times.
        for _ in 0..self.config.runs {
            let output = self.cmd.clone().execute();
            let exit_code = output.status.code().expect("failed to exit cleanly");
            cmd_runs += 1;

            // Store the results.
            let outcome = self.results.entry(exit_code).or_insert(Outcome {
                exit_code,
                runs: 0,
                stdout: None,
            });
            outcome.runs += 1;

            // Store the stdout from the run if one hasn't already been seen
            if self.config.output && outcome.stdout.is_none() {
                let stdout = String::from_utf8(output.stdout).unwrap_or_default();
                outcome.stdout = Some(stdout)
            }

            // Exit at the first non-Success value if --bail is enabled
            if self.config.bail && exit_code != SUCCESS {
                break;
            }
        }

        // Print out the results. Results are printed in a different format depending on whether
        // the --output flag is enabled or not.
        let mut codes: Vec<_> = self.results.keys().collect();
        codes.sort(); // Display the exit codes in numeric order
        if self.config.output {
            for exit_code in codes {
                let outcome = self.results.get(exit_code).unwrap();
                let result = if outcome.exit_code == SUCCESS {
                    "success"
                } else {
                    "failure"
                };
                println!(
                    "Exit Code {} ({}) occurred {} time(s). Output:",
                    outcome.exit_code, result, outcome.runs,
                );
                match &outcome.stdout {
                    Some(stdout) => println!("{}", stdout),
                    None => println!("No output recorded"),
                }
            }
        } else {
            println!("Exit Code\tRuns\tResult");
            for exit_code in codes {
                let outcome = self.results.get(exit_code).unwrap();
                let result = if outcome.exit_code == SUCCESS {
                    "success"
                } else {
                    "fail"
                };
                println!("{}\t{}", outcome, result);
            }
        }
        println!("Completed {} runs of \"{}\"", cmd_runs, self.cmd);
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
        let cmd_str = format!("{} {}", self.program, self.args);
        write!(f, "{}", cmd_str.trim())
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
    bail: bool,
    output: bool,
    runs: usize,
    // serial: bool,
}

impl Config {
    fn new(cli: Cli) -> Self {
        Config {
            bail: cli.bail,
            output: cli.output,
            runs: cli.runs,
            // serial: cli.serial,
        }
    }
}

/// Store the outcome of each run for analysis.
struct Outcome {
    exit_code: i32,
    runs: i32,
    stdout: Option<String>,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t\t{}", self.exit_code, self.runs)
    }
}

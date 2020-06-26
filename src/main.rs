use std::collections::HashMap;
use std::process::Command;

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
    count: i8,
    /// Turn off parallelization of runs (default).
    #[structopt(short, long)]
    serial: Option<bool>,
}

fn main() {
    // Parse arguments from the user.
    let cli = Cli::from_args();
    let loops = cli.count;
    let (command, arguments) = parse_args(cli);
    let (arg_str, args) = arguments;

    // Get ready to rumble.
    println!(
        "Running \"{} {}\" {} times...",
        command,
        arg_str,
        args.len()
    );
    let mut runs = HashMap::new();

    // Run the command the specified number of times.
    for _ in 0..loops {
        let output = Command::new(&command[..])
            .args(&args)
            .output()
            .expect("failed to execute process");
        let exit_code = output.status.code().expect("failed to exit cleanly");

        // Store the results.
        // TODO: Optionally include output from failures.
        let run = runs.entry(exit_code).or_insert(0);
        *run += 1; // Increment the count.
    }

    // Print out the results.
    println!(
        "Over the course of {} runs of \"{} {}\"",
        args.len(),
        command,
        arg_str
    );
    if runs.contains_key(&0) {
        println!("[Success]");
        println!("Exit Code\tOccurrences");
        println!("0\t\t{}", runs.get(&0).unwrap());
        runs.remove(&0);
    }
    if runs.len() > 0 {
        println!("[Failure]");
        println!("Exit Code\tOccurrences");
        for (exit_code, count) in runs.iter() {
            println!("{}\t\t{}", exit_code, count);
        }
    }
}

fn parse_args(cli: Cli) -> (String, (String, Vec<String>)) {
    let command = String::from(&cli.cmd[0]);
    let arguments_vec: Vec<String> = Vec::from(&cli.cmd[1..cli.cmd.len()]);
    // TODO: Move this into a Display trait on a struct
    let arguments_str: String = arguments_vec.iter().map(|s| s.chars()).flatten().collect();

    (command, (arguments_str, arguments_vec))
}

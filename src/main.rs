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
    /// The number of times to run the command
    #[structopt(short, long, default_value = "10")]
    count: i8,
}

fn main() {
    // Parse arguments from the user.
    let args = Cli::from_args();
    let command = &args.cmd[0];
    let arguments: String = args.cmd[1..args.cmd.len()]
        .iter()
        .map(|s| s.chars())
        .flatten()
        .collect();

    // Get ready to rumble.
    println!(
        "Running \"{} {}\" {} times...",
        command, arguments, &args.count
    );
    let mut runs = HashMap::new();

    // Run the command the specified number of times.
    for _ in 0..args.count {
        let output = Command::new(&args.cmd[0])
            .args(&args.cmd[1..args.cmd.len()])
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
        &args.count, command, arguments
    );
    if runs.contains_key(&0) {
        println!("[Success]");
        println!("Exit Code\tOccurrences");
        println!("0\t{}", runs.get(&0).unwrap());
        runs.remove(&0);
    }
    if runs.len() > 0 {
        println!("[Failure]");
        println!("Exit Code\tOccurrences");
        for (exit_code, count) in runs.iter() {
            println!("{}\t{}", exit_code, count);
        }
    }
}

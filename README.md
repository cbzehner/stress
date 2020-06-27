# stress

Put your programs to the test. Run a command in a loop and collect failures.

## Usage

```
stress 0.3.0
Put your programs to the test. Run a command in a loop and collect failures

USAGE:
    stress [FLAGS] [OPTIONS] <cmd>...

FLAGS:
    -b, --bail       Exit immediately upon the first non-zero exit code
    -h, --help       Prints help information
    -o, --output     Display the output (stdout) from runs. The output is grouped together by exit code. Each exit code
                     will only show output for one run, even if there were several runs with that exit code
    -V, --version    Prints version information

OPTIONS:
    -r, --runs <runs>    The number of times to run the command [default: 10]

ARGS:
    <cmd>...    The command to run. Precede this command with -- in order to pass in flags.
                Usage:
                  stress --count 10 -- echo "hello world"
                  stress -- ls -a
```

## Installation

Install with `cargo install stress`

## Alternatives

- [cargo-stress](https://lib.rs/crates/cargo-stress)

## Contributions

Contributions in the form of feedback via GitHub Issues or code contributions via Pull Requests are welcome!

If you have functionality you want to add and don't know where to start, open an Issue so I can help out!

# Credits

- Timo's [retry-cmd](https://github.com/timofurrer/retry-cmd) inspired me to use `std::process::Command` instead of relying on `run_script`.

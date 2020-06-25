# stress

Put your programs to the test. Run a command multiple times and get back a summary of its exit codes.

## Usage

```
stress 0.1.0
Run a command in a loop and collect failures

USAGE:
    stress [OPTIONS] [cmd]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --count <count>    The number of times to run the command [default: 10]

ARGS:
    <cmd>...    The command to run
```

## Installation

Install with `cargo install stress`

## Alternatives

- [cargo-stress](https://lib.rs/crates/cargo-stress)

# Credits

- Timo's [retry-cmd](https://github.com/timofurrer/retry-cmd) inspired me to use `std::process::Command` instead of relying on `run_script`.

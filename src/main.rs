mod args;
mod token;
mod analyzer;


use std::process::ExitCode;
use args::Arg;
use clap::Parser;

fn main() -> ExitCode{
    let args = Arg::parse();
    let codes = std::fs::read_to_string(args.path).unwrap();
    analyzer::Analyzer::new(&codes);

    ExitCode::SUCCESS
}
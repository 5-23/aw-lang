mod args;


use std::process::ExitCode;
use args::Arg;
use clap::Parser;

fn main() -> ExitCode{
    let args = Arg::parse();
    println!("{}", std::fs::read_to_string(args.path).unwrap());
    ExitCode::SUCCESS
}
mod wrapper;

use clap::Parser;
use std::process::ExitCode;
use wrapper::Wrapper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    Wrapper::new().run(args.file)
}

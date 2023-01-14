mod wrapper;

use clap::Parser;
use std::process::ExitCode;
use wrapper::Wrapper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: String,

    /// Turn debugging information on (requires debug build - not available in release build)
    #[arg(short, long)]
    debug: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut wrapper = Wrapper::new(args.debug);

    wrapper.run(args.file)
}

mod shell;

use std::process::ExitCode;
use clap::Parser;
use shell::Shell;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: Option<String>,

    #[arg(short, long)]
    debug: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut shell = Shell::new(args.debug);

    if let Some(f) = args.file {
        shell.file(f)
    } else {
        shell.prompt()
    }
}

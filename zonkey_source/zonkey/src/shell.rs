use std::{process::ExitCode, io::{stdout, stdin, Write}, fs::read_to_string, path::Path};
use interpreter::Interpreter;

pub struct Shell {
    interpreter: Interpreter,
}

impl Shell {
    pub fn new(debug: bool) -> Self {
        if debug {
            println!("Debug mode is on");
        }

        Self {
            interpreter: Interpreter::new(debug),
        }
    }

    pub fn prompt(&self) -> ExitCode {
        loop {
            print!("> ");
            stdout()
                .flush()
                .expect("Failed to flush stdout for prompt.");

            let mut input = String::new();

            stdin()
                .read_line(&mut input)
                .expect("Failed to read line for input.");

            let command = input.trim();

            if command == "exit" {
                break;
            }

            match self.interpreter.run(command) {
                Ok(()) => (),
                Err(e) => println!("Error: {e}"),
            };
        }

        ExitCode::SUCCESS
    }

    pub fn file(&self, file: String) -> ExitCode {
        let source = match read_to_string(Path::new(&file)) {
            Ok(s) => s,
            Err(_) => {
                eprint!("Error: Couldn't open file.");
                // Exit code for non-existent or unreadable file
                return ExitCode::from(66);
            }
        };

        match self.interpreter.run(&source) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                println!("Error running script {file}: {e}");
                ExitCode::FAILURE
            }
        }
    }
}

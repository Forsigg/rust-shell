use std::io::{self, Write};

use crate::{
    builtins::{handle_builtin_command, parse_builtin_command},
    commands::execute_external,
};

pub mod builtins;
pub mod commands;

fn main() {
    loop {
        print!("$ ");
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let command_parts: Vec<&str> = input.split_ascii_whitespace().collect();
                let command = command_parts[0];
                let args = &command_parts[1..];

                if let Some(cmd) = parse_builtin_command(command) {
                    if let Some(output) = handle_builtin_command(cmd, args) {
                        print!("{}", output);
                        if !output.ends_with("\n") {
                            println!();
                        }
                    }
                } else if let Err(e) = execute_external(command, args) {
                    eprintln!("{e}")
                }
            }

            Err(e) => println!("error: {e}"),
        }
    }
}

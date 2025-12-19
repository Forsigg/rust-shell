use std::io::{self, Write};

use crate::{
    builtins::{handle_builtin_command, parse_builtin_command},
    commands::execute_external, output::handle_output,
};

pub mod builtins;
pub mod commands;
pub mod output;

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

                let mut output = String::new();

                match parse_builtin_command(command) {
                    Some(cmd) => {
                        if let Some(cmd_output) = handle_builtin_command(cmd, args) {
                            output.push_str(&cmd_output);
                        }
                    }
                    None => match execute_external(command, args) {
                        Ok(cmd_output) => {
                            output.push_str(&cmd_output);
                        }
                        Err(_) => eprintln!("{command}: not found"),
                    },
                }

                handle_output(args, output);
            }

            Err(e) => println!("error: {e}"),
        }
    }
}

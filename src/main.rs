use std::io::{self, Write};

use crate::{
    builtins::{handle_builtin_command, parse_builtin_command},
    commands::execute_external,
    output::{RedirectType, handle_output, separare_redirect_and_args},
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
                let (real_args, redirect) = separare_redirect_and_args(args);

                let mut output = String::new();

                match parse_builtin_command(command) {
                    Some(cmd) => {
                        if let Some(cmd_output) = handle_builtin_command(cmd, real_args) {
                            output = cmd_output;
                        }
                    }
                    None => match execute_external(command, real_args) {
                        Ok((cmd_output, cmderr)) => {
                            if !cmderr.is_empty() {
                                handle_output(cmderr, RedirectType::None);
                            } 
                            output = cmd_output;
                        }
                        Err(_) => eprintln!("{command}: not found"),
                    },
                }

                handle_output(output, redirect);
            }

            Err(e) => println!("error: {e}"),
        }
    }
}

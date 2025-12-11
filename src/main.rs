#[allow(unused_imports)]
use std::io::{self, Write};

use crate::commands::{handle_command, parse_command};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let mut command_parts = input.split_ascii_whitespace();
                let command_str = command_parts.next().unwrap();
                let mut args: Vec<&str> = vec![];

                for arg in command_parts {
                    args.push(arg);
                }

                if let Some(cmd) = parse_command(command_str) {
                    handle_command(cmd, args);
                } else {
                    eprintln!("{command_str}: command not found")
                }
            }

            Err(e) => println!("error: {e}"),
        }
    }
}

mod commands;

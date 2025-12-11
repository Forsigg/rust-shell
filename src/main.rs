use std::io::{self, Write};

use crate::commands::{handle_command, parse_command};

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
                let args = command_parts[1..].into();

                if let Some(cmd) = parse_command(command) {
                    handle_command(cmd, args);
                } else {
                    eprintln!("{command}: command not found")
                }
            }

            Err(e) => println!("error: {e}"),
        }
    }
}

mod commands;

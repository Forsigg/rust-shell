#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

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
                let command = command_parts.next().unwrap();

                match command {
                    "exit" => {exit(0)},
                    _ => println!("{}: command not found", command)
                }
                
            }
            Err(e) => println!("error: {e}"),
        }
    }
}

use std::process::exit;
use std::{env, fs};

pub enum ShellCommand {
    Exit,
    Echo,
    Type,
}

pub enum ShellError {
    CommandNotFound,
}

pub fn parse_command(command_str: &str) -> Option<ShellCommand> {
    match command_str {
        "exit" => Some(ShellCommand::Exit),
        "echo" => Some(ShellCommand::Echo),
        "type" => Some(ShellCommand::Type),
        _ => None,
    }
}

pub fn handle_command(cmd: ShellCommand, args: Vec<&str>) {
    match cmd {
        ShellCommand::Exit => exit(0),
        ShellCommand::Echo => echo(args),
        ShellCommand::Type => type_(args[0]),
    }
}

fn echo(args: Vec<&str>) {
    for arg in args {
        print!("{arg} ")
    }
    println!();
}

fn type_(arg: &str) {
    match parse_command(arg) {
        Some(_) => println!("{arg} is a shell builtin"),

        None => {
            if let Ok(path) = env::var("PATH") {
                for p in path.split(":") {
                    let p_str = format!("{}/{}", p, arg);
                    if fs::metadata(p_str).is_ok() {
                        println!("{arg} is {p}/{arg}")
                    }
                }
            } else {
                println!("{arg}: not found")
            }
        }
    }
}

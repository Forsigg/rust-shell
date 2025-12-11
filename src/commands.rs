use std::process::exit;

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
        ShellCommand::Type => {type_(args[0])}
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
        None => println!("{arg}: not found"),
        Some(_) => println!("{arg} is a shell builtin")
    }
}

use core::fmt;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, exit};
use std::{env, fs};

#[derive(Debug, Clone)]
pub struct CommandNotFoundError {
    command: String,
}

impl CommandNotFoundError {
    pub fn new(command: String) -> Self {
        CommandNotFoundError { command }
    }
}

impl fmt::Display for CommandNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = &self.command;
        write!(f, "{command}: not found")
    }
}

pub enum ShellCommand {
    Exit,
    Echo,
    Type,
}

pub fn parse_command(command_str: &str) -> Option<ShellCommand> {
    match command_str {
        "exit" => Some(ShellCommand::Exit),
        "echo" => Some(ShellCommand::Echo),
        "type" => Some(ShellCommand::Type),
        _ => None,
    }
}

pub fn handle_command(cmd: ShellCommand, args: &[&str]) {
    match cmd {
        ShellCommand::Exit => exit(0),
        ShellCommand::Echo => echo(args),
        ShellCommand::Type => type_(args[0]),
    }
}

fn echo(args: &[&str]) {
    for &arg in args {
        print!("{arg} ")
    }
    println!();
}

fn type_(arg: &str) {
    match parse_command(arg) {
        Some(_) => println!("{arg} is a shell builtin"),

        None => {
            if let Some(exec_path) = is_external_executable_exist(arg) {
                println!("{arg} is {exec_path}");
            } else {
                println!("{arg}: not found");
            }
        }
    }
}

fn is_external_executable_exist(program_name: &str) -> Option<String> {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program_name);
            if let Ok(md) = fs::metadata(&p_str) {
                let permissions = md.permissions();
                if permissions.mode() & 0o111 != 0 {
                    return Some(p_str);
                }
            }
        }
    }
    None
}

pub fn execute_external(program_name: &str, args: &[&str]) -> Result<(), CommandNotFoundError> {
    match is_external_executable_exist(program_name) {
        Some(path) => {
            let mut command = Command::new(path);
            for &arg in args {
                command.arg(arg);
            }
            let output = command.output().unwrap();
            let _ = io::stdout().write_all(&output.stdout);
            let _ = io::stderr().write_all(&output.stderr);
            Ok(())
        }
        None => Err(CommandNotFoundError::new(program_name.to_owned())),
    }
}

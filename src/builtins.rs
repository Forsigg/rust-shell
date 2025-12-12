use std::{env::current_dir, fs, process::exit};

use crate::commands::is_external_executable_exist;

pub enum ShellCommand {
    Exit,
    Echo,
    Type,
    Pwd
}

pub fn parse_builtin_command(command_str: &str) -> Option<ShellCommand> {
    match command_str {
        "exit" => Some(ShellCommand::Exit),
        "echo" => Some(ShellCommand::Echo),
        "type" => Some(ShellCommand::Type),
        "pwd" => Some(ShellCommand::Pwd),
        _ => None,
    }
}

pub fn handle_builtin_command(cmd: ShellCommand, args: &[&str]) {
    match cmd {
        ShellCommand::Exit => exit(0),
        ShellCommand::Echo => echo(args),
        ShellCommand::Type => type_(args[0]),
        ShellCommand::Pwd => pwd(),
    }
}

fn type_(arg: &str) {
    match parse_builtin_command(arg) {
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

fn echo(args: &[&str]) {
    for &arg in args {
        print!("{arg} ")
    }
    println!();
}

fn pwd() {
    let pwd = current_dir().unwrap();
    let absolute_pwd = fs::canonicalize(&pwd).unwrap();
    let pwd_str = absolute_pwd.to_str().unwrap();
    println!("{pwd_str}");
}

use std::{
    env::{self, current_dir, set_current_dir},
    fs,
    path::PathBuf,
    process::exit,
};

use crate::commands::is_external_executable_exist;

pub enum ShellCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

pub fn parse_builtin_command(command_str: &str) -> Option<ShellCommand> {
    match command_str {
        "exit" => Some(ShellCommand::Exit),
        "echo" => Some(ShellCommand::Echo),
        "type" => Some(ShellCommand::Type),
        "pwd" => Some(ShellCommand::Pwd),
        "cd" => Some(ShellCommand::Cd),
        _ => None,
    }
}

pub fn handle_builtin_command(cmd: ShellCommand, args: &[&str]) -> Option<String> {
    match cmd {
        ShellCommand::Exit => exit(0),
        ShellCommand::Echo => Some(echo(args)),
        ShellCommand::Type => Some(type_(args[0])),
        ShellCommand::Pwd => Some(pwd()),
        ShellCommand::Cd => cd(args[0].to_owned()),
    }
}

fn type_(arg: &str) -> String {
    match parse_builtin_command(arg) {
        Some(_) => format!("{arg} is a shell builtin"),

        None => {
            if let Some(exec_path) = is_external_executable_exist(arg) {
                format!("{arg} is {exec_path}")
            } else {
                format!("{arg}: not found")
            }
        }
    }
}

fn echo(args: &[&str]) -> String {
    let mut output = String::new();
    for arg in args {
        output.push_str(&(format!("{} ", arg)));
    }
    output
}

fn get_pwd() -> PathBuf {
    let pwd = current_dir().unwrap();
    fs::canonicalize(&pwd).unwrap()
}

fn pwd() -> String {
    let absolute_pwd = get_pwd();
    absolute_pwd.to_str().unwrap().to_owned()
}

fn cd(mut new_dir: String) -> Option<String> {
    if new_dir == "~" {
        new_dir = env::var("HOME").unwrap();
    }

    let mut path = PathBuf::from(&new_dir);

    if !path.is_absolute() {
        path = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(_) => {
                return Some(format!("cd: {}: No such file or directory", &new_dir));
            }
        }
    }

    if path.exists() {
        let _ = set_current_dir(path);
        None
    } else {
        Some(format!("cd: {}: No such file or directory", &new_dir))
    }
}

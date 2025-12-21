use std::{
    env::{self, current_dir, set_current_dir},
    fs,
    path::PathBuf,
    process::exit,
};

use crate::commands::is_external_executable_exist;

/// Builtin commands
pub enum ShellCommand {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
}

/// Parse builtin command.
/// If command not builtin - return None.
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

/// Handle builtin command.
/// Dispatch builtin implementation by cmd enumeration.
pub fn handle_builtin_command(cmd: ShellCommand, args: &[&str]) -> (String, String) {
    match cmd {
        ShellCommand::Exit => exit(0),
        ShellCommand::Echo => echo(args),
        ShellCommand::Type => type_(args),
        ShellCommand::Pwd => pwd(),
        ShellCommand::Cd => cd(args),
    }
}

/// Implementation of `type` command builtin.
fn type_(args: &[&str]) -> (String, String) {
    let mut out = String::new();
    let mut err = String::new();

    if args.is_empty() {
        err.push_str("type: need at least one argument");
        return (out, err)
    }

    let arg = args[0];

    match parse_builtin_command(arg) {
        Some(_) => {
            out.push_str(&format!("{arg} is a shell builtin"));
        }
        None => {
            if let Some(exec_path) = is_external_executable_exist(arg) {
                out.push_str(&format!("{arg} is {exec_path}"));
            } else {
                err.push_str(&format!("{arg}: not found"));
            }
        }
    }

    (out, err)
}

/// Implementation of `echo` command builtin.
fn echo(args: &[&str]) -> (String, String) {
    let mut output = String::new();
    for arg in args {
        output.push_str(&(format!("{} ", arg)));
    }
    (output, String::from(""))
}

/// Get working directory.
fn get_wd() -> PathBuf {
    let pwd = current_dir().unwrap();
    fs::canonicalize(&pwd).unwrap()
}

/// Implementation of `pwd` command builtin.
fn pwd() -> (String, String) {
    let absolute_pwd = get_wd();
    (absolute_pwd.to_str().unwrap().to_owned(), String::new())
}

/// Implementation of `cd` command builtin.
fn cd(args: &[&str]) -> (String, String) {
    let mut new_dir: String;

    let out = String::new();
    let mut err = String::new();

    if args.is_empty() {
        new_dir = env::var("HOME").unwrap();
    } else {
        new_dir = args[0].to_owned();
    }

    if new_dir == "~" {
        new_dir = env::var("HOME").unwrap();
    }

    let mut path = PathBuf::from(&new_dir);

    if !path.is_absolute() {
        path = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(_) => {
                err.push_str(&format!("cd: {}: No such file or directory", &new_dir));
                return (out, err);
            }
        }
    }

    if path.exists() {
        let _ = set_current_dir(path);
    } else {
        err.push_str(&format!("cd: {}: No such file or directory", &new_dir));
    }

    (out, err)
}

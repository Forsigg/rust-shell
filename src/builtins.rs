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

pub fn handle_builtin_command(cmd: ShellCommand, args: &[&str]) {
    match cmd {
        ShellCommand::Exit => exit(0),
        ShellCommand::Echo => echo(args),
        ShellCommand::Type => type_(args[0]),
        ShellCommand::Pwd => pwd(),
        ShellCommand::Cd => cd(args[0].to_owned()),
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

fn get_pwd() -> PathBuf {
    let pwd = current_dir().unwrap();
    fs::canonicalize(&pwd).unwrap()
}

fn pwd() {
    let absolute_pwd = get_pwd();
    let pwd_str = absolute_pwd.to_str().unwrap();
    println!("{pwd_str}");
}

fn cd(mut new_dir: String) {
    if new_dir == "~" {
        new_dir = env::var("HOME").unwrap();
    }
    
    let mut path = PathBuf::from(&new_dir);

    if !path.is_absolute() {
        path = match fs::canonicalize(path) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("cd: {}: No such file or directory", &new_dir);
                return
            }
        }
    }

    if path.exists() {
        let _ = set_current_dir(path);
    } else {
        eprintln!("cd: {}: No such file or directory", &new_dir);
    }
}

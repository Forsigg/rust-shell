use core::fmt;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
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

/// Chech is program_name is external executable exist. 
/// If exist - return path to executable.
pub fn is_external_executable_exist(program_name: &str) -> Option<String> {
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

/// Execute external executable program.
pub fn execute_external(
    program_name: &str,
    args: &[&str],
) -> Result<(String, String), CommandNotFoundError> {
    match is_external_executable_exist(program_name) {
        Some(_) => {
            let mut command = Command::new(program_name);
            for &arg in args {
                command.arg(arg);
            }
            let output = command.output().unwrap();
            Ok((
                str::from_utf8(&output.stdout).unwrap().to_owned(),
                str::from_utf8(&output.stderr).unwrap().to_owned(),
            ))
        }
        None => Err(CommandNotFoundError::new(program_name.to_owned())),
    }
}

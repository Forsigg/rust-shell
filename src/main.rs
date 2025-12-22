use std::{env, path::PathBuf};

use crate::{
    autocompletion::ShellHelper,
    builtins::{handle_builtin_command, parse_builtin_command},
    commands::execute_external,
    output::{handle_output, separare_redirect_and_args},
};
use rustyline::{error::ReadlineError, Editor, Result};

pub mod autocompletion;
pub mod builtins;
pub mod commands;
pub mod output;

fn main() -> Result<()> {
    let mut editor: Editor<ShellHelper, _> = Editor::new()?;
    let mut helper = ShellHelper::new();

    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            helper.add_completions_from_path(PathBuf::from(p));
        }
    }

    editor.set_helper(Some(helper));

    loop {
        let readline = editor.readline("$ ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }

                let command_parts: Vec<&str> = line.split_ascii_whitespace().collect();
                let command = command_parts[0];
                let args = &command_parts[1..];
                let (real_args, redirect) = separare_redirect_and_args(args);

                let mut output = String::new();
                let mut output_err = String::new();

                match parse_builtin_command(command) {
                    Some(cmd) => {
                        (output, output_err) = handle_builtin_command(cmd, real_args);
                    }
                    None => match execute_external(command, real_args) {
                        Ok((cmd_output, cmderr)) => {
                            output_err = cmderr;
                            output = cmd_output;
                        }
                        Err(_) => eprintln!("{command}: not found"),
                    },
                }

                handle_output(output, output_err, redirect);
            }

            Err(ReadlineError::Interrupted) => {
                continue;
            }

            Err(ReadlineError::Eof) => {
                break;
            }
            Err(e) => {
                eprintln!("error: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

use std::{fs::OpenOptions, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum RedirectType<'a> {
    None,
    Stdout(&'a [&'a str]),
    StdoutOne(&'a [&'a str]),
    StdErr(&'a [&'a str]),
    Append(&'a [&'a str]),
    AppendStdout(&'a [&'a str]),
    AppendStderr(&'a [&'a str]),
}

pub fn handle_output(output: String, output_err: String, redirect: RedirectType) {
    match redirect {
        RedirectType::None => {
            console_output(output);
            console_output(output_err);
        }
        RedirectType::Stdout(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            if output_err.is_empty() {
                file_output(output, path, false)
            } else {
                file_output(output_err, path, false)
            }
        }
        RedirectType::StdoutOne(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output, path, false);
            console_output(output_err);
        }
        RedirectType::StdErr(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output_err, path, false);
            console_output(output);
        }
        RedirectType::Append(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            if output_err.is_empty() {
                file_output(output, path, true)
            } else {
                file_output(output_err, path, true)
            }
        }
        RedirectType::AppendStderr(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output_err, path, true);
            console_output(output);
        }
        RedirectType::AppendStdout(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output, path, true);
            console_output(output_err);
        }
    }
}

/// Разделяет обычные аргументы и часть «редиректа».
pub fn separare_redirect_and_args<'a>(args: &'a [&'a str]) -> (&'a [&'a str], RedirectType<'a>) {
    let redirect_idx = args.iter().position(|s| s.contains('>'));

    match redirect_idx {
        None => (args, RedirectType::None),
        Some(idx) => {
            let real_args = &args[..idx];

            let rest = &args[idx + 1..];

            let chars_count = args[idx].len();

            match chars_count {
                1 => return (real_args, RedirectType::Stdout(rest)),
                2 => {
                    let redirect_str = args[idx].to_owned();
                    match redirect_str.chars().next().unwrap() {
                        '1' => return (real_args, RedirectType::StdoutOne(rest)),
                        '2' => return (real_args, RedirectType::StdErr(rest)),
                        '>' => return (real_args, RedirectType::Append(rest)),
                        _ => {}
                    }
                }
                3 => match args[idx] {
                    "1>>" => return (real_args, RedirectType::AppendStdout(rest)),
                    "2>>" => return (real_args, RedirectType::AppendStderr(rest)),
                    _ => {}
                },
                _ => {}
            }
            (real_args, RedirectType::Stdout(rest))
        }
    }
}

fn console_output(mut output: String) {
    if output.is_empty() {
        return;
    }

    // FIXME: Need to real handle quotes
    output = output.replace("'", "").replace('"', "");
    print!("{}", output);
    if !output.ends_with("\n") {
        println!();
    }
}

fn file_output(mut output: String, path: PathBuf, append: bool) {
    output = output.replace("'", "").replace('"', "");

    if !output.ends_with("\n") && !output.is_empty() {
        output.push('\n');
    }
    let mut options = OpenOptions::new();
    options.write(true).append(append);

    let mut file = match path.exists() {
        true => options.open(path).unwrap(),
        false => options.create_new(true).open(path).unwrap(),
    };

    let _ = file.write_all(&output.into_bytes());
}

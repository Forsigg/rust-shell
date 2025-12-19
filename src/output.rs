use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug)]
pub enum RedirectType<'a> {
    None,
    Stdout(&'a [&'a str]),
    StdoutOne(&'a [&'a str]),
    StdErr(&'a [&'a str]),
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
                file_output(output, path)
            } else {
                file_output(output_err, path)
            }
        }
        RedirectType::StdoutOne(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output, path);
            console_output(output_err);
        }
        RedirectType::StdErr(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output_err, path);
            console_output(output);
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

            if chars_count == 1 {
                return (real_args, RedirectType::Stdout(rest));
            } else if chars_count == 2 {
                let redirect_str = args[idx].to_owned();
                match redirect_str.chars().next().unwrap() {
                    '1' => return (real_args, RedirectType::StdoutOne(rest)),
                    '2' => return (real_args, RedirectType::StdErr(rest)),
                    _ => return (real_args, RedirectType::Stdout(rest)),
                }
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
    output = output.replace("'", "");
    print!("{}", output);
    if !output.ends_with("\n") {
        println!();
    }
}

fn file_output(output: String, path: PathBuf) {
    let mut file = match path.exists() {
        true => File::open(path).unwrap(),
        false => File::create_new(path).unwrap(),
    };

    let _ = file.write_all(&output.into_bytes());
}

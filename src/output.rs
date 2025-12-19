use std::{fs::File, io::Write, path::PathBuf};

pub enum RedirectType<'a> {
    None,
    Stdout(&'a [&'a str]),
    StdoutOne(&'a [&'a str]),
    StdErr(&'a [&'a str]),
}

pub fn handle_output(output: String, redirect: RedirectType) {
    match redirect {
        RedirectType::None => console_output(output),
        RedirectType::Stdout(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output, path)
        }
        RedirectType::StdoutOne(args) => {
            let path_arg = String::from(args[0]);
            let path = PathBuf::from(path_arg);
            file_output(output, path)
        }
        RedirectType::StdErr(_) => {}
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

            match args[idx - 1] {
                "1" => (real_args, RedirectType::StdoutOne(rest)),
                "2" => (real_args, RedirectType::StdErr(rest)),
                " " => (real_args, RedirectType::Stdout(rest)),
                &_ => (real_args, RedirectType::Stdout(rest)),
            }
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

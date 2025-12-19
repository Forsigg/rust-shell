use std::{fs::File, io::Write, path::PathBuf};


pub enum Output {
    Console,
    File(PathBuf),
}

pub fn handle_output(args: &[&str], output: String) {
    match match_output_type(args) {
        Output::Console => console_output(output),
        Output::File(path) => {file_output(output, path);}
    }
}

fn match_output_type(args: &[&str]) -> Output {
    
    match args.join(" ").split_once(">") {
        Some((_, rest)) => {
            Output::File(PathBuf::from(rest.trim()))
        },
        None => {Output::Console}
    }

}

fn console_output(output: String) {
    print!("{}", output);
    if !output.ends_with("\n") {
        println!();
    }
}

fn file_output(output: String, path: PathBuf) {
    let mut file = match path.exists() {
        true => File::open(path).unwrap(),
        false => File::create_new(path).unwrap()
    };
        

    let _ = file.write_all(&output.into_bytes());
}

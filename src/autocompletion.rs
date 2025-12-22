use rustyline::{completion::Completer, completion::Pair, Helper};
use std::{collections::HashMap, fs, os::unix::fs::PermissionsExt, path::PathBuf};

pub struct ShellHelper {
    pub completions: HashMap<String, String>,
}

impl ShellHelper {
    pub fn new() -> Self {
        let mut builtin_completions: HashMap<String, String> = HashMap::new();
        builtin_completions.insert("ech".into(), "echo ".into());
        builtin_completions.insert("exi".into(), "exit ".into());

        Self {
            completions: builtin_completions,
        }
    }

    pub fn add_completion(&mut self, k: &str, v: &str) {
        self.completions.insert(k.into(), v.into());
    }

    pub fn add_completions_from_path(&mut self, path: PathBuf) {
        if !path.exists() {
            return;
        }

        let paths = fs::read_dir(path).expect("Not exist path");

        for path_obj in paths {
            match path_obj {
                Ok(entry) => {
                    if entry.path().is_dir() {
                        continue;
                    }
                    let permissions = entry
                        .metadata()
                        .expect("Cant access to file from PATH")
                        .permissions();
                    let is_executable = permissions.mode() & 0o111 != 1;

                    if is_executable
                        && let Some(filename) = entry.path().file_name(){
                            let filename_str = filename.to_str().unwrap();

                            let completed = &format!("{} ", filename_str);

                            if filename.len() > 6 {
                                self.add_completion(&filename_str[..6], completed)
                            } else {
                                self.add_completion(&filename_str[..filename_str.len() / 2], completed)
                            }
                        }
                }
                Err(e) => eprintln!("entry error: {:?}", e),
            }
        }
    }
}

impl Default for ShellHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl Helper for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let prefix = &line[..pos];
        let mut matches = Vec::new();

        for (trigger, replacement) in &self.completions {
            if trigger.starts_with(prefix) {
                matches.push(Pair {
                    display: replacement.clone(),
                    replacement: replacement.clone(),
                });
            }
        }

        Ok((pos - prefix.len(), matches))
    }
}

impl rustyline::hint::Hinter for ShellHelper {
    type Hint = &'static str;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        // if let Ok((_, c)) = self.complete(line, pos, ctx) {
        //     if let Some(&e) = c.get(0) {
        //         return Some(&e[pos..]);
        //     }
        // }
        None
    }
}
impl rustyline::highlight::Highlighter for ShellHelper {}
impl rustyline::validate::Validator for ShellHelper {}

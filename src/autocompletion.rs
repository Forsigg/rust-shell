use rustyline::{
    completion::{Completer, Pair},
    line_buffer::LineBuffer,
    Changeset, Helper,
};
use std::{collections::HashMap, fs, os::unix::fs::PermissionsExt, path::PathBuf};

/// Helper for autocomplete
pub struct CompletionHelper {
    pub completions: HashMap<String, Vec<String>>,
}

/// Implement constructor, add completions
impl CompletionHelper {
    pub fn new() -> Self {
        let mut builtin_completions: HashMap<String, Vec<String>> = HashMap::new();
        builtin_completions.insert("ech".into(), vec!["echo ".into()]);
        builtin_completions.insert("exi".into(), vec!["exit ".into()]);

        Self {
            completions: builtin_completions,
        }
    }

    pub fn add_completion(&mut self, k: &str, v: &str) {
        match self.completions.get_mut(k) {
            Some(vector) => vector.push(v.into()),
            None => {
                self.completions.insert(k.into(), vec![v.into()]);
            }
        }
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

                        for i in 0..filename.len() {
                            if i < 3 {
                                continue;
                            }

                            self.add_completion(&filename_str[..i], completed);
                        }
                    }
                }
                Err(e) => eprintln!("entry error: {:?}", e),
            }
        }
    }
}

impl Default for CompletionHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl Helper for CompletionHelper {}

/// Implementation for autocomplete
impl Completer for CompletionHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let prefix = &line[..pos];
        let mut matches = Vec::new();

        for (trigger, replacements) in &self.completions {
            if trigger.starts_with(prefix) {
                for replacement in replacements {
                    matches.push(Pair {
                        display: replacement.clone().trim_end().to_string(),
                        replacement: replacement.clone(),
                    });
                }
            }
        }

        matches.sort_by_key(|p| p.display.clone());

        Ok((pos - prefix.len(), matches))
    }

    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str, cl: &mut Changeset) {
        let end = line.pos();
        line.replace(start..end, &elected, cl);
    }
}

impl rustyline::hint::Hinter for CompletionHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
        //     if line.is_empty() {
        //         return None;
        //     }
        //
        //     let (_, c) = self.complete(line, pos, ctx).ok()?;
        //
        //     if c.is_empty() {
        //         return None;
        //     }
        //
        //     let hint = c
        //         .iter()
        //         .map(|pair| pair.display.clone())
        //         .collect::<Vec<_>>()
        //         .join(" ");
        //
        //     Some(hint)
    }
}
impl rustyline::highlight::Highlighter for CompletionHelper {}
impl rustyline::validate::Validator for CompletionHelper {}

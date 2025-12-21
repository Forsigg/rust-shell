use rustyline::{Helper, completion::Completer};

pub struct ShellHelper {}

impl Helper for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = &'static str;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        match &line[..pos] {
            "ech" => Ok((0, vec!["echo "])),
            "exi" => Ok((0, vec!["exit "])),
            _ => Ok((0, vec![])),
        }
    }
}
impl rustyline::hint::Hinter for ShellHelper {
    type Hint = &'static str;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        if let Ok((_, c)) = self.complete(line, pos, ctx) {
            if let Some(&e) = c.get(0) {
                return Some(&e[pos..]);
            }
        }
        None
    }
}
impl rustyline::highlight::Highlighter for ShellHelper {}
impl rustyline::validate::Validator for ShellHelper {}

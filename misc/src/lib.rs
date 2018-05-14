extern crate aho_corasick;

/// IO sink that uses the `print` macro, for test stdout capture.
pub struct PrintWriter;
impl std::io::Write for PrintWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        print!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { std::io::stdout().flush() }
}

/// IO sink that uses the `eprint` macro, for test stdout capture.
pub struct EPrintWriter;
impl std::io::Write for EPrintWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        eprint!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { std::io::stdout().flush() }
}

/// Replace many nonoverlapping substrings with replacements at once.
pub trait ReplaceMany {
    /// Replace each occurrence of a key with the corresponding value in one pass.
    fn replace_many(&self, replacements: &[(&str, &str)]) -> String;
}
impl ReplaceMany for str {
    fn replace_many(&self, replacements: &[(&str, &str)]) -> String {
        use aho_corasick::{AcAutomaton, Automaton};
        let aut = AcAutomaton::new(replacements.iter().map(|it| it.0));
        let mut s = String::with_capacity(self.len());
        let mut last_hit_end = 0;
        for hit in aut.find(self) {
            s.push_str(&self[last_hit_end..hit.start]);
            s.push_str(replacements[hit.pati].1);
            last_hit_end = hit.end;
        }
        s.push_str(&self[last_hit_end..]);
        s.shrink_to_fit();
        s
    }
}

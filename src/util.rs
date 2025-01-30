pub struct IndentedWriter<W: std::fmt::Write> {
    pub inner: W,
    pub indentation_level: usize,
    at_start_of_newline: bool,
}

impl<W: std::fmt::Write> IndentedWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            inner: writer,
            indentation_level: 0,
            at_start_of_newline: true,
        }
    }

    pub fn push_indent(&mut self) {
        self.indentation_level += 1;
    }

    pub fn pop_indent(&mut self) {
        self.indentation_level = self.indentation_level.saturating_sub(1);
    }
}

impl<W: std::fmt::Write> std::fmt::Write for IndentedWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            if self.at_start_of_newline {
                self.inner
                    .write_str(&"    ".repeat(self.indentation_level))?;
                self.at_start_of_newline = false;
            }

            self.inner.write_char(c)?;

            if c == '\n' {
                self.at_start_of_newline = true;
            }
        }

        Ok(())
    }
}

impl<W: std::fmt::Write> Drop for IndentedWriter<W> {
    fn drop(&mut self) {
        if self.indentation_level != 0 {
            eprintln!("Detected {} unclosed indents", self.indentation_level);
        }
    }
}

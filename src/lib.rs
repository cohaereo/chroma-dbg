mod config;
mod util;

pub use config::{ChromaConfig, Color};

use core::fmt;
use std::fmt::{Debug, Write};

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use util::IndentedWriter;

#[derive(Parser)]
#[grammar = "dbg.pest"]
struct DbgParser;

pub trait ChromaDebug: Debug {
    fn dbg_chroma(&self) -> String;
}

impl<T: Debug> ChromaDebug for T {
    fn dbg_chroma(&self) -> String {
        ChromaConfig::DEFAULT.format(self)
    }
}

impl ChromaConfig {
    pub fn format(&self, value: &impl Debug) -> String {
        self.try_format(value)
            .unwrap_or_else(|_| format!("{:#?}", value))
    }

    pub fn try_format(&self, value: &impl Debug) -> Result<String, pest::error::Error<Rule>> {
        let original = format!("{:#?}", value);
        self.try_format_string(&original)
    }

    pub fn try_format_string(&self, value: &str) -> Result<String, pest::error::Error<Rule>> {
        let pairs = DbgParser::parse(Rule::main, value)?;
        let mut output = String::new();
        let mut writer = IndentedWriter::new(&mut output);
        for pair in pairs {
            self.emit_value(&mut writer, pair);
        }
        drop(writer);
        Ok(output)
    }

    fn emit_value<W: fmt::Write>(&self, w: &mut IndentedWriter<W>, pair: Pair<'_, Rule>) {
        match pair.as_rule() {
            Rule::r#struct => {
                let mut pairs = pair.into_inner();
                let name = pairs.next().unwrap().as_str();
                Self::emit_colored(w, name, self.identifier_color);
                Self::emit_plain(w, " { ");
                let inline = self.inline_struct.should_inline(pairs.as_str().len());
                if !inline {
                    Self::emit_plain(w, "\n");
                }
                let fields = pairs.next().unwrap().into_inner();
                let field_count = fields.len();
                for (i, field) in fields.enumerate() {
                    let mut field = field.into_inner();
                    let name = field.next().unwrap().as_str();
                    if !inline {
                        w.push_indent();
                    }
                    Self::emit_colored(w, name, self.field_color);
                    Self::emit_plain(w, ": ");
                    self.emit_value(w, field.next().unwrap());
                    if i < field_count - 1 || !inline {
                        Self::emit_plain(w, ", ");
                    } else {
                        Self::emit_plain(w, " ");
                    }

                    if !inline {
                        Self::emit_plain(w, "\n");
                        w.pop_indent();
                    }
                }
                Self::emit_plain(w, "}");
            }
            Rule::tuple_struct => {
                let mut pairs = pair.into_inner();
                let name = pairs.next().unwrap().as_str();
                Self::emit_colored(w, name, self.identifier_color);
                Self::emit_plain(w, "(");
                let fields = pairs.next().unwrap().into_inner();
                let field_count = fields.len();
                for (i, field) in fields.enumerate() {
                    self.emit_value(w, field);
                    if i < field_count - 1 {
                        Self::emit_plain(w, ", ");
                    }
                }
                Self::emit_plain(w, ")");
            }
            Rule::number => {
                let num = pair.into_inner().next().unwrap();
                match num.as_rule() {
                    Rule::integer => {
                        // Try to parse the number as an integer. If it fails (eg. it's too large/unsigned), just print it normally
                        if let Ok(num) = num.as_str().parse::<u64>() {
                            Self::emit_colored(
                                w,
                                self.integer_format.format(num).as_str(),
                                self.numerical_color,
                            );
                        } else {
                            Self::emit_colored(w, num.as_str(), self.numerical_color);
                        }
                    }
                    // If it's not a (decimal) integer, print it normally
                    _ => {
                        Self::emit_colored(w, num.as_str(), self.numerical_color);
                    }
                }
            }
            Rule::string => {
                Self::emit_colored(w, pair.as_str(), self.string_color);
            }
            Rule::enum_variant => {
                Self::emit_colored(w, pair.as_str(), self.identifier_color);
            }
            Rule::array => {
                Self::emit_plain(w, "[");
                let inline = self.inline_array.should_inline(pair.as_str().len());
                let elements = pair.into_inner();
                let element_count = elements.len();
                for (i, field) in elements.enumerate() {
                    if i == 0 {
                        if inline {
                            Self::emit_plain(w, " ");
                        } else {
                            Self::emit_plain(w, "\n");
                        }
                    }
                    if !inline {
                        w.push_indent();
                    }
                    self.emit_value(w, field);
                    if i < element_count - 1 || !inline {
                        Self::emit_plain(w, ", ");
                    } else {
                        Self::emit_plain(w, " ");
                    }

                    if !inline {
                        Self::emit_plain(w, "\n");
                        w.pop_indent();
                    }
                }
                Self::emit_plain(w, "]");
            }
            Rule::boolean => {
                Self::emit_colored(w, pair.as_str(), self.numerical_color);
            }
            _ => {
                let value = pair.as_str();
                Self::emit_plain(w, value);
            }
        }
    }

    fn emit_plain<W: fmt::Write>(w: &mut IndentedWriter<W>, s: &str) {
        w.write_str(s).ok();
    }

    fn emit_colored<W: fmt::Write>(w: &mut IndentedWriter<W>, s: &str, color: Color) {
        let style = anstyle::Style::new().fg_color(Some(anstyle::Color::Rgb(color.into())));
        let reset = anstyle::Reset;
        write!(w, "{style}{s}{reset}").ok();
    }
}

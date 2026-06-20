mod config;
mod util;

#[cfg(test)]
mod tests;

pub use config::{ChromaConfig, Color, InlineThreshold, IntegerFormat};

use std::fmt::{Debug, Write};
use util::IndentedWriter;

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
        let original = match self.integer_format {
            IntegerFormat::Decimal => format!("{:?}", value),
            IntegerFormat::Hex => format!("{:#X?}", value),
        };
        self.format_string(&original)
    }

    pub fn format_string(&self, value: &str) -> String {
        let tokens = lex(value);
        let mut tokens_iter = tokens.into_iter().peekable();
        let nodes = parse_dom(&mut tokens_iter, None);

        let mut output = String::new();
        let mut writer = IndentedWriter::new(&mut output);
        let mut formatter = Formatter {
            config: self,
            out: &mut writer,
        };

        formatter.format_nodes(&nodes);
        drop(writer);
        output
    }
}

#[derive(Debug, Clone)]
enum Token {
    Ident(String),
    String(String),
    Char(String),
    Number(String),
    Bool(bool),
    Punct(char),
    Whitespace(String),
    Other(String),
}

impl Token {
    fn plain_len(&self) -> usize {
        match self {
            Token::Ident(s)
            | Token::String(s)
            | Token::Char(s)
            | Token::Number(s)
            | Token::Whitespace(s)
            | Token::Other(s) => s.len(),
            Token::Bool(b) => {
                if *b {
                    4
                } else {
                    5
                }
            }
            Token::Punct(c) => c.len_utf8(),
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    Token(Token),
    Group {
        open: char,
        close: char,
        children: Vec<Node>,
    },
}

impl Node {
    fn plain_len(&self) -> usize {
        match self {
            Node::Token(t) => t.plain_len(),
            Node::Group {
                open,
                close,
                children,
            } => {
                open.len_utf8()
                    + close.len_utf8()
                    + children.iter().map(|n| n.plain_len()).sum::<usize>()
            }
        }
    }
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some(&(_i, c)) = chars.peek() {
        match c {
            '{' | '}' | '[' | ']' | '(' | ')' | ':' | ',' | '=' | '|' => {
                tokens.push(Token::Punct(c));
                chars.next();
            }
            c if c.is_whitespace() => {
                // let mut s = String::new();
                while let Some(&(_, ws)) = chars.peek() {
                    if ws.is_whitespace() {
                        // s.push(ws);
                        chars.next();
                    } else {
                        break;
                    }
                }
                // tokens.push(Token::Whitespace(s));
                tokens.push(Token::Whitespace(" ".to_string()));
            }
            '"' | '\'' => {
                let quote = c;
                let mut s = String::new();
                s.push(quote);
                chars.next();
                let mut escaped = false;
                while let Some(&(_, next_c)) = chars.peek() {
                    s.push(next_c);
                    chars.next();
                    if escaped {
                        escaped = false;
                    } else if next_c == '\\' {
                        escaped = true;
                    } else if next_c == quote {
                        break;
                    }
                }
                if quote == '"' {
                    tokens.push(Token::String(s));
                } else {
                    tokens.push(Token::Char(s));
                }
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                while let Some(&(_, next_c)) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        s.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if s == "true" || s == "false" {
                    tokens.push(Token::Bool(s == "true"));
                } else {
                    tokens.push(Token::Ident(s));
                }
            }
            c if c.is_ascii_digit() || c == '-' || c == '+' => {
                let mut s = String::new();
                s.push(c);
                chars.next();
                while let Some(&(_, next_c)) = chars.peek() {
                    if next_c.is_ascii_digit()
                        || next_c.is_ascii_hexdigit()
                        || ".xX_eE-+".contains(next_c)
                    {
                        s.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if s == "-" || s == "+" {
                    tokens.push(Token::Punct(s.chars().next().unwrap()));
                } else {
                    tokens.push(Token::Number(s));
                }
            }
            _ => {
                let mut s = String::new();
                s.push(c);
                chars.next();
                tokens.push(Token::Other(s));
            }
        }
    }
    tokens
}

fn parse_dom(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    stop_at: Option<char>,
) -> Vec<Node> {
    let mut nodes = Vec::new();
    while let Some(tok) = tokens.next() {
        if let Token::Punct(c) = tok {
            if Some(c) == stop_at {
                return nodes;
            }
            if c == '}' || c == ']' || c == ')' {
                nodes.push(Node::Token(Token::Punct(c)));
                continue;
            }
            if c == '{' || c == '[' || c == '(' {
                let close_char = match c {
                    '{' => '}',
                    '[' => ']',
                    '(' => ')',
                    _ => unreachable!(),
                };
                let children = parse_dom(tokens, Some(close_char));
                nodes.push(Node::Group {
                    open: c,
                    close: close_char,
                    children,
                });
                continue;
            }
        }
        nodes.push(Node::Token(tok));
    }
    nodes
}

struct Formatter<'a, W: Write> {
    config: &'a ChromaConfig,
    out: &'a mut IndentedWriter<W>,
}

impl<'a, W: Write> Formatter<'a, W> {
    fn emit_plain(&mut self, s: &str) {
        self.out.write_str(s).ok();
    }

    fn emit_colored(&mut self, s: &str, color: Color) {
        let ansi = format!("\x1b[38;2;{};{};{}m", color.0, color.1, color.2);
        self.out.write_str(&ansi).ok();
        self.out.write_str(s).ok();
        self.out.write_str("\x1b[0m").ok();
    }

    fn format_nodes(&mut self, nodes: &[Node]) {
        for (i, node) in nodes.iter().enumerate() {
            match node {
                Node::Token(tok) => self.format_token(tok, nodes, i),
                Node::Group {
                    open,
                    close,
                    children,
                } => {
                    let inline = self.should_inline(*open, children);
                    self.format_group(*open, *close, children, inline);
                }
            }
        }
    }

    fn format_token(&mut self, tok: &Token, siblings: &[Node], index: usize) {
        match tok {
            Token::Ident(s) => {
                if is_followed_by_colon(siblings, index) {
                    self.emit_colored(s, self.config.field_color);
                } else {
                    self.emit_colored(s, self.config.identifier_color);
                }
            }
            Token::Number(s) => {
                self.emit_colored(s, self.config.numerical_color);
            }
            Token::Bool(b) => {
                self.emit_colored(
                    if *b { "true" } else { "false" },
                    self.config.numerical_color,
                );
            }
            Token::String(s) | Token::Char(s) => self.emit_escaped_string(s),
            Token::Punct(c) => self.emit_plain(&c.to_string()),
            Token::Whitespace(s) | Token::Other(s) => self.emit_plain(s),
        }
    }

    fn format_group(&mut self, open: char, close: char, children: &[Node], inline: bool) {
        self.emit_plain(&open.to_string());

        if !inline {
            self.out.push_indent();
            self.emit_plain("\n");
            // } else if open == '{' && !is_all_whitespace(children) {
            //     self.emit_plain(" ");
        }

        let mut i = 0;
        let mut just_emitted_comma = false;

        while i < children.len() {
            let child = &children[i];

            if !inline {
                if let Node::Token(Token::Whitespace(ws)) = child {
                    if i == 0 || just_emitted_comma || ws.contains('\n') {
                        i += 1;
                        continue;
                    }
                }
                if is_all_whitespace(&children[i..]) {
                    break;
                }
            }

            match child {
                Node::Token(tok) => self.format_token(tok, children, i),
                Node::Group {
                    open: inner_o,
                    close: inner_c,
                    children: inner,
                } => {
                    let child_inline = self.should_inline(*inner_o, inner);
                    self.format_group(*inner_o, *inner_c, inner, child_inline);
                }
            }

            just_emitted_comma = matches!(child, Node::Token(Token::Punct(',')));
            if !inline && just_emitted_comma {
                self.emit_plain("\n");
            }

            i += 1;
        }

        if !inline {
            self.out.pop_indent();
            self.emit_plain("\n");
            // } else if open == '{' && !children.is_empty() && !is_all_whitespace(children) {
            //     self.emit_plain(" ");
        }

        self.emit_plain(&close.to_string());
    }

    fn emit_escaped_string(&mut self, s: &str) {
        if s.len() < 2 {
            self.emit_colored(s, self.config.string_color);
            return;
        }
        self.emit_colored(&s[0..1], self.config.string_color); // opening quote
        let inner = &s[1..s.len() - 1];
        let mut chars = inner.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            if c == '\\' {
                let start = i;
                if let Some(&(_, next)) = chars.peek() {
                    match next {
                        'n' | 'r' | 't' | '\\' | '0' | '\'' | '"' => {
                            chars.next();
                            self.emit_colored(
                                &inner[start..start + 2],
                                self.config.string_escape_color,
                            );
                        }
                        'x' => {
                            chars.next();
                            let mut len = 2;
                            for _ in 0..2 {
                                if chars.next().is_some() {
                                    len += 1;
                                }
                            }
                            self.emit_colored(
                                &inner[start..start + len],
                                self.config.string_escape_color,
                            );
                        }
                        'u' => {
                            chars.next();
                            let mut len = 2;
                            if let Some(&(_, '{')) = chars.peek() {
                                chars.next();
                                len += 1;
                                while let Some(&(_, hc)) = chars.peek() {
                                    chars.next();
                                    len += hc.len_utf8();
                                    if hc == '}' {
                                        break;
                                    }
                                }
                            }
                            self.emit_colored(
                                &inner[start..start + len],
                                self.config.string_escape_color,
                            );
                        }
                        _ => self.emit_colored("\\", self.config.string_escape_color),
                    }
                } else {
                    self.emit_colored("\\", self.config.string_escape_color);
                }
            } else {
                let mut chunk = String::new();
                chunk.push(c);
                while let Some(&(_, next)) = chars.peek() {
                    if next == '\\' {
                        break;
                    }
                    chunk.push(next);
                    chars.next();
                }
                self.emit_colored(&chunk, self.config.string_color);
            }
        }
        self.emit_colored(&s[s.len() - 1..], self.config.string_color); // closing quote
    }

    fn should_inline(&self, open: char, children: &[Node]) -> bool {
        let len = children.iter().map(|n| n.plain_len()).sum::<usize>();
        match open {
            '{' => self.config.inline_struct.should_inline(len),
            '[' => self.config.inline_array.should_inline(len),
            _ => true, // Tuples usually default to inline
        }
    }
}

fn is_followed_by_colon(nodes: &[Node], mut index: usize) -> bool {
    index += 1;
    while index < nodes.len() {
        match &nodes[index] {
            Node::Token(Token::Whitespace(_)) => index += 1,
            Node::Token(Token::Punct(':')) => return true,
            _ => return false,
        }
    }
    false
}

fn is_all_whitespace(nodes: &[Node]) -> bool {
    nodes
        .iter()
        .all(|n| matches!(n, Node::Token(Token::Whitespace(_))))
}

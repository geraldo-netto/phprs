mod numbers;
mod strings;
mod words;

use super::{error, reserve, token_ids::*};
use crate::contracts::{Diagnostic, ErrorKind, Limits, ParseOptions};
use std::ops::Range;

#[derive(Clone, Debug)]
pub(super) struct Token {
    pub id: u16,
    pub span: Range<usize>,
}

#[derive(Clone, Copy, Debug)]
enum Text {
    Quote(u8),
    Heredoc { id: usize, nowdoc: bool },
}

impl Text {
    fn nowdoc(self) -> bool {
        matches!(self, Self::Heredoc { nowdoc: true, .. })
    }
    fn context(self) -> Option<usize> {
        match self {
            Self::Quote(_) => None,
            Self::Heredoc { id, .. } => Some(id),
        }
    }
    fn closing_token(self) -> u16 {
        match self {
            Self::Quote(byte) => punctuation(byte),
            Self::Heredoc { .. } => T_END_HEREDOC,
        }
    }
}

struct Heredoc {
    label: Range<usize>,
    body_start: usize,
    segments: Vec<Range<usize>>,
}

#[derive(Clone, Copy, Debug)]
enum Mode {
    Html,
    Code,
    Property,
    Text(Text),
    Tail(Text),
    TextProperty(Text),
    Offset(Text),
    Varname,
}

pub(super) struct Lexer<'source> {
    source: &'source [u8],
    pub position: usize,
    mode: Mode,
    frames: Vec<Mode>,
    heredocs: Vec<Heredoc>,
    limits: Limits,
    count: usize,
    short_open_tags: bool,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source [u8], options: ParseOptions) -> Self {
        let mut position = 0;
        if source.starts_with(b"#!") {
            position = source
                .iter()
                .position(|&byte| byte == b'\n')
                .map_or(source.len(), |p| p + 1);
        }
        Self {
            source,
            position,
            mode: Mode::Html,
            frames: Vec::new(),
            heredocs: Vec::new(),
            limits: options.limits,
            count: 0,
            short_open_tags: options.short_open_tags,
        }
    }

    pub fn next(&mut self) -> Result<Token, Diagnostic> {
        loop {
            if self.position == self.source.len() {
                return self.end();
            }
            let start = self.position;
            if let Some(id) = self.step()? {
                return self.token(id, start);
            }
        }
    }

    fn step(&mut self) -> Result<Option<u16>, Diagnostic> {
        match self.mode {
            Mode::Html => Ok(self.html()),
            Mode::Code => self.code(),
            Mode::Property => Ok(self.property()),
            Mode::Text(text) => self.text(text).map(Some),
            Mode::Tail(text) => Ok(self.tail(text)),
            Mode::TextProperty(text) => Ok(Some(self.text_property(text))),
            Mode::Offset(text) => self.offset(text).map(Some),
            Mode::Varname => Ok(self.varname()),
        }
    }

    fn end(&self) -> Result<Token, Diagnostic> {
        if !self.frames.is_empty() {
            return Err(error(
                ErrorKind::Lexical,
                self.position,
                "unclosed interpolation or brace",
            ));
        }
        match self.mode {
            Mode::Text(_) | Mode::Offset(_) | Mode::Tail(_) | Mode::TextProperty(_) => Err(error(
                ErrorKind::Lexical,
                self.position,
                "unterminated string",
            )),
            _ => Ok(Token {
                id: 0,
                span: self.position..self.position,
            }),
        }
    }

    fn token(&mut self, id: u16, start: usize) -> Result<Token, Diagnostic> {
        if self.count >= self.limits.tokens {
            return Err(error(ErrorKind::Limit, start, "token limit exceeded"));
        }
        self.count += 1;
        Ok(Token {
            id,
            span: start..self.position,
        })
    }

    fn push(&mut self, mode: Mode) -> Result<(), Diagnostic> {
        if self.frames.len() >= self.limits.stack {
            return Err(error(
                ErrorKind::Limit,
                self.position,
                "lexer stack limit exceeded",
            ));
        }
        reserve(&mut self.frames, 1)?;
        self.frames.push(mode);
        Ok(())
    }

    pub fn halt(&mut self) {
        self.position = self.source.len();
        self.frames.clear();
        self.heredocs.clear();
        self.mode = Mode::Code;
    }

    fn rest(&self) -> &'source [u8] {
        &self.source[self.position..]
    }

    fn html(&mut self) -> Option<u16> {
        if self.rest().starts_with(b"<?=") {
            self.position += 3;
            self.mode = Mode::Code;
            return Some(T_ECHO);
        }
        if let Some(length) = self.open_tag() {
            self.position += length;
            self.mode = Mode::Code;
            return None;
        }
        self.position += 1;
        while self.position < self.source.len() {
            if self.open_tag().is_some() || self.rest().starts_with(b"<?=") {
                break;
            }
            self.position += 1;
        }
        Some(T_INLINE_HTML)
    }

    fn open_tag(&self) -> Option<usize> {
        if self.short_open_tags && self.rest().starts_with(b"<?") {
            return Some(self.long_tag_length().unwrap_or(2));
        }
        self.long_tag_length()
    }

    fn long_tag_length(&self) -> Option<usize> {
        if self.rest().len() < 5 {
            return None;
        }
        if !self.rest()[..5].eq_ignore_ascii_case(b"<?php") {
            return None;
        }
        self.rest()
            .get(5)
            .is_none_or(|byte| whitespace(*byte))
            .then_some(5)
    }

    fn code(&mut self) -> Result<Option<u16>, Diagnostic> {
        if self.skip_trivia()? {
            return Ok(None);
        }
        if self.rest().starts_with(b"?>") {
            self.position += 2;
            self.skip_newline();
            self.mode = Mode::Html;
            return Ok(Some(punctuation(b';')));
        }
        self.code_token().map(Some)
    }

    fn code_token(&mut self) -> Result<u16, Diagnostic> {
        let byte = self.source[self.position];
        if byte == b'\'' {
            return self.quoted(byte);
        }
        if byte == b'"' || byte == b'`' {
            return self.start_text(byte);
        }
        if self.binary_prefix() {
            return self.binary_string();
        }
        if self.rest().starts_with(b"<<<") {
            return self.heredoc();
        }
        if byte.is_ascii_digit() {
            return self.number();
        }
        if self.fraction_start() {
            return self.number();
        }
        self.symbol_or_word()
    }

    fn symbol_or_word(&mut self) -> Result<u16, Diagnostic> {
        let byte = self.source[self.position];
        if byte == b'$' && self.next_label() {
            return Ok(self.variable());
        }
        if label_start(byte) || byte == b'\\' {
            let id = self.word();
            return self.contextual_word(id);
        }
        if byte == b'('
            && let Some(id) = self.cast()?
        {
            return Ok(id);
        }
        if byte == b'&' {
            return self.ampersand();
        }
        self.operator()
    }

    fn operator(&mut self) -> Result<u16, Diagnostic> {
        if self.rest().starts_with(b"<>") {
            self.position += 2;
            return Ok(T_IS_NOT_EQUAL);
        }
        for &(text, id) in OPERATORS {
            if self.rest().starts_with(text) {
                self.position += text.len();
                self.operator_mode(id);
                return Ok(id);
            }
        }
        let byte = self.source[self.position];
        self.position += 1;
        self.brace(byte)?;
        let id = punctuation(byte);
        if id == T_BAD_CHARACTER {
            return Err(error(
                ErrorKind::Lexical,
                self.position - 1,
                "invalid PHP byte",
            ));
        }
        Ok(id)
    }

    fn operator_mode(&mut self, id: u16) {
        if id == T_OBJECT_OPERATOR || id == T_NULLSAFE_OBJECT_OPERATOR {
            self.mode = Mode::Property;
        }
    }

    fn brace(&mut self, byte: u8) -> Result<(), Diagnostic> {
        if byte == b'{' {
            self.push(Mode::Code)?;
        }
        if byte == b'}' {
            self.mode = self.frames.pop().unwrap_or(Mode::Code);
        }
        Ok(())
    }

    fn skip_trivia(&mut self) -> Result<bool, Diagnostic> {
        if whitespace(self.source[self.position]) {
            while self.position < self.source.len() && whitespace(self.source[self.position]) {
                self.position += 1;
            }
            return Ok(true);
        }
        if self.rest().starts_with(b"/*") {
            self.block_comment()?;
            return Ok(true);
        }
        if self.line_comment_start() {
            self.line_comment();
            return Ok(true);
        }
        Ok(false)
    }

    fn line_comment_start(&self) -> bool {
        if self.rest().starts_with(b"//") {
            return true;
        }
        self.rest().starts_with(b"#") && !self.rest().starts_with(b"#[")
    }

    fn line_comment(&mut self) {
        while self.position < self.source.len() {
            if self.rest().starts_with(b"?>") || matches!(self.source[self.position], b'\n' | b'\r')
            {
                break;
            }
            self.position += 1;
        }
    }

    fn block_comment(&mut self) -> Result<(), Diagnostic> {
        let start = self.position;
        self.position += 2;
        while self.position < self.source.len() {
            if self.rest().starts_with(b"*/") {
                self.position += 2;
                return Ok(());
            }
            self.position += 1;
        }
        Err(error(
            ErrorKind::Lexical,
            start,
            "unterminated block comment",
        ))
    }

    fn skip_newline(&mut self) {
        if self.rest().starts_with(b"\r\n") {
            self.position += 2;
        } else if matches!(self.rest().first(), Some(b'\r' | b'\n')) {
            self.position += 1;
        }
    }

    fn next_label(&self) -> bool {
        self.rest().get(1).is_some_and(|&byte| label_start(byte))
    }
    fn fraction_start(&self) -> bool {
        self.rest().first() == Some(&b'.') && self.rest().get(1).is_some_and(u8::is_ascii_digit)
    }
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\r' | b'\n')
}
fn label_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte >= 0x80
}
fn label_continue(byte: u8) -> bool {
    label_start(byte) || byte.is_ascii_digit()
}

fn punctuation(byte: u8) -> u16 {
    PUNCTUATION
        .iter()
        .find(|&&(text, _)| text == byte)
        .map_or(T_BAD_CHARACTER, |&(_, id)| id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::DEFAULT_OPTIONS;

    #[test]
    fn interpolated_tokens_phprs_016() {
        let source = br#"<?php echo "{$a['key']} ${name} ${name[0]} ${$name}";"#;
        let mut lexer = Lexer::new(source, DEFAULT_OPTIONS);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next().unwrap();
            let id = token.id;
            tokens.push((id, token.span));
            if id == 0 {
                break;
            }
        }
        assert!(crate::parse(source, DEFAULT_OPTIONS).is_ok(), "{tokens:?}");
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    use crate::contracts::{DEFAULT_LIMITS, DEFAULT_OPTIONS};

    #[test]
    fn lexical_frame_limit_phprs_002() {
        let limits = Limits {
            stack: 1,
            ..DEFAULT_LIMITS
        };
        let mut lexer = Lexer::new(
            b"<?php {{",
            ParseOptions {
                limits,
                ..DEFAULT_OPTIONS
            },
        );
        assert_eq!(lexer.next().unwrap().id, punctuation(b'{'));
        assert_eq!(lexer.next().unwrap_err().kind, ErrorKind::Limit);
    }
}

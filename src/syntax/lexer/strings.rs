use super::*;

impl Lexer<'_> {
    pub(super) fn quoted(&mut self, delimiter: u8) -> Result<u16, Diagnostic> {
        let start = self.position;
        self.position += 1;
        while self.position < self.source.len() {
            let byte = self.source[self.position];
            self.position += 1;
            if byte == delimiter {
                return Ok(T_CONSTANT_ENCAPSED_STRING);
            }
            if byte == b'\\' {
                self.position += usize::from(self.position < self.source.len());
            }
        }
        Err(error(
            ErrorKind::Lexical,
            start,
            "unterminated quoted string",
        ))
    }

    pub(super) fn start_text(&mut self, delimiter: u8) -> Result<u16, Diagnostic> {
        if delimiter == b'"'
            && let Some(end) = self.simple_string_end()?
        {
            self.position = end;
            return Ok(T_CONSTANT_ENCAPSED_STRING);
        }
        self.position += 1;
        self.mode = Mode::Text(Text::Quote(delimiter));
        Ok(punctuation(delimiter))
    }

    fn simple_string_end(&self) -> Result<Option<usize>, Diagnostic> {
        let mut position = self.position + 1;
        while let Some(&byte) = self.source.get(position) {
            if byte == b'"' {
                return Ok(Some(position + 1));
            }
            if self.interpolation_at(position) {
                return Ok(None);
            }
            if byte == b'\\' {
                position = escape_end(self.source, position)?;
            } else {
                position += 1;
            }
        }
        Err(error(
            ErrorKind::Lexical,
            self.position,
            "unterminated double-quoted string",
        ))
    }

    pub(super) fn binary_prefix(&self) -> bool {
        if !matches!(self.rest().first(), Some(b'b' | b'B')) {
            return false;
        }
        if matches!(self.rest().get(1), Some(b'\'' | b'"')) {
            return true;
        }
        self.rest().get(1..4) == Some(b"<<<")
    }

    pub(super) fn binary_string(&mut self) -> Result<u16, Diagnostic> {
        self.position += 1;
        match self.source[self.position] {
            b'\'' => self.quoted(b'\''),
            b'"' => self.start_text(b'"'),
            _ => self.heredoc(),
        }
    }

    fn interpolation_at(&self, at: usize) -> bool {
        let bytes = &self.source[at..];
        if bytes.starts_with(b"{$") || bytes.starts_with(b"${") {
            return true;
        }
        bytes.first() == Some(&b'$') && bytes.get(1).is_some_and(|&byte| label_start(byte))
    }

    pub(super) fn text(&mut self, text: Text) -> Result<u16, Diagnostic> {
        if let Some(close) = self.text_close(text) {
            if let Some(id) = text.context() {
                self.finish_heredoc(id)?;
            }
            self.position = close;
            self.mode = Mode::Code;
            return Ok(text.closing_token());
        }
        if !text.nowdoc() && self.interpolation_at(self.position) {
            return self.interpolation(text);
        }
        let start = self.position;
        self.scan_text(text)?;
        if let Some(id) = text.context() {
            reserve(&mut self.heredocs[id].segments, 1)?;
            self.heredocs[id].segments.push(start..self.position);
        }
        Ok(T_ENCAPSED_AND_WHITESPACE)
    }

    fn text_close(&self, text: Text) -> Option<usize> {
        match text {
            Text::Quote(byte) => (self.source[self.position] == byte).then_some(self.position + 1),
            Text::Heredoc { id, .. } => self.heredoc_close(id),
        }
    }

    fn heredoc_close(&self, id: usize) -> Option<usize> {
        if !line_start(self.source, self.position) {
            return None;
        }
        let context = &self.heredocs[id];
        let start = skip_indentation(self.source, self.position);
        if !self.source[start..].starts_with(&self.source[context.label.clone()]) {
            return None;
        }
        let end = start + context.label.len();
        if self
            .source
            .get(end)
            .is_some_and(|&byte| label_continue(byte))
        {
            return None;
        }
        Some(end)
    }

    fn scan_text(&mut self, text: Text) -> Result<(), Diagnostic> {
        while self.position < self.source.len() {
            if self.text_close(text).is_some() {
                break;
            }
            if !text.nowdoc() && self.interpolation_at(self.position) {
                break;
            }
            if !text.nowdoc() && self.source[self.position] == b'\\' {
                self.position = escape_end(self.source, self.position)?;
            } else {
                self.position += 1;
            }
        }
        Ok(())
    }

    fn interpolation(&mut self, text: Text) -> Result<u16, Diagnostic> {
        if self.rest().starts_with(b"{$") {
            self.position += 1;
            self.push(Mode::Text(text))?;
            self.mode = Mode::Code;
            return Ok(T_CURLY_OPEN);
        }
        if self.rest().starts_with(b"${") {
            self.position += 2;
            self.push(Mode::Text(text))?;
            self.mode = Mode::Varname;
            return Ok(T_DOLLAR_OPEN_CURLY_BRACES);
        }
        self.mode = Mode::Tail(text);
        Ok(self.variable())
    }

    pub(super) fn tail(&mut self, text: Text) -> Option<u16> {
        self.mode = Mode::Text(text);
        if self.rest().starts_with(b"[") {
            self.position += 1;
            self.mode = Mode::Offset(text);
            return Some(punctuation(b'['));
        }
        let length = if self.rest().starts_with(b"?->") {
            3
        } else {
            2
        };
        if !self.property_tail(length) {
            return None;
        }
        self.position += length;
        self.mode = Mode::TextProperty(text);
        Some(if length == 3 {
            T_NULLSAFE_OBJECT_OPERATOR
        } else {
            T_OBJECT_OPERATOR
        })
    }

    fn property_tail(&self, length: usize) -> bool {
        if !self.rest().starts_with(b"->") && !self.rest().starts_with(b"?->") {
            return false;
        }
        self.rest()
            .get(length)
            .is_some_and(|&byte| label_start(byte))
    }

    pub(super) fn text_property(&mut self, text: Text) -> u16 {
        self.position = self.source[self.position..]
            .iter()
            .position(|&byte| !label_continue(byte))
            .map_or(self.source.len(), |length| self.position + length);
        self.mode = Mode::Text(text);
        T_STRING
    }

    pub(super) fn offset(&mut self, text: Text) -> Result<u16, Diagnostic> {
        let byte = self.source[self.position];
        if byte == b']' {
            self.mode = Mode::Text(text);
            self.position += 1;
            return Ok(punctuation(byte));
        }
        if byte == b'$' && self.next_label() {
            return Ok(self.variable());
        }
        if label_start(byte) {
            return Ok(self.offset_word());
        }
        if byte.is_ascii_digit() {
            return Ok(self.offset_number());
        }
        if byte == b'-' {
            self.position += 1;
            return Ok(punctuation(byte));
        }
        Err(error(
            ErrorKind::Lexical,
            self.position,
            "invalid interpolated offset",
        ))
    }

    fn offset_word(&mut self) -> u16 {
        while self
            .source
            .get(self.position)
            .is_some_and(|&byte| label_continue(byte))
        {
            self.position += 1;
        }
        T_STRING
    }

    fn offset_number(&mut self) -> u16 {
        self.integer_offset();
        T_NUM_STRING
    }

    pub(super) fn heredoc(&mut self) -> Result<u16, Diagnostic> {
        let start = self.position;
        self.position += 3;
        while matches!(self.rest().first(), Some(b' ' | b'\t')) {
            self.position += 1;
        }
        let quote = self.heredoc_quote();
        let label_start = self.position;
        if !self
            .rest()
            .first()
            .is_some_and(|&byte| super::label_start(byte))
        {
            return Err(error(ErrorKind::Lexical, start, "invalid heredoc label"));
        }
        self.offset_word();
        let label = label_start..self.position;
        self.heredoc_newline(quote)?;
        reserve(&mut self.heredocs, 1)?;
        let id = self.heredocs.len();
        self.heredocs.push(Heredoc {
            label,
            body_start: self.position,
            segments: Vec::new(),
        });
        self.mode = Mode::Text(Text::Heredoc {
            id,
            nowdoc: quote == b'\'',
        });
        Ok(T_START_HEREDOC)
    }

    fn heredoc_quote(&mut self) -> u8 {
        if matches!(self.rest().first(), Some(b'\'' | b'"')) {
            let quote = self.source[self.position];
            self.position += 1;
            quote
        } else {
            0
        }
    }

    fn heredoc_newline(&mut self, quote: u8) -> Result<(), Diagnostic> {
        if quote != 0 {
            if self.rest().first() != Some(&quote) {
                return Err(error(
                    ErrorKind::Lexical,
                    self.position,
                    "unclosed heredoc label",
                ));
            }
            self.position += 1;
        }
        if !matches!(self.rest().first(), Some(b'\r' | b'\n')) {
            return Err(error(
                ErrorKind::Lexical,
                self.position,
                "heredoc label requires newline",
            ));
        }
        self.skip_newline();
        Ok(())
    }

    fn finish_heredoc(&mut self, id: usize) -> Result<(), Diagnostic> {
        if id + 1 != self.heredocs.len() {
            return Err(error(
                ErrorKind::Lexical,
                self.position,
                "invalid heredoc nesting",
            ));
        }
        let context = self.heredocs.last().unwrap();
        let end = self.position;
        let label = skip_indentation(self.source, end);
        let indent = &self.source[end..label];
        check_indentation(self.source, context, end, indent)?;
        self.heredocs.pop();
        Ok(())
    }
}

fn skip_indentation(source: &[u8], mut position: usize) -> usize {
    while matches!(source.get(position), Some(b' ' | b'\t')) {
        position += 1;
    }
    position
}

fn next_line(source: &[u8], position: usize) -> usize {
    let end = source[position..]
        .iter()
        .position(|byte| matches!(byte, b'\r' | b'\n'));
    let Some(length) = end else {
        return source.len();
    };
    let end = position + length + 1;
    end + usize::from(source.get(end - 1..end + 1) == Some(b"\r\n"))
}

fn line_start(source: &[u8], position: usize) -> bool {
    position == 0 || matches!(source.get(position - 1), Some(b'\r' | b'\n'))
}

fn check_indentation(
    source: &[u8],
    context: &Heredoc,
    end: usize,
    indent: &[u8],
) -> Result<(), Diagnostic> {
    if indent.contains(&b' ') && indent.contains(&b'\t') {
        return Err(error(ErrorKind::Lexical, end, "mixed heredoc indentation"));
    }
    if context.body_start < end {
        let start = context.body_start;
        validate_indent_line(source, start, skip_indentation(source, start), indent)?;
    }
    for segment in &context.segments {
        check_segment_indentation(source, segment.clone(), end, indent)?;
    }
    Ok(())
}

fn check_segment_indentation(
    source: &[u8],
    segment: Range<usize>,
    end: usize,
    indent: &[u8],
) -> Result<(), Diagnostic> {
    let mut line = segment.start;
    if !line_start(source, line) {
        line = next_line(source, line);
    }
    while line <= segment.end && line < end {
        validate_indent_line(source, line, skip_indentation(source, line), indent)?;
        line = next_line(source, line);
    }
    Ok(())
}

fn validate_indent_line(
    source: &[u8],
    line: usize,
    content: usize,
    indent: &[u8],
) -> Result<(), Diagnostic> {
    let actual = &source[line..content];
    if actual.starts_with(indent) {
        return Ok(());
    }
    if matches!(source.get(content), Some(b'\r' | b'\n')) && indent.starts_with(actual) {
        return Ok(());
    }
    Err(error(
        ErrorKind::Lexical,
        line,
        "insufficient or mixed heredoc indentation",
    ))
}

fn escape_end(source: &[u8], start: usize) -> Result<usize, Diagnostic> {
    if !source[start..].starts_with(b"\\u{") {
        return Ok((start + 2).min(source.len()));
    }
    let mut position = start + 3;
    let mut value = 0_u32;
    while let Some(&byte) = source.get(position) {
        if byte == b'}' {
            return finish_unicode_escape(start, position);
        }
        let digit = super::numbers::digit_value(byte, 16)
            .ok_or_else(|| error(ErrorKind::Lexical, start, "invalid Unicode escape"))?;
        value = value
            .checked_mul(16)
            .and_then(|v| v.checked_add(digit))
            .filter(|&v| v <= 0x10ffff)
            .ok_or_else(|| error(ErrorKind::Lexical, start, "Unicode codepoint too large"))?;
        position += 1;
    }
    Err(error(ErrorKind::Lexical, start, "unclosed Unicode escape"))
}

fn finish_unicode_escape(start: usize, end: usize) -> Result<usize, Diagnostic> {
    if end == start + 3 {
        return Err(error(ErrorKind::Lexical, start, "empty Unicode escape"));
    }
    Ok(end + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::DEFAULT_OPTIONS;

    #[test]
    fn heredoc_close_preserves_active_identity_phprs_034() {
        let source = b"<?php $s=<<<OUT\n{$o->f(<<<INNER\nvalue\nINNER\n)}\nOUT;";
        let mut lexer = Lexer::new(source, DEFAULT_OPTIONS);
        while lexer.heredocs.len() < 2 {
            assert_ne!(lexer.next().unwrap().id, 0);
        }
        let position = lexer.position;
        let labels: Vec<_> = lexer.heredocs.iter().map(|h| h.label.clone()).collect();
        let diagnostic = lexer.finish_heredoc(0).unwrap_err();
        assert_eq!(diagnostic.kind, ErrorKind::Lexical);
        assert_eq!(diagnostic.message, "invalid heredoc nesting");
        assert_eq!(lexer.position, position);
        assert_eq!(
            lexer
                .heredocs
                .iter()
                .map(|h| h.label.clone())
                .collect::<Vec<_>>(),
            labels
        );
        while lexer.next().unwrap().id != 0 {}
        assert!(lexer.heredocs.is_empty());
        assert!(lexer.frames.is_empty());
        assert!(crate::parse(source, DEFAULT_OPTIONS).is_ok());
    }
}

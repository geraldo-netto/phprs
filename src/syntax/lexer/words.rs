use super::*;

impl Lexer<'_> {
    fn label_end(&self, start: usize) -> usize {
        let mut end = start;
        while self
            .source
            .get(end)
            .is_some_and(|&byte| label_continue(byte))
        {
            end += 1;
        }
        end
    }

    pub(super) fn variable(&mut self) -> u16 {
        self.position = self.label_end(self.position + 1);
        T_VARIABLE
    }

    pub(super) fn word(&mut self) -> u16 {
        let start = self.position;
        let fully_qualified = self.source[start] == b'\\';
        self.position += usize::from(fully_qualified);
        if !self
            .source
            .get(self.position)
            .is_some_and(|&byte| label_start(byte))
        {
            return T_NS_SEPARATOR;
        }
        self.position = self.label_end(self.position);
        while self.qualified_segment() {
            self.position = self.label_end(self.position + 1);
        }
        let word = &self.source[start..self.position];
        if fully_qualified {
            return T_NAME_FULLY_QUALIFIED;
        }
        if word.contains(&b'\\') {
            return qualified_kind(word);
        }
        if word.eq_ignore_ascii_case(b"die") {
            return T_EXIT;
        }
        KEYWORDS
            .iter()
            .find(|&&(name, _)| word.eq_ignore_ascii_case(name.as_bytes()))
            .map_or(T_STRING, |&(_, id)| id)
    }

    fn qualified_segment(&self) -> bool {
        if self.source.get(self.position) != Some(&b'\\') {
            return false;
        }
        self.source
            .get(self.position + 1)
            .is_some_and(|&byte| label_start(byte))
    }

    pub(super) fn contextual_word(&mut self, id: u16) -> Result<u16, Diagnostic> {
        if id != T_YIELD && id != T_ENUM {
            return Ok(id);
        }
        let saved = self.position;
        while self.position < self.source.len() && self.skip_trivia()? {}
        let end = self.label_end(self.position);
        let following = &self.source[self.position..end];
        if id == T_YIELD && following.eq_ignore_ascii_case(b"from") {
            self.position = end;
            return Ok(T_YIELD_FROM);
        }
        let is_enum = enum_context(id, saved, self.position, following);
        self.position = saved;
        Ok(is_enum)
    }

    pub(super) fn property(&mut self) -> Option<u16> {
        if whitespace(self.source[self.position]) {
            self.position += 1;
            return None;
        }
        self.mode = Mode::Code;
        if label_start(self.source[self.position]) {
            self.position = self.label_end(self.position);
            return Some(T_STRING);
        }
        None
    }

    pub(super) fn varname(&mut self) -> Option<u16> {
        self.mode = Mode::Code;
        if !label_start(self.source[self.position]) {
            return None;
        }
        let end = self.label_end(self.position);
        if matches!(self.source.get(end), Some(b'[' | b'}')) {
            self.position = end;
            return Some(T_STRING_VARNAME);
        }
        None
    }

    pub(super) fn ampersand(&mut self) -> Result<u16, Diagnostic> {
        if self.rest().starts_with(b"&=") {
            self.position += 2;
            return Ok(T_AND_EQUAL);
        }
        if self.rest().starts_with(b"&&") {
            self.position += 2;
            return Ok(T_BOOLEAN_AND);
        }
        self.position += 1;
        let saved = self.position;
        while self.position < self.source.len() && self.skip_trivia()? {}
        let followed = self.rest().starts_with(b"$") || self.rest().starts_with(b"...");
        self.position = saved;
        Ok(if followed {
            T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG
        } else {
            T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG
        })
    }

    pub(super) fn cast(&mut self) -> Result<Option<u16>, Diagnostic> {
        let mut end = self.position + 1;
        while matches!(self.source.get(end), Some(b' ' | b'\t')) {
            end += 1;
        }
        let start = end;
        end = self.label_end(end);
        let word = &self.source[start..end];
        while matches!(self.source.get(end), Some(b' ' | b'\t')) {
            end += 1;
        }
        if self.source.get(end) != Some(&b')') {
            return Ok(None);
        }
        if word.eq_ignore_ascii_case(b"real") {
            return Err(error(ErrorKind::Lexical, start, "removed real cast"));
        }
        let id = cast_kind(word);
        if id.is_some() {
            self.position = end + 1;
        }
        Ok(id)
    }
}

fn qualified_kind(word: &[u8]) -> u16 {
    if word.len() > 10 && word[..10].eq_ignore_ascii_case(b"namespace\\") {
        T_NAME_RELATIVE
    } else {
        T_NAME_QUALIFIED
    }
}

fn enum_context(id: u16, start: usize, end: usize, following: &[u8]) -> u16 {
    if id != T_ENUM {
        return id;
    }
    if start == end || following.is_empty() {
        return T_STRING;
    }
    if following.eq_ignore_ascii_case(b"extends") || following.eq_ignore_ascii_case(b"implements") {
        return T_STRING;
    }
    T_ENUM
}

fn cast_kind(word: &[u8]) -> Option<u16> {
    let casts: &[(&[u8], u16)] = &[
        (b"int", T_INT_CAST),
        (b"integer", T_INT_CAST),
        (b"float", T_DOUBLE_CAST),
        (b"double", T_DOUBLE_CAST),
        (b"string", T_STRING_CAST),
        (b"binary", T_STRING_CAST),
        (b"array", T_ARRAY_CAST),
        (b"object", T_OBJECT_CAST),
        (b"bool", T_BOOL_CAST),
        (b"boolean", T_BOOL_CAST),
        (b"unset", T_UNSET_CAST),
    ];
    casts
        .iter()
        .find(|&&(name, _)| word.eq_ignore_ascii_case(name))
        .map(|&(_, id)| id)
}

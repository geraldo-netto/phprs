use super::*;

impl Lexer<'_> {
    pub(super) fn number(&mut self) -> Result<u16, Diagnostic> {
        let start = self.position;
        if let Some(radix) = self.radix_prefix() {
            return self.radix_number(radix);
        }
        self.digits(10);
        let mut float = false;
        if self.source.get(self.position) == Some(&b'.') {
            self.position += 1;
            self.digits(10);
            float = true;
        }
        float |= self.exponent();
        if float {
            return Ok(T_DNUMBER);
        }
        let bytes = &self.source[start..self.position];
        if invalid_octal(bytes) {
            return Err(error(ErrorKind::Lexical, start, "invalid octal integer"));
        }
        let radix = if bytes.starts_with(b"0") { 8 } else { 10 };
        Ok(integer_kind(bytes, radix))
    }

    pub(super) fn integer_offset(&mut self) {
        if let Some(radix) = self.radix_prefix() {
            self.position += 2;
            self.digits(radix);
        } else {
            self.digits(10);
        }
    }

    fn radix_prefix(&self) -> Option<u32> {
        if self.rest().first() != Some(&b'0') {
            return None;
        }
        let radix = match self.rest().get(1)?.to_ascii_lowercase() {
            b'x' => 16,
            b'b' => 2,
            b'o' => 8,
            _ => return None,
        };
        digit_value(*self.rest().get(2)?, radix)?;
        Some(radix)
    }

    fn radix_number(&mut self, radix: u32) -> Result<u16, Diagnostic> {
        self.position += 2;
        let start = self.position;
        self.digits(radix);
        Ok(integer_kind(&self.source[start..self.position], radix))
    }

    fn digits(&mut self, radix: u32) {
        let start = self.position;
        while let Some(&byte) = self.source.get(self.position) {
            if digit_value(byte, radix).is_some()
                || (self.position > start && self.digit_separator(radix))
            {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn digit_separator(&self, radix: u32) -> bool {
        if self.source[self.position] != b'_' {
            return false;
        }
        self.source
            .get(self.position + 1)
            .is_some_and(|&byte| digit_value(byte, radix).is_some())
    }

    fn exponent(&mut self) -> bool {
        if !matches!(self.source.get(self.position), Some(b'e' | b'E')) {
            return false;
        }
        let saved = self.position;
        self.position += 1;
        if matches!(self.source.get(self.position), Some(b'+' | b'-')) {
            self.position += 1;
        }
        if !self
            .source
            .get(self.position)
            .is_some_and(u8::is_ascii_digit)
        {
            self.position = saved;
            return false;
        }
        self.digits(10);
        true
    }
}

pub(super) fn digit_value(byte: u8, radix: u32) -> Option<u32> {
    let value = match byte {
        b'0'..=b'9' => u32::from(byte - b'0'),
        b'a'..=b'f' => u32::from(byte - b'a') + 10,
        b'A'..=b'F' => u32::from(byte - b'A') + 10,
        _ => return None,
    };
    (value < radix).then_some(value)
}

fn integer_kind(bytes: &[u8], radix: u32) -> u16 {
    let mut value = 0_u64;
    for &byte in bytes {
        if byte == b'_' {
            continue;
        }
        let digit = u64::from(digit_value(byte, radix).unwrap_or(0));
        let Some(next) = value
            .checked_mul(u64::from(radix))
            .and_then(|v| v.checked_add(digit))
        else {
            return T_DNUMBER;
        };
        if next > i64::MAX as u64 {
            return T_DNUMBER;
        }
        value = next;
    }
    T_LNUMBER
}

fn invalid_octal(bytes: &[u8]) -> bool {
    bytes.starts_with(b"0") && bytes.iter().any(|byte| matches!(byte, b'8' | b'9'))
}

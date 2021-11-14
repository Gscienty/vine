use crate::is_alpha;
use crate::toolbox::chr::{BITS, IDENT};

use super::token::{LexNumberValue, LexToken};
use std::str::Chars;

struct LexStatus<'src_lt> {
    more_fn: fn() -> Option<&'src_lt str>,
    src: &'src_lt str,
    src_p: Chars<'src_lt>,
    buf: Vec<char>,
    chr: Option<char>,
}

impl<'src_lt> LexStatus<'src_lt> {
    fn new(src: &'src_lt str) -> Self {
        LexStatus {
            more_fn: || None,
            src,
            src_p: src.chars(),
            buf: Vec::new(),
            chr: None,
        }
    }

    fn more(&mut self) -> bool {
        match (self.more_fn)() {
            Some(src) => {
                self.src = src;
                self.src_p = self.src.chars();

                true
            }
            None => false,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.chr = match self.src_p.next() {
            Some(c) => Some(c),
            None => {
                if self.more() {
                    self.src_p.next()
                } else {
                    None
                }
            }
        };

        self.chr
    }

    fn save(&mut self, c: char) {
        self.buf.push(c)
    }

    fn save_next(&mut self) -> Option<char> {
        if let Some(c) = self.chr {
            self.save(c);
        }

        self.next()
    }

    fn number(&mut self) -> Option<LexToken> {
        let mut xp = 'e';
        let mut c = self.chr.unwrap();

        if c == '0' {
            if let Some(next) = self.save_next() {
                if (u32::from(next) | 0x20) == u32::from('x') {
                    xp = 'p';
                }
            }
        }

        loop {
            if let Some(t) = self.chr {
                if is_alpha!(t, IDENT)
                    || t == '.'
                    || ((t == '-' || t == '+') && (u32::from(c) | 0x20) == u32::from(xp))
                {
                    c = t;
                    self.save_next();
                    continue;
                }
            }
            break;
        }

        let base = if Some(&'0') == self.buf.get(0) {
            match self.buf.get(1) {
                Some(&'x') => 16,
                Some(&'b') => 2,
                _ => 10,
            }
        } else {
            10
        };

        let value = match base {
            2 => LexNumberValue::new_binary(&self.buf),
            10 => LexNumberValue::new_dec(&self.buf),
            16 => LexNumberValue::new_hex(&self.buf),
            _ => LexNumberValue::Invalid,
        };
        self.buf.clear();

        if value.is_invalid() {
            None
        } else {
            Some(LexToken::Number(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_number_uint_eq(cd: &str, value: u64) {
        let mut lex_status = LexStatus::new(cd);
        lex_status.next();

        let binary_num = lex_status.number();

        assert!(binary_num.is_some());

        if let LexToken::Number(number_value) = binary_num.unwrap() {
            assert!(!number_value.is_invalid());

            assert_eq!(number_value.as_int(), value);
        } else {
            panic!("err");
        }
    }

    fn lex_number_uint_invalid(cd: &str) {
        let mut lex_status = LexStatus::new(cd);
        lex_status.next();

        let binary_num = lex_status.number();

        assert!(binary_num.is_none());
    }

    #[test]
    fn lex_number_valid() {
        lex_number_uint_eq("0b11010011", 0xd3);
        lex_number_uint_eq("0b0000000000111", 7);
        lex_number_uint_eq("0xD3", 0xd3);
        lex_number_uint_eq("0xd3", 0xd3);
        lex_number_uint_eq("0x00d3D3", 0xd3d3);
        lex_number_uint_eq("0x001234", 0x1234);
        lex_number_uint_eq("0x100000", 0x100000);
        lex_number_uint_eq("325", 325);
        lex_number_uint_eq("0", 0);
    }

    #[test]
    fn lex_number_invalid() {
        lex_number_uint_invalid("0b11010012");
        lex_number_uint_invalid("0xabcg");
        lex_number_uint_invalid("132c");
    }
}

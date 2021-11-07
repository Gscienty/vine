use crate::{
    is_alpha,
    toolbox::{
        chr::{BITS, DIGIT, IDENT},
        tag_value::TagValue,
    },
};
use std::str::Chars;

trait Lex {
    fn next(&mut self);
}

enum LexToken {
    And,
    Break,
    Do,
    Else,
    ElseIf,
    End,
    False,
    For,
    Function,
    Goto,
    If,
    In,
    Local,
    Nil,
    Not,
    Or,
    Repeat,
    Return,
    Then,
    True,
    Util,
    While,
    Concat,
    Dots,
    Equal,
    GreateEqual,
    LessEqual,
    NotEqual,
    Label,
    Number,
    Name,
    Str,
    Eof,
}

impl Clone for LexToken {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for LexToken {}

// 语法分析
#[derive()]
pub struct LexState<'src_lf> {
    c: Option<char>,
    token: LexToken,
    look_ahead: LexToken,
    line_number: u32,
    last_line: u32,
    token_value: TagValue,
    look_ahead_value: TagValue,
    buf: Vec<char>,
    p: Chars<'src_lf>,
}

impl<'src_lf> LexState<'src_lf> {
    pub fn new(src: &'src_lf str) -> Self {
        LexState {
            c: None,
            token: LexToken::Eof,
            look_ahead: LexToken::Eof,
            line_number: 0,
            last_line: 0,
            token_value: TagValue::new(0),
            look_ahead_value: TagValue::new(0),
            buf: Vec::new(),
            p: src.chars(),
        }
    }

    fn save(&mut self, c: char) {
        self.buf.push(c)
    }

    pub fn next(&mut self) -> Option<char> {
        self.c = self.p.next();

        self.c
    }

    pub fn save_next(&mut self) -> Option<char> {
        if let Some(c) = self.c {
            self.save(c)
        }

        self.next()
    }

    pub fn scan_num(&mut self) {
        let mut xp = 'e';
        if Some('0') == self.c {
            if let Some(c) = self.save_next() {
                if Some('x') == char::from_u32(c as u32 | 0x20) {
                    xp = 'p';
                }
            }
        }

        loop {
            if let Some(c) = self.c {
                if is_alpha!(c, IDENT)
                    || c == '.'
                    || ((c == '-' || c == '+') && Some(xp) == char::from_u32(c as u32 | 0x20))
                {
                    self.save_next();
                    continue;
                }
            }
            break;
        }
        self.save('\0');
    }

    fn scan(&mut self) -> LexToken {
        let mut ret: LexToken = LexToken::Eof;

        loop {
            if let Some(c) = self.c {
                if is_alpha!(c, IDENT) {
                    if is_alpha!(c, DIGIT) {
                        self.scan_num();

                        ret = LexToken::Number;
                        break;
                    }
                }
            }

            break;
        }

        ret
    }
}

impl<'src_lf> Lex for LexState<'src_lf> {
    fn next(&mut self) {
        self.last_line = self.line_number;
        if let LexToken::Eof = self.look_ahead {
            self.token = self.scan();
        } else {
            self.token = self.look_ahead;
            self.look_ahead = LexToken::Eof;
            self.token_value.set_u64(self.look_ahead_value.get_u64());
        }
    }
}

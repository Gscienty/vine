use crate::is_alpha;
use crate::toolbox::chr::{BITS, DIGIT, IDENT};

use super::token::{LexNumberValue, LexToken};
use std::str::Chars;

// 词法分析状态器
pub struct LexStatus<'src_lt> {
    more_fn: fn() -> Option<&'src_lt str>,
    src: &'src_lt str,
    src_p: Chars<'src_lt>,
    buf: Vec<char>,
    chr: Option<char>,
    line_number: u32,
}

impl<'src_lt> LexStatus<'src_lt> {
    // 构造新的LexStatus
    //
    // @param src: 传入的Lua源码
    //
    // @return: LexStatus
    pub fn new(src: &'src_lt str) -> Self {
        LexStatus {
            more_fn: || None,
            src,
            src_p: src.chars(),
            buf: Vec::new(),
            chr: None,
            line_number: 0,
        }
    }

    // 从more_fn中获取下一段Lua源码
    //
    // @return: Lua源码是否已结束
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

    // 从Lua源码指针处获取一个字符，并将源码指针向下移动一位。
    // 如果源码指针指向结束，则调用more方法加载下一段源码。
    // 如果more方法返回源码读取结束，则返回None
    //
    // @return: 返回一个字符
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

    // 保存一个字符到缓存中
    //
    // @param c: 待保存的字符
    fn save(&mut self, c: char) {
        self.buf.push(c)
    }

    // 保存当前指向的字符到缓存中，并移动源码指针
    //
    // @return: 返回一个字符
    fn save_next(&mut self) -> Option<char> {
        if let Some(c) = self.chr {
            self.save(c);
        }

        self.next()
    }

    // 将接下来一段源码解析为Number
    //
    // @return: 返回一个Token
    pub fn number(&mut self) -> Option<LexToken> {
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

        let value = LexNumberValue::new(&self.buf);
        self.buf.clear();

        if value.is_invalid() {
            None
        } else {
            Some(LexToken::Number(value))
        }
    }

    // 跳过若干等于号
    //
    // @return: 等于号个数
    pub fn skip_eq(&mut self) -> i32 {
        let mut count: i32 = 0;
        let s = self.chr;

        while self.save_next().eq(&Some('=')) && count < 0x20000000 {
            count = count + 1;
        }

        if self.chr == s {
            count
        } else {
            (-count) - 1
        }
    }

    // 判断当前是否处于源码中一行的结束
    //
    // @return: 是否处于源码中的一行
    fn is_eol(&self) -> bool {
        self.chr.eq(&Some('\n')) || self.chr.eq(&Some('\r'))
    }

    // 向下一行
    fn new_line(&mut self) {
        let old = self.chr;

        self.next();

        if self.is_eol() && self.chr != old {
            self.next();
        }

        self.line_number = self.line_number + 1;
    }

    // 将缓冲区的内容转化为字符串
    //
    // @return: 字符串
    fn buf_to_string(&mut self) -> String {
        let mut s = String::new();
        for c in self.buf.iter() {
            s.push(*c);
        }
        self.buf.clear();

        s
    }

    // 接下来的一段源码解析为LongString
    //
    // @return: 返回一个Token
    fn longstring(&mut self, sep: i32) -> Option<LexToken> {
        self.save_next();
        if self.is_eol() {
            self.new_line();
        }

        loop {
            match self.chr {
                None => return None,
                Some(']') => {
                    if self.skip_eq() == sep {
                        self.save_next();
                        break;
                    }
                }
                Some('\r') | Some('\n') => {
                    self.save('\n');
                    self.new_line();
                }
                _ => {
                    self.save_next();
                }
            }
        }

        Some(LexToken::Str(self.buf_to_string()))
    }

    fn string(&mut self) -> Option<LexToken> {
        let delim = self.chr;
        self.save_next();

        while self.chr != delim {
            match self.chr {
                Some('\\') => {
                    let val: char = match self.next() {
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('t') => '\t',
                        _ => return None,
                    };

                    self.save(val);
                    self.next();
                }
                Some('\n') | Some('\r') => return None,
                None => return None,
                _ => {
                    self.save_next();
                }
            }
        }
        self.save_next();

        Some(LexToken::Str(self.buf_to_string()))
    }

    pub fn setup(&mut self) {
        self.next();
    }

    fn name(&mut self) -> Option<LexToken> {
        let tok = self.buf_to_string();
        Some(match tok.as_str() {
            "and" => LexToken::And,
            "break" => LexToken::Break,
            "do" => LexToken::Do,
            "else" => LexToken::Else,
            "elseif" => LexToken::ElseIf,
            "end" => LexToken::End,
            "false" => LexToken::False,
            "for" => LexToken::For,
            "function" => LexToken::Function,
            "goto" => LexToken::Goto,
            "if" => LexToken::If,
            "in" => LexToken::In,
            "local" => LexToken::Local,
            "nil" => LexToken::Nil,
            "not" => LexToken::Not,
            "or" => LexToken::Or,
            "repeat" => LexToken::Repeat,
            "return" => LexToken::Return,
            "then" => LexToken::Then,
            "true" => LexToken::True,
            "util" => LexToken::Util,
            "while" => LexToken::While,
            _ => LexToken::Name(tok),
        })
    }

    fn scan(&mut self) -> Option<LexToken> {
        self.buf.clear();

        loop {
            if is_alpha!(self.chr.unwrap(), IDENT) {
                if is_alpha!(self.chr.unwrap(), DIGIT) {
                    return self.number();
                }

                loop {
                    self.save_next();
                    match self.chr {
                        None => break,
                        Some(c) => {
                            if !is_alpha!(c, IDENT) {
                                break;
                            }
                        }
                    }
                }

                return self.name();
            }

            match self.chr {
                Some('\n') | Some('\r') => {
                    self.new_line();
                }
                Some(' ') | Some('\t') => {
                    self.next();
                }
                Some('-') => {
                    self.next();
                    if self.chr.ne(&Some('-')) {
                        return Some(LexToken::Sub);
                    }
                    self.next();
                    if self.chr.eq(&Some('[')) {
                        let sep = self.skip_eq();
                        self.buf.clear();

                        if sep >= 0 {
                            self.longstring(sep);
                            continue;
                        }
                    }

                    while !self.is_eol() && self.chr.ne(&None) {
                        self.next();
                    }
                }
                Some('[') => {
                    let sep = self.skip_eq();
                    if sep.gt(&0) {
                        return self.longstring(sep);
                    } else if sep.eq(&-1) {
                        return Some(LexToken::SquareBracketLeft);
                    } else {
                        return None;
                    }
                }
                Some('=') => {
                    self.next();
                    if self.chr.ne(&Some('=')) {
                        return Some(LexToken::Assign);
                    } else {
                        self.next();
                        return Some(LexToken::Equal);
                    }
                }
                Some('<') => {
                    self.next();
                    if self.chr.ne(&Some('=')) {
                        return Some(LexToken::Less);
                    } else {
                        self.next();
                        return Some(LexToken::LessEqual);
                    }
                }
                Some('>') => {
                    self.next();
                    if self.chr.ne(&Some('=')) {
                        return Some(LexToken::Greate);
                    } else {
                        self.next();
                        return Some(LexToken::GreateEqual);
                    }
                }
                Some('~') => {
                    self.next();
                    if self.chr.ne(&Some('=')) {
                        return None;
                    } else {
                        self.next();
                        return Some(LexToken::NotEqual);
                    }
                }
                Some(':') => {
                    self.next();
                    if self.chr.ne(&Some(':')) {
                        return Some(LexToken::MethodCall);
                    } else {
                        self.next();
                        return Some(LexToken::Label);
                    }
                }
                Some('\'') | Some('"') => {
                    return self.string();
                }
                Some('.') => {
                    if self.save_next().eq(&Some('.')) {
                        self.next();
                        if self.chr.eq(&Some('.')) {
                            self.next();
                            return Some(LexToken::Dots);
                        }
                        return Some(LexToken::Concat);
                    } else if is_alpha!(self.chr.unwrap(), DIGIT) {
                        return Some(LexToken::Dot);
                    } else {
                        return self.number();
                    }
                }
                Some('+') => return Some(LexToken::Add),
                Some('*') => return Some(LexToken::Mul),
                Some('/') => return Some(LexToken::Div),
                Some('%') => return Some(LexToken::Mod),
                None => return Some(LexToken::Eof),
                _ => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_scan_dec_number() {
        let mut lex = LexStatus::new("1123");
        lex.setup();

        let token = lex.scan();
        assert!(token.is_some());
        if let Some(LexToken::Number(x)) = token {
            assert_eq!(x.as_int(), 1123);
        }
    }

    #[test]
    fn lex_scan_hex_number() {
        let mut lex = LexStatus::new("0x1123");
        lex.setup();

        let token = lex.scan();
        assert!(token.is_some());
        if let Some(LexToken::Number(x)) = token {
            assert_eq!(x.as_int(), 0x1123);
        }
    }

    #[test]
    fn lex_scan_bin_number() {
        let mut lex = LexStatus::new("0b0001000100100011");
        lex.setup();

        let token = lex.scan();
        assert!(token.is_some());
        if let Some(LexToken::Number(x)) = token {
            assert_eq!(x.as_int(), 0x1123);
        }
    }

    #[test]
    fn lex_scan_float_number_1() {
        let mut lex = LexStatus::new(".51");
        lex.setup();

        let token = lex.scan();
        assert!(token.is_some());
        if let Some(LexToken::Number(x)) = token {
            assert_eq!(x.as_float(), 0.51f64);
        }
    }

    #[test]
    fn lex_scan_float_number_2() {
        let mut lex = LexStatus::new("1123.51");
        lex.setup();

        let token = lex.scan();
        assert!(token.is_some());
        if let Some(LexToken::Number(x)) = token {
            assert_eq!(x.as_float(), 1123.51f64);
        }
    }

    fn assert_name(s: &str) -> LexToken {
        let mut lex = LexStatus::new(s);
        lex.setup();
        let token = lex.scan();

        assert!(token.is_some());
        if let Some(x) = token {
            x
        } else {
            LexToken::Eof
        }
    }

    #[test]
    fn lex_scan_keyword() {
        assert!(assert_name("and") == LexToken::And);
        assert!(assert_name("break") == LexToken::Break);
        assert!(assert_name("do") == LexToken::Do);
        assert!(assert_name("else") == LexToken::Else);
        assert!(assert_name("elseif") == LexToken::ElseIf);
        assert!(assert_name("end") == LexToken::End);
        assert!(assert_name("false") == LexToken::False);
        assert!(assert_name("for") == LexToken::For);
        assert!(assert_name("function") == LexToken::Function);
        assert!(assert_name("goto") == LexToken::Goto);
        assert!(assert_name("if") == LexToken::If);
        assert!(assert_name("in") == LexToken::In);
        assert!(assert_name("local") == LexToken::Local);
        assert!(assert_name("nil") == LexToken::Nil);
        assert!(assert_name("not") == LexToken::Not);
        assert!(assert_name("or") == LexToken::Or);
        assert!(assert_name("repeat") == LexToken::Repeat);
        assert!(assert_name("return") == LexToken::Return);
        assert!(assert_name("then") == LexToken::Then);
        assert!(assert_name("true") == LexToken::True);
        assert!(assert_name("util") == LexToken::Util);
        assert!(assert_name("while") == LexToken::While);
    }

    #[test]
    fn lex_scan_operator() {
        assert!(assert_name("+") == LexToken::Add);
        assert!(assert_name("-") == LexToken::Sub);
        assert!(assert_name("*") == LexToken::Mul);
        assert!(assert_name("/") == LexToken::Div);
        assert!(assert_name("%") == LexToken::Mod)
    }
}

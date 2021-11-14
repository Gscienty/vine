pub use super::number::LexNumberValue;

pub enum LexToken {
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
    Number(LexNumberValue),
    Name,
    Str,
    Eof,
}

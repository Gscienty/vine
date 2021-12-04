pub use super::number::LexNumberValue;

pub enum LexToken {
    Add,
    And,
    Assign,
    Break,
    Concat,
    Div,
    Do,
    Dot,
    Dots,
    Else,
    ElseIf,
    End,
    Eof,
    Equal,
    False,
    For,
    Function,
    Goto,
    Greate,
    GreateEqual,
    If,
    In,
    Label,
    Less,
    LessEqual,
    Local,
    MethodCall,
    Mod,
    Mul,
    Name(String),
    Nil,
    Not,
    NotEqual,
    Number(LexNumberValue),
    Or,
    Repeat,
    Return,
    SquareBracketLeft,
    Str(String),
    Sub,
    Then,
    True,
    Util,
    While,
}

impl PartialEq for LexToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Name(l0), Self::Name(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

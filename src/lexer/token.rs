use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'de> {
    pub origin: &'de str,
    pub offset: usize,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Identifier,
    Literal(Literal),
    Punctuation(Punctuation),
    Delimiter(Delimiter),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    String,
    Number(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuation {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Comma,
    Dot,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Delimiter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = self.origin;
        match &self.kind {
            TokenKind::Keyword(kw) => write!(f, "{} {} null", kw, origin),
            TokenKind::Identifier => write!(f, "IDENTIFIER {} null", origin),
            TokenKind::Literal(Literal::String) => {
                write!(f, "STRING {} {}", origin, Token::unescape(origin))
            }
            TokenKind::Literal(Literal::Number(n)) => {
                if *n == n.trunc() {
                    write!(f, "NUMBER {} {n}.0", origin)
                } else {
                    write!(f, "NUMBER {} {n}", origin)
                }
            }
            TokenKind::Punctuation(punc) => write!(f, "{} {} null", punc, origin),
            TokenKind::Delimiter(delim) => write!(f, "{} {} null", delim, origin),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keyword_str = match self {
            Keyword::And => "AND",
            Keyword::Class => "CLASS",
            Keyword::Else => "ELSE",
            Keyword::False => "FALSE",
            Keyword::For => "FOR",
            Keyword::Fun => "FUN",
            Keyword::If => "IF",
            Keyword::Nil => "NIL",
            Keyword::Or => "OR",
            Keyword::Print => "PRINT",
            Keyword::Return => "RETURN",
            Keyword::Super => "SUPER",
            Keyword::This => "THIS",
            Keyword::True => "TRUE",
            Keyword::Var => "VAR",
            Keyword::While => "WHILE",
        };
        write!(f, "{}", keyword_str)
    }
}

impl fmt::Display for Punctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let punct_str = match self {
            Punctuation::Bang => "BANG",
            Punctuation::BangEqual => "BANG_EQUAL",
            Punctuation::Equal => "EQUAL",
            Punctuation::EqualEqual => "EQUAL_EQUAL",
            Punctuation::Greater => "GREATER",
            Punctuation::GreaterEqual => "GREATER_EQUAL",
            Punctuation::Less => "LESS",
            Punctuation::LessEqual => "LESS_EQUAL",
            Punctuation::Plus => "PLUS",
            Punctuation::Minus => "MINUS",
            Punctuation::Star => "STAR",
            Punctuation::Slash => "SLASH",
            Punctuation::Comma => "COMMA",
            Punctuation::Dot => "DOT",
            Punctuation::Semicolon => "SEMICOLON",
        };
        write!(f, "{}", punct_str)
    }
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let delim_str = match self {
            Delimiter::LeftParen => "LEFT_PAREN",
            Delimiter::RightParen => "RIGHT_PAREN",
            Delimiter::LeftBrace => "LEFT_BRACE",
            Delimiter::RightBrace => "RIGHT_BRACE",
        };
        write!(f, "{}", delim_str)
    }
}

impl Token<'_> {
    pub fn unescape(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s.trim_matches('"'))
    }
}

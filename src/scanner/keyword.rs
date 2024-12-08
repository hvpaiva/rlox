use phf::phf_map;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Keyword {
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "and" => Keyword::AND,
    "class" => Keyword::CLASS,
    "else" => Keyword::ELSE,
    "false" => Keyword::FALSE,
    "for" => Keyword::FOR,
    "fun" => Keyword::FUN,
    "if" => Keyword::IF,
    "nil" => Keyword::NIL,
    "or" => Keyword::OR,
    "print" => Keyword::PRINT,
    "return" => Keyword::RETURN,
    "super" => Keyword::SUPER,
    "this" => Keyword::THIS,
    "true" => Keyword::TRUE,
    "var" => Keyword::VAR,
    "while" => Keyword::WHILE,
};

impl Keyword {
    pub fn as_str(&self) -> &'static str {
        match self {
            Keyword::AND => "and",
            Keyword::CLASS => "class",
            Keyword::ELSE => "else",
            Keyword::FALSE => "false",
            Keyword::FOR => "for",
            Keyword::FUN => "fun",
            Keyword::IF => "if",
            Keyword::NIL => "nil",
            Keyword::OR => "or",
            Keyword::PRINT => "print",
            Keyword::RETURN => "return",
            Keyword::SUPER => "super",
            Keyword::THIS => "this",
            Keyword::TRUE => "true",
            Keyword::VAR => "var",
            Keyword::WHILE => "while",
        }
    }

    pub fn from(lexeme: &str) -> Option<Keyword> {
        KEYWORDS.get(lexeme).cloned()
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

use crate::lexer::token::{Delimiter, Keyword, Literal, Punctuation};

use super::errors::{Eof, SingleTokenError, StringTerminationError};
use super::token::{Token, TokenKind};
use miette::{Error, LabeledSpan, SourceSpan};

pub struct Lexer<'de> {
    source: &'de str,
    remainder: &'de str,
    byte_offset: usize,
    peeked: Option<Result<Token<'de>, miette::Error>>,
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            source: input,
            remainder: input,
            byte_offset: 0,
            peeked: None,
        }
    }

    pub fn expect(
        &mut self,
        expected: TokenKind,
        unexpected: &str,
    ) -> Result<Token<'de>, miette::Error> {
        self.expect_where(|next| next.kind == expected, unexpected)
    }

    pub fn expect_where(
        &mut self,
        mut check: impl FnMut(&Token<'de>) -> bool,
        unexpected: &str,
    ) -> Result<Token<'de>, miette::Error> {
        match self.next() {
            Some(Ok(token)) if check(&token) => Ok(token),
            Some(Ok(token)) => Err(miette::miette! {
                labels = vec![
                    LabeledSpan::at(token.offset..token.offset + token.origin.len(), "here"),
                ],
                help = format!("Expected {token:?}"),
                "{unexpected}",
            }
            .with_source_code(self.source.to_string())),
            Some(Err(e)) => Err(e),
            None => Err(Eof.into()),
        }
    }

    pub fn peek(&mut self) -> Option<&Result<Token<'de>, miette::Error>> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }

        self.peeked = self.next();
        self.peeked.as_ref()
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;

    /// Once the iterator returns `Err`, it will only return `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.peeked.take() {
            return Some(next);
        }

        loop {
            // NOTE: this must be in the loop for the indices to match up with c_onwards
            let mut chars = self.remainder.chars();
            let c = chars.next()?;
            let c_at = self.byte_offset;
            let c_str = &self.remainder[..c.len_utf8()];
            let c_onwards = self.remainder;
            self.remainder = chars.as_str();
            self.byte_offset += c.len_utf8();

            enum Started {
                Slash,
                String,
                Number,
                Ident,
                IfEqualElse(Punctuation, Punctuation),
            }

            let just = move |kind: TokenKind| {
                Some(Ok(Token {
                    kind,
                    offset: c_at,
                    origin: c_str,
                }))
            };

            let started = match c {
                '(' => return just(TokenKind::Delimiter(Delimiter::LeftParen)),
                ')' => return just(TokenKind::Delimiter(Delimiter::RightParen)),
                '{' => return just(TokenKind::Delimiter(Delimiter::LeftBrace)),
                '}' => return just(TokenKind::Delimiter(Delimiter::RightBrace)),
                ',' => return just(TokenKind::Punctuation(Punctuation::Comma)),
                '.' => return just(TokenKind::Punctuation(Punctuation::Dot)),
                '-' => return just(TokenKind::Punctuation(Punctuation::Minus)),
                '+' => return just(TokenKind::Punctuation(Punctuation::Plus)),
                ';' => return just(TokenKind::Punctuation(Punctuation::Semicolon)),
                '*' => return just(TokenKind::Punctuation(Punctuation::Star)),
                '/' => Started::Slash,
                '<' => Started::IfEqualElse(Punctuation::LessEqual, Punctuation::Less),
                '>' => Started::IfEqualElse(Punctuation::GreaterEqual, Punctuation::Greater),
                '!' => Started::IfEqualElse(Punctuation::BangEqual, Punctuation::Bang),
                '=' => Started::IfEqualElse(Punctuation::EqualEqual, Punctuation::Equal),
                '"' => Started::String,
                '0'..='9' => Started::Number,
                'a'..='z' | 'A'..='Z' | '_' => Started::Ident,
                c if c.is_whitespace() => continue,
                c => {
                    return Some(Err(SingleTokenError {
                        src: self.source.to_string(),
                        token: c,
                        err_span: SourceSpan::from(
                            self.byte_offset - c.len_utf8()..self.byte_offset,
                        ),
                    }
                    .into()));
                }
            };

            break match started {
                Started::String => {
                    if let Some(end) = self.remainder.find('"') {
                        let literal = &c_onwards[..end + 1 + 1];
                        self.byte_offset += end + 1;
                        self.remainder = &self.remainder[end + 1..];
                        Some(Ok(Token {
                            origin: literal,
                            offset: c_at,
                            kind: TokenKind::Literal(Literal::String),
                        }))
                    } else {
                        let err = StringTerminationError {
                            src: self.source.to_string(),
                            err_span: SourceSpan::from(
                                self.byte_offset - c.len_utf8()..self.source.len(),
                            ),
                        };

                        // swallow the remainder of input as being a string
                        self.byte_offset += self.remainder.len();
                        self.remainder = &self.remainder[self.remainder.len()..];

                        return Some(Err(err.into()));
                    }
                }
                Started::Slash => {
                    if self.remainder.starts_with('/') {
                        let line_end = self.remainder.find('\n').unwrap_or(self.remainder.len());
                        self.byte_offset += line_end;
                        self.remainder = &self.remainder[line_end..];
                        continue;
                    } else {
                        Some(Ok(Token {
                            origin: c_str,
                            offset: c_at,
                            kind: TokenKind::Punctuation(Punctuation::Slash),
                        }))
                    }
                }
                Started::Ident => {
                    let first_non_ident = c_onwards
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                        .unwrap_or(c_onwards.len());

                    let literal = &c_onwards[..first_non_ident];
                    let extra_bytes = literal.len() - c.len_utf8();
                    self.byte_offset += extra_bytes;
                    self.remainder = &self.remainder[extra_bytes..];

                    let kind = match literal {
                        "and" => TokenKind::Keyword(Keyword::And),
                        "class" => TokenKind::Keyword(Keyword::Class),
                        "else" => TokenKind::Keyword(Keyword::Else),
                        "false" => TokenKind::Keyword(Keyword::False),
                        "for" => TokenKind::Keyword(Keyword::For),
                        "fun" => TokenKind::Keyword(Keyword::Fun),
                        "if" => TokenKind::Keyword(Keyword::If),
                        "nil" => TokenKind::Keyword(Keyword::Nil),
                        "or" => TokenKind::Keyword(Keyword::Or),
                        "print" => TokenKind::Keyword(Keyword::Print),
                        "return" => TokenKind::Keyword(Keyword::Return),
                        "super" => TokenKind::Keyword(Keyword::Super),
                        "this" => TokenKind::Keyword(Keyword::This),
                        "true" => TokenKind::Keyword(Keyword::True),
                        "var" => TokenKind::Keyword(Keyword::Var),
                        "while" => TokenKind::Keyword(Keyword::While),
                        _ => TokenKind::Identifier,
                    };

                    return Some(Ok(Token {
                        origin: literal,
                        offset: c_at,
                        kind,
                    }));
                }
                Started::Number => {
                    let first_non_digit = c_onwards
                        .find(|c| !matches!(c, '.' | '0'..='9'))
                        .unwrap_or(c_onwards.len());

                    let mut literal = &c_onwards[..first_non_digit];
                    let mut dotted = literal.splitn(3, '.');
                    match (dotted.next(), dotted.next(), dotted.next()) {
                        (Some(one), Some(two), Some(_)) => {
                            literal = &literal[..one.len() + 1 + two.len()];
                        }
                        (Some(one), Some(""), None) => {
                            literal = &literal[..one.len()];
                        }
                        _ => {
                            // leave literal as-is
                        }
                    }
                    let extra_bytes = literal.len() - c.len_utf8();
                    self.byte_offset += extra_bytes;
                    self.remainder = &self.remainder[extra_bytes..];

                    let n = match literal.parse() {
                        Ok(n) => n,
                        Err(e) => {
                            return Some(Err(miette::miette! {
                                labels = vec![
                                    LabeledSpan::at(self.byte_offset - literal.len()..self.byte_offset, "this numeric literal"),
                                ],
                                "{e}",
                            }.with_source_code(self.source.to_string())));
                        }
                    };

                    return Some(Ok(Token {
                        origin: literal,
                        offset: c_at,
                        kind: TokenKind::Literal(Literal::Number(n)),
                    }));
                }
                Started::IfEqualElse(yes, no) => {
                    self.remainder = self.remainder.trim_start();
                    let trimmed = c_onwards.len() - self.remainder.len() - 1;
                    self.byte_offset += trimmed;
                    if self.remainder.starts_with('=') {
                        let span = &c_onwards[..c.len_utf8() + trimmed + 1];
                        self.remainder = &self.remainder[1..];
                        self.byte_offset += 1;
                        Some(Ok(Token {
                            origin: span,
                            offset: c_at,
                            kind: TokenKind::Punctuation(yes),
                        }))
                    } else {
                        Some(Ok(Token {
                            origin: c_str,
                            offset: c_at,
                            kind: TokenKind::Punctuation(no),
                        }))
                    }
                }
            };
        }
    }
}

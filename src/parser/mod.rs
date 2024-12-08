use std::fmt::{Display, Formatter};

use crate::{
    formatter::format_number,
    report::Reporter,
    scanner::{Keyword, Literal as TokenLiteral, Token, TokenType},
    Process,
};

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(binary) => {
                write!(f, "({} {} {})", binary.operator, binary.left, binary.right)
            }
            Expr::Unary(unary) => write!(f, "({} {})", unary.operator, unary.right),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Number(n) => write!(f, "{}", format_number(*n)),
            Literal::String(s) => write!(f, "{}", s),
            Literal::None => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    left: Box<Expr>,
    operator: Operator,
    right: Box<Expr>,
}

#[derive(Debug)]
pub struct Unary {
    operator: Operator,
    right: Box<Expr>,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    reporter: Reporter,
}

#[derive(Debug)]
pub enum Operator {
    EqualEqual,
    Equal,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl Operator {
    fn from_token(token: &Token) -> Option<Self> {
        match token.ty {
            TokenType::EQUAL => Some(Self::Equal),
            TokenType::EQUAL_EQUAL => Some(Self::EqualEqual),
            TokenType::BANG => Some(Self::Bang),
            TokenType::BANG_EQUAL => Some(Self::BangEqual),
            TokenType::LESS => Some(Self::Less),
            TokenType::LESS_EQUAL => Some(Self::LessEqual),
            TokenType::GREATER => Some(Self::Greater),
            TokenType::GREATER_EQUAL => Some(Self::GreaterEqual),
            TokenType::PLUS => Some(Self::Plus),
            TokenType::MINUS => Some(Self::Minus),
            TokenType::STAR => Some(Self::Star),
            TokenType::SLASH => Some(Self::Slash),
            _ => None,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Equal => write!(f, "="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Bang => write!(f, "!"),
            Operator::BangEqual => write!(f, "!="),
            Operator::Less => write!(f, "<"),
            Operator::LessEqual => write!(f, "<="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Star => write!(f, "*"),
            Operator::Slash => write!(f, "/"),
        }
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            reporter: Reporter::new(),
        }
    }

    fn make_binary_expr(left: Option<Expr>, operator: Token, right: Expr) -> Option<Expr> {
        Some(Expr::Binary(Box::new(Binary {
            left: Box::new(left?),
            operator: Operator::from_token(&operator)?,
            right: Box::new(right),
        })))
    }

    fn binary_expr_loop<F>(
        &mut self,
        mut left: Option<Expr>,
        token_types: &[TokenType],
        next_level: F,
    ) -> Option<Expr>
    where
        F: Fn(&mut Self) -> Option<Expr>,
    {
        while token_types.iter().any(|tt| self.match_ty(tt)) {
            let operator = self.previous().clone();
            let right = next_level(self)?;
            left = Self::make_binary_expr(left, operator, right);
        }
        left
    }

    /// expression -> equality
    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    /// equality -> comparison ( ( "!=" | "==" ) comparison )*
    fn equality(&mut self) -> Option<Expr> {
        let expr = self.comparison();
        self.binary_expr_loop(
            expr,
            &[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL],
            Self::comparison,
        )
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, ty: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().ty == *ty
        }
    }

    fn match_ty(&mut self, ty: &TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Option<Expr> {
        let expr = self.term();
        self.binary_expr_loop(
            expr,
            &[
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                TokenType::LESS,
                TokenType::LESS_EQUAL,
            ],
            Self::term,
        )
    }

    /// term -> factor ( ( "-" | "+" ) factor )*
    fn term(&mut self) -> Option<Expr> {
        let expr = self.factor();
        self.binary_expr_loop(expr, &[TokenType::MINUS, TokenType::PLUS], Self::factor)
    }

    /// factor -> unary ( ( "*" | "/" ) unary )*
    fn factor(&mut self) -> Option<Expr> {
        let expr = self.unary();
        self.binary_expr_loop(expr, &[TokenType::STAR, TokenType::SLASH], Self::unary)
    }

    /// unary -> ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Option<Expr> {
        if self.match_ty(&TokenType::BANG) || self.match_ty(&TokenType::MINUS) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Some(Expr::Unary(Box::new(Unary {
                operator: Operator::from_token(&operator)?,
                right: Box::new(right),
            })));
        }
        self.primary()
    }

    /// primary -> "true" | "false" | "nil" | NUMBER | STRING | "(" expression ")"
    fn primary(&mut self) -> Option<Expr> {
        if self.match_ty(&TokenType::KEYWORD(Keyword::TRUE)) {
            return Some(Expr::Literal(Literal::Boolean(true)));
        }

        if self.match_ty(&TokenType::KEYWORD(Keyword::FALSE)) {
            return Some(Expr::Literal(Literal::Boolean(false)));
        }

        if self.match_ty(&TokenType::KEYWORD(Keyword::NIL)) {
            return Some(Expr::Literal(Literal::None));
        }

        if self.match_ty(&TokenType::NUMBER) {
            let token = self.previous();
            if let TokenLiteral::Number(n) = token.literal {
                return Some(Expr::Literal(Literal::Number(n)));
            }
        }

        if self.match_ty(&TokenType::STRING) {
            let token = self.previous();
            if let TokenLiteral::String(ref s) = token.literal {
                return Some(Expr::Literal(Literal::String(s.clone())));
            }
        }

        if self.match_ty(&TokenType::LEFT_PAREN) {
            let expr = self.expression()?;
            if self.match_ty(&TokenType::RIGHT_PAREN) {
                return Some(Expr::Grouping(Box::new(expr)));
            }
            self.report("Expected ')' after expression.");
        }

        self.report("Expected expression.");

        None
    }

    #[allow(dead_code)]
    fn sync(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().ty == TokenType::SEMICOLON {
                return;
            }
            match self.peek().ty {
                TokenType::KEYWORD(Keyword::CLASS)
                | TokenType::KEYWORD(Keyword::FUN)
                | TokenType::KEYWORD(Keyword::VAR)
                | TokenType::KEYWORD(Keyword::FOR)
                | TokenType::KEYWORD(Keyword::IF)
                | TokenType::KEYWORD(Keyword::WHILE)
                | TokenType::KEYWORD(Keyword::PRINT)
                | TokenType::KEYWORD(Keyword::RETURN) => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn report(&mut self, message: &str) {
        let token = self.peek();
        if token.ty == TokenType::EOF {
            self.reporter
                .report_with_local(token.line, message.to_string(), "at end".to_string());
        } else {
            self.reporter.report_with_local(
                token.line,
                message.to_string(),
                format!(" at '{}'", token.lexeme),
            );
        }
    }
}

impl Process for Parser {
    type Input = Vec<Token>;
    type Output = Option<Expr>;

    fn run(&mut self, input: Self::Input) -> Self::Output {
        self.tokens = input;
        let expr = self.expression();
        self.reporter.print();
        expr
    }

    fn had_error(&self) -> bool {
        self.reporter.had_error()
    }
}

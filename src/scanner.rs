use crate::token::Token;
use crate::token_type::TokenType;
use crate::Lox;

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    source: String,
    pub tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner<'_> {
    pub const KEYWORDS: [(&'static str, TokenType); 16] = [
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ];

    pub fn new<'a>(lox: &'a mut Lox, source: &str) -> Scanner<'a> {
        Scanner {
            lox: lox,
            source: String::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, String::from(""))
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: String) {
        let text = self
            .source
            .chars()
            .skip(self.start as usize)
            .take(self.current as usize)
            .collect();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            "".to_string(),
            self.line,
        ));
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '\"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox.error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self
            .source
            .chars()
            .skip(self.start as usize)
            .take(self.current as usize)
            .collect();

        for keyword in Scanner::KEYWORDS.iter() {
            if keyword.0 == text {
                self.add_token(keyword.1);
                return;
            }
        }
        self.add_token(TokenType::Identifier);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let text = self
            .source
            .chars()
            .skip(self.start as usize)
            .take(self.current as usize)
            .collect();
        self.add_token_literal(TokenType::Number, text);
    }

    fn string(&mut self) {
        while self.peek() != '\"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.lox.error(self.line, "Unterminated string.");
            return;
        }

        // The closing "
        self.advance();

        // Trimmed the surrounding quotes
        let text = self
            .source
            .chars()
            .skip(self.start as usize)
            .take(self.current as usize)
            .collect();
        self.add_token_literal(TokenType::String, text);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() == expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) as usize >= self.source.len() {
            return '\0';
        }

        self.source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap()
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}

use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String, // TODO: accommodate number literal
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?} {} {}", self.token_type, self.lexeme, self.literal);
    }
}

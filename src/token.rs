#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenType {
    // Literals
    Number,
    String,
    Identifier,
    True,
    False,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    
    // Comparison operators
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    EqualEqual,
    BangEqual,
    
    // Logical operators
    And,
    Or,
    Not,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Dot,
    
    // Keywords
    Fn,
    Return,
    If,
    Else,
    For,
    In,
    While,
    Transformer,
    Use,
    
    // End of file
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    #[allow(dead_code)]
    pub fn repr(&self) -> String {
        format!("{:?} '{}'", self.token_type, self.literal)
    }
}
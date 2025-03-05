use std::io;

use crate::token::{Token, TokenType};

pub struct Lexer {
    pub line: Result<Vec<String>, io::Error>,
}

impl Lexer {
    pub fn new(content: &str) -> Self {
        Lexer {
            line: Ok(vec![content.to_string()]),
        }
    }
    
    pub fn tokenize(&self) -> Result<Vec<Token>, String> {
        Ok(self.lex())
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        match &self.line {
            Ok(lines) => {
                for line in lines {
                    let mut chars = line.chars().peekable();
                    let mut _position = 0;
                    
                    while let Some(c) = chars.next() {
                        match c {
                            ' ' | '\t' | '\r' | '\n' => {
                                // Skip whitespace
                                _position += 1;
                            },
                            // Digits
                            '0'..='9' => {
                                let mut number = c.to_string();
                                
                                // Consume all consecutive digits
                                while let Some(&next_c) = chars.peek() {
                                    if next_c.is_digit(10) || next_c == '.' {
                                        number.push(chars.next().unwrap());
                                    } else {
                                        break;
                                    }
                                }
                                
                                tokens.push(Token {
                                    token_type: TokenType::Number,
                                    literal: number.clone(),
                                });
                                
                                _position += number.len();
                            },
                            // String literals
                            '"' => {
                                let mut string = String::new();
                                
                                // Consume all characters until the closing quote
                                while let Some(&next_c) = chars.peek() {
                                    chars.next();
                                    if next_c == '"' {
                                        break;
                                    }
                                    string.push(next_c);
                                }
                                
                                tokens.push(Token {
                                    token_type: TokenType::String,
                                    literal: string.clone(),
                                });
                                
                                _position += string.len() + 2; // +2 for the quotes
                            },
                            // Identifiers and keywords
                            'a'..='z' | 'A'..='Z' | '_' => {
                                let mut identifier = c.to_string();
                                
                                // Consume all consecutive alphanumeric characters
                                while let Some(&next_c) = chars.peek() {
                                    if next_c.is_alphanumeric() || next_c == '_' {
                                        identifier.push(chars.next().unwrap());
                                    } else {
                                        break;
                                    }
                                }
                                
                                // Check if it's a keyword
                                let token_type = match identifier.as_str() {
                                    "fn" => TokenType::Fn,
                                    "return" => TokenType::Return,
                                    "true" => TokenType::True,
                                    "false" => TokenType::False,
                                    "if" => TokenType::If,
                                    "else" => TokenType::Else,
                                    "for" => TokenType::For,
                                    "in" => TokenType::In,
                                    "while" => TokenType::While,
                                    "transformer" => TokenType::Transformer,
                                    "and" => TokenType::And,
                                    "or" => TokenType::Or,
                                    "not" => TokenType::Not,
                                    "use" => TokenType::Use,
                                    _ => TokenType::Identifier,
                                };
                                
                                tokens.push(Token {
                                    token_type,
                                    literal: identifier.clone(),
                                });
                                
                                _position += identifier.len();
                            },
                            // Operators and delimiters
                            '+' => {
                                tokens.push(Token {
                                    token_type: TokenType::Plus,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '-' => {
                                tokens.push(Token {
                                    token_type: TokenType::Minus,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '*' => {
                                tokens.push(Token {
                                    token_type: TokenType::Multiply,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '/' => {
                                // Check if it's a comment
                                if chars.peek() == Some(&'/') {
                                    // Consume the second '/'
                                    chars.next();
                                    
                                    // Consume the rest of the line
                                    while let Some(c) = chars.next() {
                                        if c == '\n' {
                                            break;
                                        }
                                    }
                                    
                                    _position += 1;
                                } else {
                                    tokens.push(Token {
                                        token_type: TokenType::Divide,
                                        literal: c.to_string(),
                                    });
                                    
                                    _position += 1;
                                }
                            },
                            '%' => {
                                tokens.push(Token {
                                    token_type: TokenType::Modulo,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '<' => {
                                // Check if it's <= or just <
                                if chars.peek() == Some(&'=') {
                                    chars.next(); // Consume the '='
                                    tokens.push(Token {
                                        token_type: TokenType::LessThanEqual,
                                        literal: "<=".to_string(),
                                    });
                                    _position += 2;
                                } else {
                                    tokens.push(Token {
                                        token_type: TokenType::LessThan,
                                        literal: c.to_string(),
                                    });
                                    _position += 1;
                                }
                            },
                            '>' => {
                                // Check if it's >= or just >
                                if chars.peek() == Some(&'=') {
                                    chars.next(); // Consume the '='
                                    tokens.push(Token {
                                        token_type: TokenType::GreaterThanEqual,
                                        literal: ">=".to_string(),
                                    });
                                    _position += 2;
                                } else {
                                    tokens.push(Token {
                                        token_type: TokenType::GreaterThan,
                                        literal: c.to_string(),
                                    });
                                    _position += 1;
                                }
                            },
                            '=' => {
                                // Check if it's == or just =
                                if chars.peek() == Some(&'=') {
                                    chars.next(); // Consume the second '='
                                    tokens.push(Token {
                                        token_type: TokenType::EqualEqual,
                                        literal: "==".to_string(),
                                    });
                                    _position += 2;
                                } else {
                                    tokens.push(Token {
                                        token_type: TokenType::Equal,
                                        literal: c.to_string(),
                                    });
                                    _position += 1;
                                }
                            },
                            '!' => {
                                // Check if it's != or just !
                                if chars.peek() == Some(&'=') {
                                    chars.next(); // Consume the '='
                                    tokens.push(Token {
                                        token_type: TokenType::BangEqual,
                                        literal: "!=".to_string(),
                                    });
                                    _position += 2;
                                } else {
                                    // Just skip the character for now
                                    _position += 1;
                                }
                            },
                            '(' => {
                                tokens.push(Token {
                                    token_type: TokenType::LeftParen,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            ')' => {
                                tokens.push(Token {
                                    token_type: TokenType::RightParen,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '[' => {
                                tokens.push(Token {
                                    token_type: TokenType::LeftBracket,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            ']' => {
                                tokens.push(Token {
                                    token_type: TokenType::RightBracket,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '{' => {
                                tokens.push(Token {
                                    token_type: TokenType::LeftBrace,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '}' => {
                                tokens.push(Token {
                                    token_type: TokenType::RightBrace,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            ',' => {
                                tokens.push(Token {
                                    token_type: TokenType::Comma,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            ';' => {
                                tokens.push(Token {
                                    token_type: TokenType::Semicolon,
                                    literal: c.to_string(),
                                });
                                _position += 1;
                            },
                            '.' => {
                                tokens.push(Token {
                                    token_type: TokenType::Dot,
                                    literal: ".".to_string(),
                                });
                                _position += 1;
                            },
                            _ => {
                                // Ignore unrecognized characters
                                _position += 1;
                            }
                        }
                    }
                }
            },
            Err(_) => {
                // Handle error
            }
        }
        
        // Add EOF token
        tokens.push(Token {
            token_type: TokenType::EOF,
            literal: String::new(),
        });
        
        tokens
    }
}
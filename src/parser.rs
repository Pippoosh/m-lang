use crate::token::{Token, TokenType};
use crate::ast::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let mut expressions = Vec::new();
        
        while !self.is_at_end() {
            expressions.push(self.statement()?);
            
            // Allow optional semicolons between expressions
            self.match_tokens(&[TokenType::Semicolon]);
        }
        
        // If there's only one expression, return it directly
        if expressions.len() == 1 {
            Ok(expressions.remove(0))
        } else {
            // Otherwise, create a block expression
            Ok(Expr::Block(expressions))
        }
    }

    fn statement(&mut self) -> Result<Expr, String> {
        // Check for function definition
        if self.match_tokens(&[TokenType::Fn]) {
            return self.function_definition();
        }

        // Check for transformer definition
        if self.match_tokens(&[TokenType::Transformer]) {
            return self.transformer_definition();
        }

        // Check for use statement
        if self.match_tokens(&[TokenType::Use]) {
            return self.use_statement();
        }

        // Check for return statement
        if self.match_tokens(&[TokenType::Return]) {
            return self.return_statement();
        }

        // Check for if statement
        if self.match_tokens(&[TokenType::If]) {
            return self.if_statement();
        }

        // Check for for loop
        if self.match_tokens(&[TokenType::For]) {
            return self.for_loop();
        }
        
        // Check for while loop
        if self.match_tokens(&[TokenType::While]) {
            return self.while_loop();
        }

        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.logical_and()?;

        while self.match_tokens(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.assignment()?;

        while self.match_tokens(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.assignment()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?;

        if self.match_tokens(&[TokenType::Equal]) {
            let value = Box::new(self.assignment()?);

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign { name, value });
            }

            return Err("Invalid assignment target".to_string());
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::EqualEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenType::LessThan,
            TokenType::LessThanEqual,
            TokenType::GreaterThan,
            TokenType::GreaterThanEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Multiply, TokenType::Divide, TokenType::Modulo]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Minus, TokenType::Not]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_tokens(&[TokenType::LeftBracket]) {
                let index = self.expression()?;
                self.consume(TokenType::RightBracket, "Expected ']' after index")?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_tokens(&[TokenType::Dot]) {
                // Handle dot notation for applying transformers
                if self.match_tokens(&[TokenType::Identifier]) {
                    let transformer_name = self.previous().literal.clone();
                    
                    // Parse arguments
                    self.consume(TokenType::LeftParen, "Expected '(' after transformer name")?;
                    let arguments = self.arguments()?;
                    self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
                    
                    expr = Expr::Apply {
                        object: Box::new(expr),
                        transformer: transformer_name,
                        arguments,
                    };
                } else {
                    return Err("Expected identifier after '.'".to_string());
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let arguments = self.arguments()?;
        self.consume(TokenType::RightParen, "Expected ')' after arguments")?;

        match callee {
            Expr::Variable(name) => Ok(Expr::Call { callee: name, arguments }),
            _ => Err("Expected function name".to_string()),
        }
    }

    fn arguments(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        if !self.check(TokenType::RightParen) {
            // Parse first argument
            args.push(self.expression()?);

            // Parse remaining arguments
            while self.match_tokens(&[TokenType::Comma]) {
                args.push(self.expression()?);
            }
        }

        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Number]) {
            let value = self.previous().literal.parse::<f64>().unwrap();
            return Ok(Expr::Number(value));
        }

        if self.match_tokens(&[TokenType::String]) {
            let value = self.previous().literal.clone();
            return Ok(Expr::String(value));
        }

        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Boolean(true));
        }

        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Boolean(false));
        }

        if self.match_tokens(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().literal.clone()));
        }

        if self.match_tokens(&[TokenType::LeftBracket]) {
            return self.array();
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(expr);
        }

        if self.match_tokens(&[TokenType::Return]) {
            return self.return_statement();
        }

        Err("Expected expression".to_string())
    }

    fn array(&mut self) -> Result<Expr, String> {
        let mut elements = Vec::new();

        if !self.check(TokenType::RightBracket) {
            // Parse first element
            elements.push(self.expression()?);

            // Parse remaining elements
            while self.match_tokens(&[TokenType::Comma]) {
                elements.push(self.expression()?);
            }
        }

        self.consume(TokenType::RightBracket, "Expected ']' after array elements")?;

        Ok(Expr::Array(elements))
    }

    fn if_statement(&mut self) -> Result<Expr, String> {
        // Parse condition
        let condition = Box::new(self.expression()?);

        // Parse then branch
        self.consume(TokenType::LeftBrace, "Expected '{' after if condition")?;

        let mut then_statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            then_statements.push(self.statement()?);

            // Allow optional semicolons
            self.match_tokens(&[TokenType::Semicolon]);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after then branch")?;

        let then_branch = if then_statements.len() == 1 {
            Box::new(then_statements.remove(0))
        } else {
            Box::new(Expr::Block(then_statements))
        };

        // Parse else branch if present
        let else_branch = if self.match_tokens(&[TokenType::Else]) {
            self.consume(TokenType::LeftBrace, "Expected '{' after else")?;

            let mut else_statements = Vec::new();

            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
                else_statements.push(self.statement()?);

                // Allow optional semicolons
                self.match_tokens(&[TokenType::Semicolon]);
            }

            self.consume(TokenType::RightBrace, "Expected '}' after else branch")?;

            if else_statements.len() == 1 {
                Some(Box::new(else_statements.remove(0)))
            } else {
                Some(Box::new(Expr::Block(else_statements)))
            }
        } else {
            None
        };

        Ok(Expr::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn for_loop(&mut self) -> Result<Expr, String> {
        // Parse variable
        let variable = if self.match_tokens(&[TokenType::Identifier]) {
            self.previous().literal.clone()
        } else {
            return Err("Expected variable name".to_string());
        };

        // Parse iterable
        self.consume(TokenType::In, "Expected 'in' after variable")?;
        let iterable = Box::new(self.expression()?);

        // Parse body
        self.consume(TokenType::LeftBrace, "Expected '{' after iterable")?;

        let mut body = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);

            // Allow optional semicolons
            self.match_tokens(&[TokenType::Semicolon]);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after for loop body")?;

        Ok(Expr::For { variable, iterable, body: Box::new(Expr::Block(body)) })
    }

    fn while_loop(&mut self) -> Result<Expr, String> {
        // Parse condition
        let condition = Box::new(self.expression()?);
        
        // Parse body
        self.consume(TokenType::LeftBrace, "Expected '{' after while condition")?;
        
        let mut body = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
            
            // Allow optional semicolons
            self.match_tokens(&[TokenType::Semicolon]);
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after while loop body")?;
        
        Ok(Expr::While { 
            condition, 
            body: Box::new(Expr::Block(body)) 
        })
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(message.to_string())
        }
    }

    fn function_definition(&mut self) -> Result<Expr, String> {
        // Parse function name
        let name = if self.match_tokens(&[TokenType::Identifier]) {
            self.previous().literal.clone()
        } else {
            return Err("Expected function name".to_string());
        };

        // Parse parameter list
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();

        if !self.check(TokenType::RightParen) {
            // Parse first parameter
            if self.match_tokens(&[TokenType::Identifier]) {
                params.push(self.previous().literal.clone());
            }

            // Parse remaining parameters
            while self.match_tokens(&[TokenType::Comma]) {
                if self.match_tokens(&[TokenType::Identifier]) {
                    params.push(self.previous().literal.clone());
                } else {
                    return Err("Expected parameter name".to_string());
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        // Parse function body
        self.consume(TokenType::LeftBrace, "Expected '{' before function body")?;

        let mut body = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);

            // Allow optional semicolons
            self.match_tokens(&[TokenType::Semicolon]);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after function body")?;

        Ok(Expr::Function { name, params, body })
    }

    fn transformer_definition(&mut self) -> Result<Expr, String> {
        // Parse transformer name
        let name = if self.match_tokens(&[TokenType::Identifier]) {
            self.previous().literal.clone()
        } else {
            return Err("Expected transformer name".to_string());
        };

        // Parse parameters
        self.consume(TokenType::LeftParen, "Expected '(' after transformer name")?;
        
        let mut params = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            loop {
                if self.match_tokens(&[TokenType::Identifier]) {
                    params.push(self.previous().literal.clone());
                } else {
                    return Err("Expected parameter name".to_string());
                }
                
                if !self.match_tokens(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        
        // Parse body
        self.consume(TokenType::LeftBrace, "Expected '{' before transformer body")?;
        
        let mut body = Vec::new();
        
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            body.push(self.statement()?);
            
            // Allow optional semicolons
            self.match_tokens(&[TokenType::Semicolon]);
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after transformer body")?;
        
        Ok(Expr::Transformer { name, params, body })
    }

    fn return_statement(&mut self) -> Result<Expr, String> {
        let value = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(Box::new(self.statement()?))
        };

        // Allow optional semicolon
        self.match_tokens(&[TokenType::Semicolon]);

        Ok(Expr::Return { value })
    }

    fn use_statement(&mut self) -> Result<Expr, String> {
        // Parse the path to import
        if self.match_tokens(&[TokenType::String]) {
            let path = self.previous().literal.clone();
            
            // Allow optional semicolon
            self.match_tokens(&[TokenType::Semicolon]);
            
            Ok(Expr::Use { path })
        } else {
            Err("Expected string path after 'use'".to_string())
        }
    }
}
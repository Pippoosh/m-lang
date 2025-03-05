use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use crate::token::{Token, TokenType};
use crate::lexer::Lexer;
use crate::ast::Expr;
use crate::value::Value;
use crate::environment::Environment;
use crate::parser::Parser;

pub struct Interpreter {
    environment: Environment,
    _globals: Environment,
    imported_files: Vec<String>,
    base_path: Option<PathBuf>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut environment = Environment::new();
        
        // Add built-in functions
        environment.define("print".to_string(), Value::Function {
            params: vec!["message".to_string()],
            body: vec![],
        });
        
        environment.define("range".to_string(), Value::Function {
            params: vec!["start".to_string(), "end".to_string()],
            body: vec![],
        });
        
        let environment = Environment::new_with_enclosing(Some(Box::new(environment)));

        Interpreter {
            environment,
            _globals: Environment::new(),
            imported_files: Vec::new(),
            base_path: None,
        }
    }

    pub fn with_base_path(base_path: &Path) -> Self {
        let mut interpreter = Self::new();
        interpreter.base_path = Some(base_path.to_path_buf());
        interpreter
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(value) => Ok(Value::Number(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Boolean(value) => Ok(Value::Boolean(*value)),
            Expr::Array(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.evaluate(element)?);
                }
                Ok(Value::Array(values))
            },
            Expr::Variable(name) => {
                match self.environment.get(name) {
                    Some(value) => Ok(value),
                    None => Err(format!("Undefined variable: {}", name)),
                }
            },
            Expr::Binary { left, operator, right } => self.evaluate_binary(left, operator, right),
            Expr::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expr::Assign { name, value } => {
                let evaluated_value = self.evaluate(value)?;

                if self.environment.get(name).is_some() {
                    self.environment.assign(name, evaluated_value.clone())?;
                } else {
                    self.environment.define(name.clone(), evaluated_value.clone());
                }

                Ok(evaluated_value)
            },
            Expr::Call { callee, arguments } => self.call(callee, arguments),
            Expr::Function { name, params, body } => {
                // Create function value
                let function = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                };
                
                self.environment.define(name.clone(), function.clone());
                
                Ok(function)
            },
            Expr::Return { value } => {
                match value {
                    Some(expr) => {
                        let result = self.evaluate(expr)?;
                        return Ok(result);
                    },
                    None => {
                        return Ok(Value::Nil);
                    },
                }
            },
            Expr::Index { object, index } => {
                let object_val = self.evaluate(object)?;
                let index_val = self.evaluate(index)?;

                match (object_val, index_val) {
                    (Value::Array(elements), Value::Number(i)) => {
                        let idx = i as usize;
                        if idx < elements.len() {
                            Ok(elements[idx].clone())
                        } else {
                            Err(format!("Index out of bounds: {}", idx))
                        }
                    },
                    _ => Err("Cannot index non-array type".to_string()),
                }
            },
            Expr::Block(expressions) => {
                let mut result = Value::Nil;

                for expr in expressions {
                    result = self.evaluate(expr)?;
                }

                Ok(result)
            },
            Expr::If { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition)?;

                match condition_val {
                    Value::Boolean(true) => self.evaluate(then_branch),
                    Value::Boolean(false) => else_branch.as_ref().map_or(Ok(Value::Nil), |branch| self.evaluate(branch)),
                    _ => Err("Condition must be a boolean value".to_string()),
                }
            },
            Expr::For { variable, iterable, body } => {
                let iterable_val = self.evaluate(iterable)?;

                match iterable_val {
                    Value::Array(elements) => {
                        let mut result = Value::Nil;
                        for element in elements {
                            let mut environment = Environment::new_with_enclosing(Some(Box::new(self.environment.clone())));
                            environment.define(variable.clone(), element.clone());
                            self.environment = environment;
                            result = self.evaluate(body)?;
                            self.environment = *self.environment.enclosing.clone().unwrap();
                        }
                        Ok(result)
                    },
                    Value::String(s) => {
                        // Make strings iterable by character
                        let mut result = Value::Nil;
                        for c in s.chars() {
                            let mut environment = Environment::new_with_enclosing(Some(Box::new(self.environment.clone())));
                            environment.define(variable.clone(), Value::String(c.to_string()));
                            self.environment = environment;
                            result = self.evaluate(body)?;
                            self.environment = *self.environment.enclosing.clone().unwrap();
                        }
                        Ok(result)
                    },
                    _ => Err(format!("Cannot iterate over non-iterable value: {:?}", iterable_val)),
                }
            },
            Expr::While { condition, body } => {
                loop {
                    let condition_val = self.evaluate(condition)?;
                    
                    match condition_val {
                        Value::Boolean(true) => {
                            self.evaluate(body)?;
                        },
                        Value::Boolean(false) => {
                            break;
                        },
                        _ => return Err("Condition must be a boolean value".to_string()),
                    }
                }
                
                Ok(Value::Nil)
            },
            Expr::Transformer { name, params, body } => {
                let transformer = Value::Transformer {
                    params: params.clone(),
                    body: body.clone(),
                };
                
                self.environment.define(name.clone(), transformer.clone());
                
                Ok(transformer)
            },
            Expr::Apply { object, transformer, arguments } => {
                let object_val = self.evaluate(object)?;
                
                // Handle built-in transformers
                match transformer.as_str() {
                    "to_string" => {
                        // Convert any value to a string
                        match object_val {
                            Value::Number(n) => Ok(Value::String(n.to_string())),
                            Value::String(s) => Ok(Value::String(s)),
                            Value::Boolean(b) => Ok(Value::String(if b { "true".to_string() } else { "false".to_string() })),
                            Value::Array(arr) => {
                                let mut result = String::new();
                                for (i, val) in arr.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(", ");
                                    }
                                    match val {
                                        Value::String(s) => result.push_str(s),
                                        _ => result.push_str(&val.to_string()),
                                    }
                                }
                                Ok(Value::String(result))
                            },
                            Value::Function { .. } => Ok(Value::String("[Function]".to_string())),
                            Value::Transformer { .. } => Ok(Value::String("[Transformer]".to_string())),
                            Value::Nil => Ok(Value::String("nil".to_string())),
                        }
                    },
                    "to_number" => {
                        // Convert a value to a number
                        match object_val {
                            Value::Number(n) => Ok(Value::Number(n)),
                            Value::String(s) => {
                                // Try to parse the string as a number
                                match s.parse::<f64>() {
                                    Ok(n) => Ok(Value::Number(n)),
                                    Err(_) => {
                                        // Special cases
                                        if s == "true" {
                                            Ok(Value::Number(1.0))
                                        } else if s == "false" {
                                            Ok(Value::Number(0.0))
                                        } else {
                                            Ok(Value::Number(0.0)) // Default for unparseable strings
                                        }
                                    }
                                }
                            },
                            Value::Boolean(b) => Ok(Value::Number(if b { 1.0 } else { 0.0 })),
                            Value::Array(_) => Ok(Value::Number(0.0)), // Default for arrays
                            Value::Function { .. } => Ok(Value::Number(0.0)),
                            Value::Transformer { .. } => Ok(Value::Number(0.0)),
                            Value::Nil => Ok(Value::Number(0.0)),
                        }
                    },
                    "to_bool" => {
                        // Convert a value to a boolean
                        match object_val {
                            Value::Number(n) => Ok(Value::Boolean(n != 0.0)),
                            Value::String(s) => {
                                // Empty string, "false", and "0" are false, everything else is true
                                Ok(Value::Boolean(!(s.is_empty() || s == "false" || s == "0")))
                            },
                            Value::Boolean(b) => Ok(Value::Boolean(b)),
                            Value::Array(arr) => Ok(Value::Boolean(!arr.is_empty())),
                            Value::Function { .. } => Ok(Value::Boolean(true)),
                            Value::Transformer { .. } => Ok(Value::Boolean(true)),
                            Value::Nil => Ok(Value::Boolean(false)),
                        }
                    },
                    "to_array" => {
                        // Convert a value to an array
                        match object_val {
                            Value::Array(arr) => Ok(Value::Array(arr)),
                            _ => Ok(Value::Array(vec![object_val])),
                        }
                    },
                    "parse_number" => {
                        // Parse a string to a number
                        match object_val {
                            Value::String(s) => {
                                match s.parse::<f64>() {
                                    Ok(n) => Ok(Value::Number(n)),
                                    Err(_) => Ok(Value::Number(0.0)), // Default for unparseable strings
                                }
                            },
                            Value::Number(n) => Ok(Value::Number(n)),
                            _ => Ok(Value::Number(0.0)),
                        }
                    },
                    "parse_bool" => {
                        // Parse a string to a boolean
                        match object_val {
                            Value::String(s) => {
                                Ok(Value::Boolean(s == "true" || s == "1" || s == "yes"))
                            },
                            Value::Boolean(b) => Ok(Value::Boolean(b)),
                            _ => {
                                // Use the to_bool logic for other types
                                match object_val {
                                    Value::Number(n) => Ok(Value::Boolean(n != 0.0)),
                                    Value::Array(arr) => Ok(Value::Boolean(!arr.is_empty())),
                                    Value::Function { .. } => Ok(Value::Boolean(true)),
                                    Value::Transformer { .. } => Ok(Value::Boolean(true)),
                                    Value::Nil => Ok(Value::Boolean(false)),
                                    _ => Ok(Value::Boolean(false)), // Default case
                                }
                            },
                        }
                    },
                    "to_json" => {
                        // Convert a value to its JSON string representation
                        match object_val {
                            Value::String(s) => Ok(Value::String(format!("\"{}\"", s))),
                            Value::Number(n) => Ok(Value::String(n.to_string())),
                            Value::Boolean(b) => Ok(Value::String(if b { "true".to_string() } else { "false".to_string() })),
                            Value::Array(arr) => {
                                let mut result = String::from("[");
                                for (i, val) in arr.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(",");
                                    }
                                    
                                    // Recursively convert each item to JSON
                                    let json_val = match val {
                                        Value::String(s) => format!("\"{}\"", s),
                                        Value::Number(n) => n.to_string(),
                                        Value::Boolean(b) => if *b { "true".to_string() } else { "false".to_string() },
                                        Value::Array(_) => "[...]".to_string(), // Simplified for nested arrays
                                        _ => "null".to_string(),
                                    };
                                    
                                    result.push_str(&json_val);
                                }
                                result.push_str("]");
                                Ok(Value::String(result))
                            },
                            Value::Function { .. } => Ok(Value::String("null".to_string())),
                            Value::Transformer { .. } => Ok(Value::String("null".to_string())),
                            Value::Nil => Ok(Value::String("null".to_string())),
                        }
                    },
                    _ => {
                        // Look up the transformer in the environment
                        if let Some(Value::Transformer { params, body }) = self.environment.get(transformer) {
                            // Create a new environment for the transformer execution
                            let mut env = Environment::new_with_enclosing(Some(Box::new(self.environment.clone())));
                            
                            // Define the special 'applied' variable with the object value
                            env.define("applied".to_string(), object_val.clone());
                            
                            // Define parameters
                            for (i, param) in params.iter().enumerate() {
                                let arg_value = if i < arguments.len() {
                                    self.evaluate(&arguments[i])?
                                } else {
                                    Value::Nil
                                };
                                
                                env.define(param.clone(), arg_value);
                            }
                            
                            // Save the current environment
                            let old_env = self.environment.clone();
                            
                            // Set the new environment
                            self.environment = env;
                            
                            // Execute the transformer body
                            let mut result = Value::Nil;
                            
                            for expr in body.iter() {
                                result = self.evaluate(expr)?;
                                
                                // Handle return statements
                                if let Expr::Return { .. } = expr {
                                    break;
                                }
                            }
                            
                            // Restore the old environment
                            self.environment = old_env;
                            
                            // Update the original object with the result
                            if let Expr::Variable(name) = &**object {
                                self.environment.assign(name, result.clone())?;
                            }
                            
                            Ok(result)
                        } else {
                            Err(format!("Undefined transformer '{}'", transformer))
                        }
                    }
                }
            },
            Expr::Use { path } => {
                // Check if file has already been imported to prevent circular imports
                if self.imported_files.contains(path) {
                    return Ok(Value::Nil); // Skip already imported files
                }
                
                // Add to imported files list
                self.imported_files.push(path.clone());
                
                // Resolve the path
                let file_path = if let Some(base_path) = &self.base_path {
                    base_path.join(path)
                } else {
                    PathBuf::from(path)
                };
                
                // Read the file
                let content = match fs::read_to_string(&file_path) {
                    Ok(content) => content,
                    Err(e) => return Err(format!("Failed to read file '{}': {}", file_path.display(), e)),
                };
                
                // Tokenize
                let lexer = Lexer::new(&content);
                let tokens = match lexer.tokenize() {
                    Ok(tokens) => tokens,
                    Err(e) => return Err(format!("Failed to tokenize file '{}': {}", file_path.display(), e)),
                };
                
                // Parse
                let mut parser = Parser::new(tokens);
                let ast = match parser.parse() {
                    Ok(ast) => ast,
                    Err(e) => return Err(format!("Failed to parse file '{}': {}", file_path.display(), e)),
                };
                
                // Create a new interpreter with the same environment
                let mut file_interpreter = Interpreter {
                    environment: self.environment.clone(),
                    _globals: self._globals.clone(),
                    imported_files: self.imported_files.clone(),
                    base_path: if let Some(base_path) = &self.base_path {
                        Some(base_path.clone())
                    } else {
                        // If the file has a parent directory, use that as the base path
                        file_path.parent().map(|p| p.to_path_buf())
                    },
                };
                
                // Evaluate the imported file
                match file_interpreter.evaluate(&ast) {
                    Ok(_) => {
                        // Copy all variables and functions from the file's environment to our environment
                        for (name, value) in file_interpreter.get_variables() {
                            self.environment.define(name, value);
                        }
                        Ok(Value::Nil)
                    },
                    Err(e) => Err(format!("Error evaluating file '{}': {}", file_path.display(), e)),
                }
            },
        }
    }

    fn evaluate_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Value, String> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator.token_type {
            // Arithmetic operators
            TokenType::Plus => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::String(l.clone() + r)),
                    (Value::String(l), _) => Ok(Value::String(l.clone() + &right_val.to_string())),
                    (_, Value::String(r)) => Ok(Value::String(left_val.to_string() + r)),
                    (Value::Array(l), Value::Array(r)) => {
                        let mut elements = l.clone();
                        elements.extend(r.clone());
                        Ok(Value::Array(elements))
                    },
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::Minus => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::Multiply => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::Divide => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    },
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::Modulo => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => {
                        if *r == 0.0 {
                            Err("Modulo by zero".to_string())
                        } else {
                            // Use the rem_euclid method for proper floating-point modulo
                            let result = l.rem_euclid(*r);
                            Ok(Value::Number(result))
                        }
                    },
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            // Comparison operators
            TokenType::LessThan => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::LessThanEqual => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::GreaterThan => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::GreaterThanEqual => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::EqualEqual => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l == r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Boolean(true)),
                    _ => Ok(Value::Boolean(false)),
                }
            },
            TokenType::BangEqual => {
                match (&left_val, &right_val) {
                    (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l != r)),
                    (Value::String(l), Value::String(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                    (Value::Nil, Value::Nil) => Ok(Value::Boolean(false)),
                    _ => Ok(Value::Boolean(true)),
                }
            },
            // Logical operators
            TokenType::And => {
                match (&left_val, &right_val) {
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(*l && *r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            TokenType::Or => {
                match (&left_val, &right_val) {
                    (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(*l || *r)),
                    _ => Err(format!("Invalid operands for operator: {:?}", operator.token_type)),
                }
            },
            _ => Err(format!("Unknown operator: {:?}", operator.token_type)),
        }
    }

    fn evaluate_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, String> {
        let right_val = self.evaluate(right)?;

        match operator.token_type {
            // Negation
            TokenType::Minus => match right_val {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(format!("Invalid operand for unary operator: {:?}", operator.token_type)),
            },
            // Logical NOT
            TokenType::Not => match right_val {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                _ => Err(format!("Invalid operand for unary operator: {:?}", operator.token_type)),
            },
            _ => Err(format!("Unknown unary operator: {:?}", operator.token_type)),
        }
    }

    fn call(&mut self, callee: &str, arguments: &[Expr]) -> Result<Value, String> {
        // Handle built-in functions
        if callee == "print" {
            if arguments.len() != 1 {
                return Err("print() takes exactly 1 argument".to_string());
            }

            let value = self.evaluate(&arguments[0])?;

            // Print without quotes for strings
            match &value {
                Value::String(s) => println!("{}", s),
                _ => println!("{}", value),
            }

            return Ok(Value::Nil);
        } else if callee == "input" {
            if arguments.len() != 1 {
                return Err("input() takes exactly 1 argument".to_string());
            }

            let prompt = match self.evaluate(&arguments[0])? {
                Value::String(s) => s,
                _ => return Err("Argument to input() must be a string".to_string()),
            };

            // Print the prompt without a newline
            print!("{}", prompt);
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

            // Read user input
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // Trim the trailing newline
                    let input = input.trim_end().to_string();
                    return Ok(Value::String(input));
                },
                Err(e) => return Err(format!("Failed to read input: {}", e)),
            }
        } else if callee == "range" {
            if arguments.len() != 2 {
                return Err("range() takes exactly 2 arguments".to_string());
            }

            let start = match self.evaluate(&arguments[0])? {
                Value::Number(n) => n as i32,
                _ => return Err("First argument to range() must be a number".to_string()),
            };

            let end = match self.evaluate(&arguments[1])? {
                Value::Number(n) => n as i32,
                _ => return Err("Second argument to range() must be a number".to_string()),
            };

            let mut elements = Vec::new();
            for i in start..end {
                elements.push(Value::Number(i as f64));
            }

            return Ok(Value::Array(elements));
        }

        // Look up the function in the environment
        if let Some(Value::Function { params, body }) = self.environment.get(callee) {
            // Create a new environment for the function execution
            let mut env = Environment::new_with_enclosing(Some(Box::new(self.environment.clone())));

            // Define parameters
            for (i, param) in params.iter().enumerate() {
                let arg_value = if i < arguments.len() {
                    self.evaluate(&arguments[i])?
                } else {
                    Value::Nil
                };

                env.define(param.clone(), arg_value);
            }

            // Save the current environment
            let old_env = self.environment.clone();

            // Set the new environment
            self.environment = env;

            // Execute the function body
            let mut result = Value::Nil;

            for expr in body.iter() {
                result = self.evaluate(expr)?;

                // Handle return statements
                if let Expr::Return { .. } = expr {
                    break;
                }
            }

            // Restore the old environment
            self.environment = old_env;

            Ok(result)
        } else {
            Err(format!("Undefined function '{}'", callee))
        }
    }

    #[allow(dead_code)]
    pub fn get_variables(&self) -> HashMap<String, Value> {
        self.environment.values.clone()
    }

    // Commented out unused method
    // pub fn get_variable(&self, name: &str) -> Option<&Value> {
    //     self.environment.values.get(name)
    // }
}
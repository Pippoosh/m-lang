use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::env;

mod token;
mod lexer;
mod parser;
mod ast;
mod value;
mod environment;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn read_file(file_path: &Path, line_index: i32) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    
    if line_index >= 0 && line_index < lines.len() as i32 {
        // Return only the specified line
        Ok(vec![lines[line_index as usize].clone()])
    } else {
        // Return all lines
        Ok(lines)
    }
}

fn main() {
    // Get the current directory to use as the base path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    // Get the file path from command-line arguments or use the default
    let file_path = env::args().nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("main.m"));
    
    println!("Running file: {}", file_path.display());
    
    // Create a single interpreter instance to maintain state across all processing
    let mut interpreter = Interpreter::with_base_path(&current_dir);
    
    // Process the specified file
    process_file(&file_path, -1, &mut interpreter);
}

fn process_file(file_path: &Path, line_index: i32, interpreter: &mut Interpreter) {
    let file_content = read_file(file_path, line_index);
    
    match file_content {
        Ok(lines) => {
            // Process the entire file as a single string
            let file_str = lines.join("\n");
            
            // Create a lexer with the entire file content
            let lexer = Lexer { line: Ok(vec![file_str]) };
            let tokens = lexer.lex();
            
            // Parse the tokens
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(expr) => {
                    // Evaluate the expression using the interpreter
                    match interpreter.evaluate(&expr) {
                        Ok(_) => (), // Don't print the result
                        Err(e) => eprintln!("Error: {}", e),
                    }
                },
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

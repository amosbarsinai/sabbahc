use crate::tokenizer::{Token, TokenType};
use crate::typing::{Type, TYPE_INT32};
use std::process::exit;

#[derive(Clone, Debug)]
pub enum ExpressionType {
    IntLiteral(i32),
}

#[derive(Clone, Debug)]
pub enum AstNodeType {
    ExitStatement,
    Expression(Type, ExpressionType)
}

#[derive(Debug)]
pub struct Ast {
    pub children: Vec<AstNodeType>,
}

impl Ast {
    pub fn new() -> Self {
        Ast { children: Vec::new() }
    }
    pub fn add_child(&mut self, child: AstNodeType) {
        self.children.push(child);
    }
}

pub struct Parser {
    input: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Parser { input, index: 0 }
    }
    fn peek(&self) -> Option<&Token> {
        self.input.get(self.index)
    }
    fn consume(&mut self) -> Option<&Token> {
        if self.index < self.input.len() {
            let token = &self.input[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
    pub fn parse(&mut self) -> Vec<Ast> {
        let mut parsed = Vec::new();
        
        while let Some(token) = self.consume() {
            let mut statement = Ast::new();
            
            match token.token_type {
                TokenType::ExitStatement => {
                    statement.add_child(AstNodeType::ExitStatement);
                    if let Some(next_token) = self.consume() {
                        match next_token.token_type {
                            TokenType::IntegerLiteral => {
                                if let Some(ref val_str) = next_token.value {
                                    if let Ok(value) = val_str.parse::<i32>() {
                                        statement.add_child(AstNodeType::Expression(
                                            TYPE_INT32.clone(),
                                            ExpressionType::IntLiteral(value)
                                        ));
                                    }
                                }
                            }
                            TokenType::Semicolon => {
                                statement.add_child(AstNodeType::Expression(
                                    TYPE_INT32.clone(),
                                    ExpressionType::IntLiteral(0),
                                ));
                            }
                            _ => {
                                println!("ERROR: Expected integer literal or semicolon after exit statement");
                                exit(1);
                            }
                        }

                        // Expect the semicolon if we haven't already handled it
                        if next_token.token_type != TokenType::Semicolon {
                            if let Some(semi) = self.consume() {
                                if semi.token_type != TokenType::Semicolon {
                                    println!("ERROR: Expected semicolon after exit statement");
                                    exit(1);
                                }
                            } else {
                                println!("ERROR: Missing semicolon after exit statement");
                                exit(1);
                            }
                        }

                        parsed.push(statement);
                        continue;
                    }
                    else if let Some(next_token) = self.peek() {
                        if let TokenType::Semicolon = next_token.token_type {
                            statement.add_child(AstNodeType::Expression(
                                TYPE_INT32.clone(),
                                ExpressionType::IntLiteral(0)
                            ));
                        } else {
                            println!("ERROR: Expected integer literal after exit statement");
                            exit(1);
                        }
                    }
                    if let Some(next_token) = self.consume() {
                        if let TokenType::Semicolon = next_token.token_type {
                            parsed.push(statement);
                            continue;
                        }
                    }
                    println!("ERROR: Invalid exit statement syntax");
                    exit(1);
                }
                _ => {
                    println!("ERROR: Unexpected token: {:?}", token);
                    exit(1);
                }
            }
        }
        parsed
    }
}
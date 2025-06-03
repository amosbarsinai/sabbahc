use crate::tokenizer::{Token, TokenType};
use crate::typing::{Type, TYPE_INT32};
use crate::Diagnostic;
use std::process::exit;

#[derive(Clone, Debug)]
pub enum ExpressionType {
    IntLiteral(i32),
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression_type: ExpressionType,
    pub eval_type: Type,
}

#[derive(Clone, Debug)]
pub enum AstNodeType {
    ExitStatement(Expression)
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
    filename: String,
}

impl Parser {
    pub fn new(input: Vec<Token>, filename: String) -> Self {
        Parser { input, index: 0, filename }
    }
    fn peek(&self) -> Option<Token> {
        self.input.get(self.index).cloned()
    }
    fn consume(&mut self) -> Option<Token> {
        if self.index < self.input.len() {
            let token = &self.input[self.index];
            self.index += 1;
            Some(token.clone())
        } else {
            None
        }
    }
    pub fn parse(&mut self, source_code: &String) -> Vec<Ast> {
        let mut parsed = Vec::new();
        
        while let Some(token) = self.consume() {
            let mut statement = Ast::new();
            let mut exit_code: Option<i32> = None;
            match token.token_type {
                TokenType::ExitKeyword => {
                    if self.peek().is_none() {
                        Diagnostic {
                            message: "Exit keyword is expected to be followed by an expression or semicolon".to_string(),
                            file: self.filename.clone(),
                            line: token.line,
                            column: token.column,
                            suggestion: None,
                        }.out(source_code);
                        exit(1);
                    }
                    let next_token = self.consume().unwrap();
                    match next_token.token_type {
                        TokenType::IntegerLiteral => {
                            let value: i32 = next_token.value.unwrap().parse().unwrap();
                            exit_code = Some(value);
                            if self.peek().is_none() || self.peek().unwrap().token_type != TokenType::Semicolon {
                                Diagnostic {
                                    message: "full exit statement must end with a semicolon".to_string(),
                                    file: self.filename.clone(),
                                    line: next_token.line,
                                    column: next_token.column,
                                    suggestion: None,
                                }.out(source_code);
                                exit(1);
                            }
                            self.consume();
                            statement.add_child(AstNodeType::ExitStatement(Expression {
                                expression_type: ExpressionType::IntLiteral(value),
                                eval_type: TYPE_INT32.clone(),
                            }));
                            parsed.push(statement);
                        }
                        TokenType::Semicolon => {
                            exit_code = Some(0);
                            statement.add_child(AstNodeType::ExitStatement(Expression {
                                expression_type: ExpressionType::IntLiteral(exit_code.unwrap_or(0)),
                                eval_type: TYPE_INT32.clone(),
                            }));
                            parsed.push(statement);
                        }
                        _ => {
                            Diagnostic {
                                message: "Exit keyword must be followed by an integer literal or semicolon".to_string(),
                                file: self.filename.clone(),
                                line: next_token.line,
                                column: next_token.column,
                                suggestion: None,
                            }.out(source_code);
                            exit(1);
                        }
                    }
                }
                _ => {}
            }
        }
        parsed
    }
}
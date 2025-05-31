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
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Parser { input, index: 0 }
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
    pub fn parse(&mut self, filename: String) -> Vec<Ast> {
        let mut parsed = Vec::new();
        
        while let Some(token) = self.consume() {
            let mut statement = Ast::new();
            match token.token_type {
                TokenType::ExitStatement => {
                    let mut code: Option<i32> = None;

                    let next_token = self.consume();
                    match next_token {
                        None => {
                            Diagnostic {
                                file: filename.clone(),
                                line: token.line,
                                column: token.column,
                                message: String::from("ExitStatement token is expected to be followed by an Expression token or a Semicolon token"),
                                suggestion: Some(String::from("add a semicolon at the end of the statement")),
                                code_snippet: None
                            }.out();
                            exit(1);
                        }
                        Some(ref t) if t.token_type == TokenType::IntegerLiteral => {
                            code = t.value.as_ref()
                                .and_then(|v| v.parse::<i32>().ok());
                            statement.add_child(AstNodeType::ExitStatement(Expression {
                                expression_type: ExpressionType::IntLiteral(code.unwrap()),
                                eval_type: TYPE_INT32.clone(),
                            }));
                            if let Some(next) = self.peek() {
                                if next.token_type == TokenType::Semicolon {
                                    self.consume(); // consume the semicolon
                                    parsed.push(statement);
                                    statement = Ast::new();
                                    continue;
                                } else {
                                    Diagnostic {
                                        file: filename.clone(),
                                        line: next.line,
                                        column: next.column,
                                        message: String::from("Expected a semicolon after exit statement"),
                                        suggestion: Some(String::from("add a semicolon at the end of the statement")),
                                        code_snippet: None
                                    }.out();
                                    exit(1);
                                }
                            } else {
                                Diagnostic {
                                    file: filename.clone(),
                                    line: token.line,
                                    column: token.column,
                                    message: String::from("ExitStatement token is expected to be followed by a Semicolon token"),
                                    suggestion: Some(String::from("add a semicolon at the end of the statement")),
                                    code_snippet: None
                                }.out();
                                exit(1);
                            }
                        }
                        Some(ref t) if t.token_type == TokenType::Semicolon => {
                            statement.add_child(AstNodeType::ExitStatement(Expression {
                                expression_type: ExpressionType::IntLiteral(0),
                                eval_type: TYPE_INT32.clone(),
                            }));
                            parsed.push(statement);
                            statement = Ast::new();
                            continue;
                        }
                        _ => {}
                    }
                }
                TokenType::Semicolon => {
                    parsed.push(statement);
                    statement = Ast::new();
                }
                _ => {
                    Diagnostic {
                        file: filename.clone(),
                        line: token.line,
                        column: token.column,
                        message: format!("Unexpected token: {:?}", token.token_type),
                        suggestion: Some(String::from("Check the syntax of your code")),
                        code_snippet: None
                    }.out();
                    exit(1);
                }
            }
        }                
        parsed
    }
}
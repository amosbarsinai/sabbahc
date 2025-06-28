use crate::{
    err::ErrorHandler, structure::*, tokenizer::{Token, TokenType, TokenValue}, typing::UINT8
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parser<'a> {
    input: &'a [Token<'a>],
    index: u64,
    error_handler: &'a ErrorHandler
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token<'a>], error_handler: &'a ErrorHandler) -> Self {
        Self { input, index: 0, error_handler }
    }
    fn peek(&self) -> Option<Token<'a>> {
        self.input.get(self.index as usize).cloned()
    }
    pub fn parse<'b>(&mut self) -> Scope<'b> where 'a: 'b {
        let mut parsed = Scope {
            children: Vec::new()
        };
        let mut current = Statement::new();
        loop {
            let token = self.input.get(self.index as usize).cloned();
            if token.is_none() {
                break;
            }
            let token = token.unwrap();
            self.index += 1;
            match token.token_type {
                TokenType::FunctionKeyword => {current.push(AstNodeType::fk());}
                TokenType::FunctionIdent => {current.push(AstNode::fi(token.get_funcid()));}
                TokenType::OpenParen => {
                    // Assume function parameter tuple
                    // Functions do not yet have parameters
                    let next_token = self.input.get(self.index as usize).cloned();
                    self.index += 1;
                    if let Some(some_token) = next_token {
                        if let TokenType::CloseParen = some_token.token_type {
                            current.push(AstNode::tup());
                        }
                        else {
                            self.error_handler.err(
                                some_token.line,
                                some_token.column,
                                format!("Expected closing parentheses (found {})", some_token),
                                None,
                            )
                        }
                    }
                }
                TokenType::CloseParen => {
                    self.error_handler.err(
                        token.line,
                        token.column,
                        String::from("Unexpected closing parentheses"),
                        None,
                    )
                }
                TokenType::ThinArrow => {
                    // Expect following TypeIdent
                    let next_token = self.peek();
                    if let Some(some_token) = next_token {
                        if let TokenType::TypeIdent = some_token.token_type {
                            if let TokenValue::TypeIdent(ident) = some_token.value.unwrap() {
                                current.push(AstNode::ti(ident));
                                self.index += 1;
                            }
                        } else {
                            self.error_handler.err(
                                some_token.line,
                                some_token.column,
                                String::from("Expected type identifier after ->"),
                                None
                            );
                        }
                    } else {
                        self.error_handler.err(
                            token.line,
                            token.column,
                            String::from("Unexpected EOF (expected type identifier after ->)"),
                            None,
                        )
                    }
                }
                TokenType::TypeIdent => {
                    self.error_handler.err(
                        token.line,
                        token.column,
                        String::from("Unexpected type identifier"),
                        None,
                    )
                }
                TokenType::OpenCurly => {
                    let mut depth = 1;
                    let start_index = self.index as usize;
                    while depth > 0 {
                        if let Some(token) = self.input.get(self.index as usize) {
                            match token.token_type {
                                TokenType::OpenCurly => depth += 1,
                                TokenType::CloseCurly => depth -= 1,
                                _ => {}
                            }
                            self.index += 1;
                        } else {
                            self.error_handler.err(
                                token.line,
                                token.column,
                                String::from("Unexpected EOF while parsing scope"),
                                None,
                            );
                            break;
                        }
                    }
                    // Exclude the last CloseCurly
                    let end_index: usize = self.index as usize - 1;
                    let scope_tokens_slice: &'b [Token<'b>] = &self.input[start_index..end_index];
                    let mut inner_parser = Parser::<'b>::new(scope_tokens_slice, self.error_handler);
                    let inner_scope = inner_parser.parse();
                    current.push(AstNode::scope(inner_scope.clone()));

                    // Don't handle pro-scope tokens yet
                    parsed.children.push(current);
                    current = Statement::new();
                }
                TokenType::ReturnKeyword => {
                    let next_token = self.peek();
                    if let Some(some_token) = next_token {
                        if let TokenType::IntLiteral = some_token.token_type {
                            if let TokenValue::IntLiteral(value) = some_token.value.unwrap() {
                                current.push(AstNode::ret(value as u8));
                                self.index += 1;
                            }
                        } else {
                            self.error_handler.err(
                                some_token.line,
                                some_token.column,
                                String::from("Expected integer literal after return keyword"),
                                None,
                            );
                        }
                    } else {
                        self.error_handler.err(
                            token.line,
                            token.column,
                            String::from("Unexpected EOF (expected statement end after return)"),
                            None,
                        );
                    }
                }
                TokenType::Semicolon => {
                    parsed.children.push(current);
                    current = Statement::new();
                }
                _ => {}
            }
        }
        parsed
    }
}
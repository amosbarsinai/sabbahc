use crate::{
    err::Diagnostic,
    structure::*,
    tokenizer::{Token, TokenType, TokenValue}, typing::INT8,
};
use std::process::exit;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parser {
    input: Vec<Token>,
    source_code: String,
}

impl Parser {
    pub fn new(input: Vec<Token>, source_code: String) -> Self {
        Self { input, source_code }
    }

    fn peek(&self, index: usize) -> Option<&Token> {
        self.input.get(index)
    }

    fn parse_slice(&self, start: usize, end: usize, outer: bool, filename: &str) -> Scope {
        let mut parsed = Scope {
            children: Vec::new(),
            is_outer: outer,
        };

        let mut index = start;
        while index < end {
            let mut current = Statement::new();
            let token = match self.input.get(index) {
                Some(t) => t,
                None => break,
            };

            match token.token_type {
                TokenType::OpenCurly => {
                    let mut depth = 1;
                    let mut close_index = index + 1;
                    while close_index < end && depth > 0 {
                        match self.input[close_index].token_type {
                            TokenType::OpenCurly => depth += 1,
                            TokenType::CloseCurly => depth -= 1,
                            _ => {}
                        }
                        close_index += 1;
                    }

                    if depth != 0 {
                        Diagnostic {
                            file: filename.to_string(),
                            line: token.line,
                            column: token.column,
                            message: "Unmatched '{'".to_string(),
                            suggestion: None,
                        }
                        .out(&self.source_code);
                        exit(1);
                    }

                    // Recursively parse inner scope
                    let inner_scope = self.parse_slice(index + 1, close_index - 1, false, filename);

                    index = close_index; // Continue after the matching '}'

                    current.add_child(
                        AstNode {
                            node_type: AstNodeType::Scope,
                            value: Some(
                                AstNodeValue::Scope(
                                    inner_scope
                                )
                            )
                        }
                    )
                }
                TokenType::CloseCurly => {
                    Diagnostic {
                        file: filename.to_string(),
                        line: token.line,
                        column: token.column,
                        message: format!("Unmatched '}}'"),
                        suggestion: None,
                    }.out(&self.source_code);
                    exit(1);
                }
                TokenType::FunctionKeyword => {
                    current.add_child(AstNode { node_type: AstNodeType::FunctionKeyword, value: None })
                }
                TokenType::ReturnKeyword => {
                    current.add_child(AstNode { node_type: AstNodeType::ReturnKeyword, value: None });
                }
                TokenType::IntLiteral => {
                    if let TokenValue::IntLiteral(value) = token.clone().value.unwrap() {
                        if value >= u8::MIN as u64 && value <= u8::MAX as u64 {
                            current.add_child(
                                AstNode {
                                    node_type: AstNodeType::Expression,
                                    value: Some(
                                        AstNodeValue::Expression(
                                            Expression {
                                                eval_type: INT8.to_owned(),
                                                content: vec![
                                                    ExpressionContent::UnsignedInt8Literal(
                                                        value as u8
                                                    )
                                                ]
                                            }
                                        )
                                    )
                                }
                            );
                        } else {
                            Diagnostic {
                                file: filename.to_string(),
                                line: token.line,
                                column: token.column,
                                message: String::from("Integer values are capped at 255"),
                                suggestion: None,
                            }.out(&self.source_code);
                            exit(1);
                        }
                    }
                }
                TokenType::FunctionIdent => {
                    if let Some(TokenValue::FunctionIdent(ident)) = &token.value {
                        current.add_child(AstNode {
                            node_type: AstNodeType::FunctionIdent,
                            value: Some(AstNodeValue::FunctionIdent(ident.clone()))
                        });
                    }
                    else {
                        Diagnostic {
                            file: filename.to_string(),
                            line: token.line,
                            column: token.column,
                            message: format!("Something went wrong with the compiler. FunctionIdent value is null"),
                            suggestion: None,
                        }.out(&self.source_code);
                        exit(1);
                    }
                }
                TokenType::OpenParen | TokenType::CloseParen => {
                    // i didn't implement signatures yet
                }
                TokenType::ThinArrow => {
                    // for return types
                }
                TokenType::TypeIdent => { // function return type
                    if let Some(TokenValue::TypeIdent(ident)) = &token.value {
                        current.add_child(AstNode {
                            node_type: AstNodeType::FunctionReturnTypeAnnotation,
                            value: Some(AstNodeValue::FunctionReturnTypeAnnotation(ident.clone()))
                        });
                    }
                }
                TokenType::Semicolon => {
                    parsed.children.push(current);
                }
                _ => {
                    Diagnostic {
                        file: filename.to_string(),
                        line: token.line,
                        column: token.column,
                        message: format!("Unrecognized token: {:?}", token),
                        suggestion: None,
                    }.out(&self.source_code);
                    exit(1);
                }
            }
            index += 1;
        }
        parsed
    }

    pub fn parse(&self, filename: String) -> Scope {
        self.parse_slice(0, self.input.len(), true, &filename)
    }
}

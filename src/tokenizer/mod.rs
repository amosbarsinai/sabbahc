use std::process::exit;

use crate::err::Diagnostic;
use crate::typing::{Type, BUILTIN_TYPES};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenType {
    FunctionKeyword,
    FunctionIdent,
    ThinArrow,
    TypeIdent,
    ParemeterTuple,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    ReturnKeyword,
    IntLiteral,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenValue {
    FunctionIdent(String),
    TypeIdent(Type),
    IntLiteral(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub token_type: TokenType,
    pub value: Option<TokenValue>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tokenizer {
    input: String,
    index: u64,
    ln: usize,
    cl: usize,
    filename: String,
}

impl Tokenizer {
    pub fn new(input: &String, filename: String) -> Self {
        Tokenizer {
            input: input.clone(),
            index: 0,
            ln: 1,
            cl: 1,
            filename,
        }
    }
    fn peek(&self, index: u8) -> Option<char> {
        let peek_index = self.index + index as u64;
        if peek_index >= self.input.len() as u64 {
            return None;
        }
        let char = self.input.chars().nth(peek_index as usize).unwrap();
        Some(char)
    }
    fn consume(&mut self, index: u8) -> Option<char> {
        let mut consume_index = self.index + index as u64;
        if consume_index >= self.input.len() as u64 {
            return None;
        }
        let char = self.input.chars().nth(consume_index as usize).unwrap();
        consume_index += 1;
        self.index = consume_index;
        if char == '\n' {
            self.cl = 1;
            self.ln += 1;
        } else {
            self.cl += 1;
        }
        Some(char)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut func_idents: Vec<String> = Vec::new();
        let type_idents: Vec<String> = BUILTIN_TYPES.keys().cloned().collect(); // I didn't implement struct creation yet - no need for the variable to be mutable
        let source_code = &self.input.clone();
        let mut tokens: Vec<Token> = Vec::new();

        let mut expected: Option<TokenType> = None;

        while self.index < self.input.len() as u64 {
            let char = self.consume(0);
            if char.is_none() {
                break;
            }
            let char = char.unwrap();
            if char.is_alphabetic() {
                let mut current = String::new();
                let start: usize = self.cl.clone() as usize;
                current.push(char);
                while let Some(next_char) = self.peek(0) {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        current.push(self.consume(0).unwrap());
                    } else {
                        break;
                    }
                }
                match current.as_str() {
                    "f" => {
                        tokens.push(Token {
                            line: self.ln,
                            column: start,
                            token_type: TokenType::FunctionKeyword,
                            value: None,
                        });
                        expected = Some(TokenType::FunctionIdent);
                    }
                    "return" => {
                        tokens.push(Token {
                            line: self.ln,
                            column: start,
                            token_type: TokenType::ReturnKeyword,
                            value: None,
                        });
                    }
                    _ => {
                        if expected == Some(TokenType::FunctionIdent) {
                            tokens.push(Token {
                                line: self.ln,
                                column: start,
                                token_type: TokenType::FunctionIdent,
                                value: Some(TokenValue::FunctionIdent(current.clone())),
                            });
                            func_idents.push(current.clone());
                            expected = None;
                        } else {
                            if type_idents.contains(&current) {
                                tokens.push(Token {
                                    line: self.ln,
                                    column: start,
                                    token_type: TokenType::TypeIdent,
                                    value: Some(TokenValue::TypeIdent(BUILTIN_TYPES.get(&current).unwrap().clone())),
                                });
                            } else {
                                Diagnostic {
                                    file: self.filename.clone(),
                                    line: self.ln,
                                    column: start,
                                    message: format!("Unexpected text: \"{}\"", current),
                                    suggestion: None,
                                }
                                .out(source_code);
                                exit(1);
                            }
                        }
                    }
                }
            } else if char.is_numeric() {
                let start: usize = self.cl.clone() as usize;
                let mut current = String::new();
                current.push(char);
                while let Some(next_char) = self.peek(0) {
                    if next_char.is_numeric() {
                        current.push(self.consume(0).unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    line: self.ln,
                    column: start,
                    token_type: TokenType::IntLiteral,
                    value: Some(TokenValue::IntLiteral(current.parse::<u64>().unwrap())),
                });
            } else if char == '-' {
                if let Some(next_char) = self.peek(0) {
                    if next_char == '>' {
                        self.consume(0);
                        tokens.push(Token {
                            line: self.ln,
                            column: self.cl,
                            token_type: TokenType::ThinArrow,
                            value: None,
                        });
                        expected = Some(TokenType::TypeIdent);
                    } else {
                        Diagnostic {
                            file: self.filename.clone(),
                            line: self.ln,
                            column: self.cl,
                            message: "Unexpected '-' character".to_string(),
                            suggestion: Some("Did you mean '->'?".to_string()),
                        }
                        .out(source_code);
                    }
                }
            } else if char == ';' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::Semicolon,
                    value: None,
                });
            } else if char == '(' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::OpenParen,
                    value: None,
                });
                expected = Some(TokenType::ParemeterTuple);
            } else if char == ')' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::CloseParen,
                    value: None,
                });
                expected = None;
            } else if char == '{' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::OpenCurly,
                    value: None,
                });
            } else if char == '}' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::CloseCurly,
                    value: None,
                });
            } else if char.is_whitespace() {
                continue;
            } else {
                let diagnostic: Diagnostic = Diagnostic {
                    file: self.filename.clone(),
                    line: self.ln,
                    column: self.cl,
                    message: format!("Unexpected character: '{}'", char),
                    suggestion: None,
                };
                diagnostic.out(source_code);
                exit(7);
            }
        }
        tokens
    }
}

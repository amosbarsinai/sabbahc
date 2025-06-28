use std::fmt;

use crate::err::ErrorHandler;
use crate::typing::{Type, BUILTIN_TYPES};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenType {
    FunctionKeyword,
    FunctionIdent,
    ThinArrow,
    TypeIdent,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    ReturnKeyword,
    IntLiteral,
    Semicolon,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FunctionKeyword => write!(f, "FunctionKeyword"),
            Self::FunctionIdent => write!(f, "FunctionIdent"),
            Self::ThinArrow => write!(f, "ThinArrow"),
            Self::TypeIdent => write!(f, "TypeIdent"),
            Self::OpenParen => write!(f, "OpenParen"),
            Self::CloseParen => write!(f, "CloseParen"),
            Self::OpenCurly => write!(f, "OpenCurly"),
            Self::CloseCurly => write!(f, "CloseCurly"),
            Self::ReturnKeyword => write!(f, "ReturnKeyword"),
            Self::IntLiteral => write!(f, "IntLiteral"),
            Self::Semicolon => write!(f, "Semicolon"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenValue<'a> {
    FunctionIdent(String),
    TypeIdent(&'a Type),
    IntLiteral(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Token<'a> {
    pub line: usize,
    pub column: usize,
    pub token_type: TokenType,
    pub value: Option<TokenValue<'a>>,
}

impl<'a> Token<'a> {
    pub fn get_funcid(&self) -> String {
        if let Some(TokenValue::FunctionIdent(ref ident)) = self.value {
            return ident.clone();
        } else {
            panic!("token is not a function identifier");
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Token", self.token_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tokenizer<'a> {
    input: String,
    index: u64,
    ln: usize,
    cl: usize,
    filename: String,
    error_handler: &'a ErrorHandler
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &String, filename: String, error_handler: &'a ErrorHandler) -> Self {
        Tokenizer {
            input: input.clone(),
            index: 0,
            ln: 1,
            cl: 1,
            filename,
            error_handler
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
                        } else if expected == Some(TokenType::TypeIdent) {
                            if BUILTIN_TYPES.contains_key(&current) {
                                tokens.push(Token {
                                    line: self.ln,
                                    column: start,
                                    token_type: TokenType::TypeIdent,
                                    value: Some(TokenValue::TypeIdent(&BUILTIN_TYPES.get(&current).unwrap())),
                                });
                            } else {
                                self.error_handler.err(
                                    self.ln,
                                    start,
                                    format!("No such type: \"{}\"", current),
                                    None,
                                );
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
                        self.error_handler.err(
                            self.ln,
                            self.cl,
                            String::from("Unexpected dash character '-'"),
                            Some(String::from("Maybe you meant to add a thin arrow?"))
                        );
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
                self.error_handler.err(
                    self.ln,
                    self.cl,
                    format!("Unexpected character '{}'", char),
                    None,
                );
            }
        }
        tokens
    }
}

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
    fn peek(&self) -> Option<char> {
        let peek_index = self.index;
        if peek_index >= self.input.len() as u64 {
            return None;
        }
        let char = self.input.chars().nth(peek_index as usize).unwrap();
        Some(char)
    }
    fn consume(&mut self) -> Option<char> {
        let mut consume_index = self.index;
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
    pub fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }
    pub fn consume_word(&mut self) -> Option<String> {
        let mut word = String::new();
        self.consume_whitespace();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                word.push(c);
                self.consume();
            } else {
                break;
            }
        }
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume_whitespace();
            }
            else if c.is_alphabetic() || c == '_' {
                let word = self.consume_word().unwrap();
                match word.as_str() {
                    "f" => {
                        tokens.push(Token {
                            line: self.ln,
                            column: self.cl,
                            token_type: TokenType::FunctionKeyword,
                            value: None,
                        });
                        self.consume_whitespace();
                        if let Some(func_name) = self.consume_word() {
                            tokens.push(Token {
                                line: self.ln,
                                column: self.cl,
                                token_type: TokenType::FunctionIdent,
                                value: Some(TokenValue::FunctionIdent(func_name)),
                            });
                        } else {
                            self.error_handler.err(
                                self.ln,
                                self.cl,
                                "Expected function identifier after 'f' keyword".to_string(),
                                None
                            );
                        }
                    }
                    "return" => {
                        tokens.push(Token {
                            line: self.ln,
                            column: self.cl,
                            token_type: TokenType::ReturnKeyword,
                            value: None,
                        });
                    }
                    _ => {
                        if BUILTIN_TYPES.contains_key(word.as_str()) {
                            tokens.push(Token {
                                line: self.ln,
                                column: self.cl,
                                token_type: TokenType::TypeIdent,
                                value: Some(TokenValue::TypeIdent(&BUILTIN_TYPES[word.as_str()])),
                            });
                        } else {
                            self.error_handler.err(
                                self.ln,
                                self.cl,
                                format!("Unexpected identifier: '{}'", word),
                                None
                            );
                        }
                    }
                }
            } else if c.is_numeric() {
                let mut num = String::new();
                while let Some(c) = self.peek() {
                    if c.is_numeric() {
                        num.push(c);
                        self.consume();
                    } else {
                        break;
                    }
                }
                let int_value = num.parse::<u64>().unwrap();
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::IntLiteral,
                    value: Some(TokenValue::IntLiteral(int_value)),
                });
            } else if c == '(' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::OpenParen,
                    value: None,
                });
                self.consume();
            } else if c == ')' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::CloseParen,
                    value: None,
                });
                self.consume();
            } else if c == '{' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::OpenCurly,
                    value: None,
                });
                self.consume();
            } else if c == '}' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::CloseCurly,
                    value: None,
                });
                self.consume();
            } else if c == ';' {
                tokens.push(Token {
                    line: self.ln,
                    column: self.cl,
                    token_type: TokenType::Semicolon,
                    value: None,
                });
                self.consume();
            } else if c == '-' {
                self.consume(); // consume the hyphen
                // consume again for the next character
                if let Some(next_char) = self.consume() {
                    if next_char == '>' {
                        tokens.push(Token {
                            line: self.ln,
                            column: self.cl,
                            token_type: TokenType::ThinArrow,
                            value: None,
                        });
                        self.consume(); // consume the '>'
                    } else {
                        self.error_handler.err(
                            self.ln,
                            self.cl,
                            format!("Unexpected character after hyphen sign: '{}'", c),
                            None
                        );
                        self.consume(); // consume the '-'
                    }
                }
            }
        }

        tokens
    }
}

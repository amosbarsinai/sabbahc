use std::process::exit;

use crate::err::Diagnostic;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TokenType {
    ExitStatement,
    IntegerLiteral,
    Semicolon,
    LetKeyword,
    Identifier,
    Assigner,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    input: String,
    index: u64,
    ln: usize,
    cl: usize,
    filename: String
}

enum CommentType {
    Line,
    Block
}

impl Tokenizer {
    pub fn new(input: String, filename: String) -> Self {
        Tokenizer {
            input,
            index: 0,
            ln: 1,
            cl: 1,
            filename
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
        }
        else {
            self.cl += 1;
        }
        Some(char)
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.index < self.input.len() as u64 {
            let char = self.consume(0);
            if char.is_none() {
                break;
            }
            let char = char.unwrap();
            if char.is_alphabetic() {
                let mut current = String::from(format!("{}", char));
                while let Some(c) = self.peek(0) {
                    if c.is_alphanumeric() {
                        current = format!("{}{}", current, c);
                        self.consume(0);
                    } else {
                        break;
                    }
                }
                if current == "exit" {
                    tokens.push(Token {
                        token_type: TokenType::ExitStatement,
                        value: None,
                        line: self.ln,
                        column: self.cl
                    })
                }
                else if current == "let" {
                    tokens.push(Token {
                        token_type: TokenType::LetKeyword,
                        value: None,
                        line: self.ln,
                        column: self.cl
                    });
                }
                else if current == "=" {
                    tokens.push(Token {
                        token_type: TokenType::Assigner,
                        value: None,
                        line: self.ln,
                        column: self.cl
                    });
                }
                else {
                    let diagnostic: Diagnostic = Diagnostic {
                        file: self.filename.clone(),
                        line: self.ln,
                        column: self.cl,
                        message: format!("Unexpected text: {}", current),
                        suggestion: None,
                        code_snippet: Some(self.input.clone()),
                    };
                    diagnostic.out();
                    exit(7);
                }
            }
            else if char.is_ascii_digit() {
                let mut current = String::from(format!("{}", char));
                while let Some(c) = self.peek(0) {
                    if c.is_ascii_digit() {
                        current = format!("{}{}", current, c);
                        self.consume(0);
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    token_type: TokenType::IntegerLiteral,
                    value: Some(current),
                    line: self.ln,
                    column: self.cl
                });
            }
            else if char == ';' {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    value: None,
                    line: self.ln,
                    column: self.cl
                });
            }
            else if char.is_whitespace() {}
            else if char == '/' /* comment checking - division not implemented yet */ {
                if let None = self.peek(0) {
                    let diagnostic: Diagnostic = Diagnostic {
                        file: self.filename.clone(),
                        line: self.ln,
                        column: self.cl,
                        message: String::from("Unexpected slash character (at EOF)"),
                        suggestion: Some(String::from("division not implemented yet - maybe you meant to add a comment?")),
                        code_snippet: Some(self.input.clone()),
                    };
                    diagnostic.out();
                    exit(7);
                }
                let next = self.consume(0).unwrap();
                let mut comment_type: CommentType = CommentType::Line;
                if next == '/' {}
                else if next == '*' {comment_type = CommentType::Block;}
                else {
                    let diagnostic: Diagnostic = Diagnostic {
                        file: self.filename.clone(),
                        line: self.ln,
                        column: self.cl,
                        message: format!("Unexpected comment initalizer: {}", next),
                        suggestion: Some(String::from("division not implemented yet. * or / expected after slash characters, since comment initialization is assumed.")),
                        code_snippet: Some(self.input.clone()),
                    };
                    diagnostic.out();
                    exit(7);
                }
                if let CommentType::Line = comment_type {
                    while self.consume(0).unwrap_or('\n') != '\n' {}
                }
                else if let CommentType::Block = comment_type {
                    let mut previous: char = '*';
                    while let Some(char) = self.consume(0) {
                        if format!("{}{}", previous, char) == "*/" {
                            break;
                        }
                        previous = char;
                    }
                }
            }
            else {
                let diagnostic: Diagnostic = Diagnostic {
                        file: self.filename.clone(),
                        line: self.ln,
                        column: self.cl,
                        message: format!("Unexpected character: {}", char),
                        suggestion: None,
                        code_snippet: Some(self.input.clone()),
                    };
                    diagnostic.out();
                    exit(7);
            }
        }
        tokens
    }
}
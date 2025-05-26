use std::process::exit;

#[derive(Debug)]
pub enum TokenType {
    ExitStatement,
    IntegerLiteral,
    Semicolon,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct Tokenizer {
    input: String,
    index: u64
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer {
            input,
            index: 0,
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
                        value: None
                    })
                }
                else {
                    println!("ERROR: Unexpected text: {}", current);
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
                    value: Some(current)
                });
            }
            else if char == ';' {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    value: None
                });
            }
            else if char.is_whitespace() {}
            else {
                println!("ERROR: Unexpected character: {}", char);
                exit(7);
            }
        }
        tokens
    }
}
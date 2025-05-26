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

enum CommentType {
    Line,
    Block
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
            else if char == '/' /* comment checking - division not implemented yet */ {
                if let None = self.peek(0) {
                    println!("ERROR: unexpected characater '/'");
                    exit(7);
                }
                let next = self.consume(0).unwrap();
                let mut comment_type: CommentType = CommentType::Line;
                if next == '/' {}
                else if next == '*' {comment_type = CommentType::Block;}
                else {
                    println!("ERROR: unexpected '{}' character after comment initializer (expected * or /)", next);
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
                println!("ERROR: Unexpected character: {}", char);
                exit(7);
            }
        }
        tokens
    }
}
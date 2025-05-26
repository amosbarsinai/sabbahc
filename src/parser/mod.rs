use crate::tokenizer::{Token, TokenType};

#[derive(Clone)]
pub enum AstNodeType {
    ExitStatement(Option<i32>),
}

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
    fn peek(&self) -> Option<&Token> {
        self.input.get(self.index)
    }
    fn consume(&mut self) -> Option<&Token> {
        if self.index < self.input.len() {
            let token = &self.input[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
    pub fn parse(&mut self) -> Vec<Ast> {
        let mut parsed = Vec::new();
        let mut statement = Ast::new();
        while let Some(token) = &self.consume() {
            if let TokenType::ExitStatement = token.token_type {
                let mut exit_code = None;
                if let Some(next_token) = self.peek() {
                    if let TokenType::IntegerLiteral = next_token.token_type {
                        exit_code = next_token.value.as_ref().and_then(|v| v.parse::<i32>().ok());
                        self.consume();
                    }
                }
                statement.add_child(AstNodeType::ExitStatement(exit_code));
            }

            parsed.push(statement);
            statement = Ast::new();
        }
        parsed
    }
}
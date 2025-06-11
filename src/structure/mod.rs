pub mod parser;
use crate::typing::Type;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstNodeType {
    Scope,
    ReturnKeyword,
    Expression,
    FunctionKeyword,
    FunctionIdent,
    FunctionReturnTypeAnnotation,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpressionContent {
    UnsignedInt8Literal(u8),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstNodeValue {
    Scope(Scope),
    Expression(Expression),
    FunctionIdent(String),
    FunctionReturnTypeAnnotation(Type)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstNode {
    pub node_type: AstNodeType,
    pub value: Option<AstNodeValue>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expression {
    eval_type: Type,
    content: Vec<ExpressionContent>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statement {
    pub children: Vec<AstNode>,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: AstNode) {
        self.children.push(child);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scope {
    pub children: Vec<Statement>,
    pub is_outer: bool,
}
pub mod parser;
use crate::typing::Type;
use crate::typing::UINT8;
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstNodeType {
    Scope,
    ReturnKeyword,
    Expression,
    FunctionKeyword,
    FunctionIdent,
    ParamTypeTuple,
    TypeIdent,
    ThinArrow
}

impl AstNodeType {
    pub fn fk<'a>() -> AstNode<'a> {
        AstNode { node_type: Self::FunctionKeyword, value: None }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AstNodeValue<'a> {
    Scope(Scope<'a>),
    Expression(Expression<'a>),
    FunctionIdent(String),
    ParamTypeTuple(ParamTypeTuple),
    TypeIdent(&'a Type)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AstNode<'a> {
    pub node_type: AstNodeType,
    pub value: Option<AstNodeValue<'a>>,
}

impl<'a> AstNode<'a> {
    pub fn scope(input: Scope<'a>) -> AstNode<'a> {
        AstNode {
            node_type: AstNodeType::Scope,
            value: Some(
                AstNodeValue::Scope(
                    input
                )
            )
        }
    }
    pub fn ret(content: u8) -> AstNode<'a> {
        AstNode {
            node_type: AstNodeType::ReturnKeyword,
            value: Some(AstNodeValue::Expression(Expression {
                eval_type: &UINT8,
                content // Placeholder for return expression
            }))
        }
    }
    pub fn fi(funcident: String) -> AstNode<'a> {
        AstNode {
            node_type: AstNodeType::FunctionIdent,
            value: Some(AstNodeValue::FunctionIdent(funcident))
        }
    }
    pub fn tup() -> AstNode<'a> {
        AstNode {
            node_type: AstNodeType::ParamTypeTuple,
            value: Some(AstNodeValue::ParamTypeTuple(
                ParamTypeTuple {}
            ))
        }
    }
    pub fn ti(typeident: &'a Type) -> AstNode<'a> {
        AstNode {
            node_type: AstNodeType::TypeIdent,
            value: Some(
                AstNodeValue::TypeIdent(typeident)
            )
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParamTypeTuple {}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expression<'a> {
    eval_type: &'a Type,
    content: u8 // anything else not implemented yet :|
}
impl<'a> Expression<'a> {
    pub fn new(content: u8) -> Self {
        Self { eval_type: &UINT8, content }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statement<'a> {
    pub children: Vec<AstNode<'a>>
}

impl<'a> Statement<'a> {
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }
    pub fn push(&mut self, child: AstNode<'a>) {
        self.children.push(child);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scope<'a> {
    pub children: Vec<Statement<'a>>,
}


impl<'a> fmt::Display for Scope<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Scope {{")?;
        for stmt in &self.children {
            writeln!(f, "  {}", stmt)?;
        }
        write!(f, "}}")
    }
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in &self.children {
            writeln!(f, "    {}", child)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for AstNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{:?}: {}", self.node_type, val),
            None => write!(f, "{:?}", self.node_type),
        }
    }
}

impl<'a> fmt::Display for AstNodeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNodeValue::Scope(scope) => write!(f, "{}", scope),
            AstNodeValue::Expression(expr) => write!(f, "{}", expr),
            AstNodeValue::FunctionIdent(ident) => write!(f, "FunctionIdent({})", ident),
            AstNodeValue::ParamTypeTuple(_) => write!(f, "ParamTypeTuple"),
            AstNodeValue::TypeIdent(ty) => write!(f, "TypeIdent({:?})", ty),
        }
    }
}

impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expression(type: {:?}, content: {})", self.eval_type, self.content)
    }
}
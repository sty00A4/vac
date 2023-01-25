use std::fmt::Display;

use crate::*;
use scanning::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ID(String), Int(i64), Float(f64), Continue,
    BinaryOperation { left: Box<Expr>, right: Box<Expr>, op: Token },
    UnaryOperation { expr: Box<Expr>, op: Token }, UnaryOperationRight { expr: Box<Expr>, op: Token },
    Vector(Vec<Expr>), Set(Vec<Expr>),
    Absolute(Box<Expr>)
}
impl Expr {
    pub fn name(&self) -> &str {
        match self {
            Self::ID(_) => "identifier",
            Self::Int(_) => "integer",
            Self::Float(_) => "number",
            Self::Continue => "continuation",
            Self::BinaryOperation { left:_, right:_, op:_ } => "binary operation",
            Self::UnaryOperation { expr:_, op:_ } => "unary operation",
            Self::UnaryOperationRight { expr:_, op:_ } => "unary operation (right sided)",
            Self::Vector(_) => "vector",
            Self::Set(_) => "set",
            Self::Absolute(_) => "absolute expression",
        }
    }
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ID(id) => write!(f, "{id}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Continue => write!(f, "..."),
            Self::BinaryOperation { left, right, op } => write!(f, "({left} {op} {right})"),
            Self::UnaryOperation { expr, op } => write!(f, "({op} {expr})"),
            Self::UnaryOperationRight { expr, op } => write!(f, "({expr} {op})"),
            Self::Vector(vector) => write!(f, "( {} )", vector.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Set(set) => write!(f, "{{ {} }}", set.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Absolute(expr) => write!(f, "| {expr} |"),
        }
    }
}
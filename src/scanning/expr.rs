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
    pub fn count(&self) -> usize {
        match self {
            Self::ID(_) | Self::Int(_) | Self::Float(_) | Self::Continue => 1,
            Self::BinaryOperation { left, right, op:_ } => 1 + left.count() + right.count(),
            Self::UnaryOperation { expr, op:_ } | Self::UnaryOperationRight { expr, op:_ } |
            Self::Absolute(expr) => 1 + expr.count(),
            Self::Vector(vector) => 1 + vector.iter().map(|x| x.count()).collect::<Vec<usize>>().iter().sum::<usize>(),
            Self::Set(set) => 1 + set.iter().map(|x| x.count()).collect::<Vec<usize>>().iter().sum::<usize>(),
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
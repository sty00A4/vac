use std::fmt::Display;

use crate::*;
use scanning::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    ID(String), Int(i64), Float(f64), Continue,
    BinaryOperation { left: Box<Expr>, right: Box<Expr>, op: Token },
    UnaryOperation { node: Box<Expr>, op: Token }, UnaryOperationRight { node: Box<Expr>, op: Token },
    Vector(Vec<Expr>), Set(Vec<Expr>),
    Absolute(Box<Expr>)
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ID(id) => write!(f, "{id}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Continue => write!(f, "..."),
            Self::BinaryOperation { left, right, op } => write!(f, "({left} {op} {right})"),
            Self::UnaryOperation { node, op } => write!(f, "({op} {node})"),
            Self::UnaryOperationRight { node, op } => write!(f, "({node} {op})"),
            Self::Vector(vector) => write!(f, "( {} )", vector.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Set(set) => write!(f, "{{ {} }}", set.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Absolute(node) => write!(f, "| {node} |"),
        }
    }
}
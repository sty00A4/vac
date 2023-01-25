use std::collections::HashSet;
use std::fmt::Display;

use crate::*;
use runtime::value::Value;
use scanning::expr::Expr;
use scanning::token::Token;

pub type EvalResult = Result<Return, String>;

pub fn binary(left: &Expr, right: &Expr, op: &Token) -> EvalResult {
    let left = eval(left)?;
    let right = eval(right)?;
    match op {
        
        _ => error!("illegal binary operator '{op}'")
    }
}
pub fn unary(expr: &Expr, op: &Token) -> EvalResult {
    let ret = eval(expr)?;
    match op {
        
        _ => error!("illegal unary operator '{op}'")
    }
}
pub fn absolute(expr: &Expr) -> EvalResult {
    let ret = eval(expr)?;
    match ret {
        
        _ => error!("cannot evaluate the absolute value of {ret}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Return { Value(Value), Expr(Expr), None }
impl Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "the value {value}"),
            Self::Expr(expr) => write!(f, "the expression {expr}"),
            Self::None => write!(f, "nothing"),
        }
    }
}

pub fn eval(expr: &Expr) -> EvalResult {
    match expr {
        Expr::ID(id) => Ok(Return::Expr(expr.clone())),
        Expr::Int(v) => Ok(Return::Value(Value::Number(*v as f64))),
        Expr::Float(v) => Ok(Return::Value(Value::Number(*v))),
        Expr::Continue => error!("unexpected {} `{expr}`", expr.name()),
        Expr::BinaryOperation { left, right, op } => binary(left.as_ref(), right.as_ref(), op),
        Expr::UnaryOperation { expr, op } | Expr::UnaryOperationRight { expr, op } => unary(expr.as_ref(), op),
        Expr::Absolute(expr) => absolute(expr.as_ref()),
        Expr::Vector(vector) => {
            let mut values = vec![];
            let mut typ = None;
            for e in vector.iter() {
                let ret = eval(e)?;
                if let Return::Value(value) = ret {
                    if let Some(typ) = &typ {
                        if typ != &value.typ() {
                            return error!("expected type {typ}, got {}", value.typ())
                        }
                    } else {
                        typ = Some(value.typ());
                    }
                    values.push(value);
                } else {
                    return error!("expected a value, got {ret}")
                }
            }
            Ok(Return::Value(Value::Vector(values, typ.unwrap())))
        }
        Expr::Set(set) => {
            let mut values = HashSet::new();
            if values.len() == 0 { return Ok(Return::None) }
            if values.len() == 1 { return eval(&set[0]) }
            let mut typ = None;
            for e in set.iter() {
                let ret = eval(e)?;
                if let Return::Value(value) = ret {
                    if let Some(typ) = &typ {
                        if typ != &value.typ() {
                            return error!("expected type {typ}, got {}", value.typ())
                        }
                    } else {
                        typ = Some(value.typ());
                    }
                    values.insert(value);
                } else {
                    return error!("expected a value, got {ret}")
                }
            }
            Ok(Return::Value(Value::Set(values, typ.unwrap())))
        }
    }
}
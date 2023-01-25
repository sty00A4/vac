use std::collections::HashSet;
use std::fmt::Display;

use crate::*;
use runtime::value::Value;
use scanning::expr::Expr;
use scanning::token::Token;

pub type EvalResult = Result<Return, String>;

pub fn binary_expr(left_left_expr: &Expr, left_right_expr: &Expr, left_op: &Token, right_op: &Token, right_value: &Value) -> Option<EvalResult> {
    let left_left_ret = eval(left_left_expr);
    let Ok(left_left_ret) = left_left_ret else {
        return Some(Err(left_left_ret.err().unwrap()))
    };
    let left_right_ret = eval(left_right_expr);
    let Ok(left_right_ret) = left_right_ret else {
        return Some(Err(left_right_ret.err().unwrap()))
    };
    match (left_left_ret, left_right_ret) {
        (Return::Expr(left_expr), Return::Value(left_value)) => match (left_op, right_op) {
            (Token::Add, Token::Add) | (Token::Add, Token::Sub) => match left_value.binary(right_value, right_op) {
                Ok(value) => Some(Ok(Return::Expr(Expr::BinaryOperation {
                    left: Box::new(left_expr), right: Box::new(value.expr()), op: left_op.clone()
                }))),
                Err(err) => Some(Err(err))
            }
            (Token::Sub, Token::Add) | (Token::Sub, Token::Sub) =>
            match right_value.unary(left_op) {
                Ok(right_value) => match left_value.binary(&right_value, right_op) {
                    Ok(value) => Some(Ok(Return::Expr(Expr::BinaryOperation {
                        left: Box::new(left_expr), right: Box::new(value.expr()), op: left_op.clone()
                    }))),
                    Err(err) => Some(Err(err))
                }
                Err(err) => Some(Err(err))
            }
            _ => None
        }
        (Return::Value(left_value), Return::Expr(left_expr)) => match left_value.binary(right_value, right_op) {
            Ok(value) => Some(Ok(Return::Expr(Expr::BinaryOperation {
                left: Box::new(value.expr()), right: Box::new(left_expr), op: left_op.clone()
            }))),
            Err(err) => Some(Err(err))
        }
        _ => None
    }
}

pub fn binary(left: &Expr, right: &Expr, op: &Token) -> EvalResult {
    let left_ret = eval(left)?;
    let right_ret = eval(right)?;
    match left_ret {
        Return::Expr(left_expr) => match right_ret {
            Return::Expr(right_expr) => Ok(Return::Expr(Expr::BinaryOperation { left: Box::new(left_expr), right: Box::new(right_expr), op: op.clone() })),
            Return::Value(right_value) => match &left_expr {
                Expr::BinaryOperation { left: left_left_expr, right: left_right_expr, op: left_op } =>
                    match binary_expr(left_left_expr.as_ref(), left_right_expr.as_ref(), &left_op, op, &right_value) {
                        Some(res) => res,
                        None => Ok(Return::Expr(Expr::BinaryOperation { left: Box::new(left_expr), right: Box::new(right_value.expr()), op: op.clone() }))
                    }
                _ => Ok(Return::Expr(Expr::BinaryOperation { left: Box::new(left_expr), right: Box::new(right_value.expr()), op: op.clone() }))
            }
            Return::None => error!("expected a value, got nothing")
        }
        Return::Value(left_value) => match right_ret {
            Return::Expr(right_expr) => match right_expr {
                
                _ => Ok(Return::Expr(Expr::BinaryOperation { left: Box::new(left_value.expr()), right: Box::new(right_expr), op: op.clone() }))
            }
            Return::Value(right_value) => Ok(Return::Value(left_value.binary(&right_value, op)?)),
            Return::None => error!("expected a value, got nothing")
        }
        Return::None => error!("expected a value, got nothing")
    }
}
pub fn unary(expr: &Expr, op: &Token) -> EvalResult {
    let ret = eval(expr)?;
    match ret {
        Return::Expr(expr) => match expr {
            
            _ => Ok(Return::Expr(Expr::UnaryOperation { expr: Box::new(expr.clone()), op: op.clone() }))
        }
        Return::Value(value) => Ok(Return::Value(value.unary(op)?)),
        Return::None => error!("expected a value, got nothing")
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
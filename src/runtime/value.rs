use std::{collections::HashSet, hash::Hash, fmt::Display};
use crate::{*, scanning::expr::Expr};
use scanning::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64), Vector(Vec<Value>, Type), Set(HashSet<Value>, Type)
}
impl Value {
    pub fn typ(&self) -> Type {
        match self {
            Self::Number(_) => Type::Number,
            Self::Vector(_, typ) => Type::Vector(Box::new(typ.clone())),
            Self::Set(_, typ) => Type::Set(Box::new(typ.clone())),
        }
    }
    pub fn expr(&self) -> Expr {
        match self {
            Self::Number(number) => if number.floor() == *number {
                Expr::Int(*number as i64)
            } else {
                Expr::Float(*number)
            }
            Self::Vector(vector, _) => {
                let mut values = vec![];
                for value in vector.iter() {
                    values.push(value.expr());
                }
                Expr::Vector(values)
            }
            Self::Set(set, _) => {
                let mut values = vec![];
                for value in set.iter() {
                    values.push(value.expr());
                }
                Expr::Set(values)
            }
        }
    }
    pub fn binary(&self, other: &Self, op: &Token) -> Result<Self, Error> {
        match (self, other) {
            (Self::Number(number1), Self::Number(number2)) => match op {
                Token::Add => Ok(Value::Number(number1 + number2)),
                Token::Sub => Ok(Value::Number(number1 - number2)),
                Token::Mult => Ok(Value::Number(number1 * number2)),
                Token::Div => Ok(Value::Number(number1 / number2)),
                Token::Power => Ok(Value::Number(number1.powf(*number2))),
                _ => error!("illegal binary operator '{op}'")
            }
            (Self::Vector(vector, typ), other) | (other, Self::Vector(vector, typ)) => {
                let mut values = vec![];
                for value in vector.iter() { values.push(value.binary(other, op)?); }
                Ok(Value::Vector(values, typ.clone()))
            }
            _ => error!("cannot '{op}' the values of type {} and {}", self.typ(), other.typ())
        }
    }
    pub fn unary(&self, op: &Token) -> Result<Self, Error> {
        match self {
            Self::Number(number) => match op {
                Token::Sub => Ok(Self::Number(-number)),
                Token::Percent => Ok(Self::Number(number / 100.)),
                _ => error!("illegal unary operator for number '{op}'")
            }
            Self::Vector(vector, typ) => {
                let mut values = vec![];
                for value in vector.iter() { values.push(value.unary(op)?); }
                Ok(Value::Vector(values, typ.clone()))
            }
            Self::Set(set, typ) => {
                let mut values = HashSet::new();
                for value in set.iter() { values.insert(value.unary(op)?); }
                Ok(Value::Set(values, typ.clone()))
            }
        }
    }
}
impl Eq for Value {
    fn assert_receiver_is_total_eq(&self) {}
}
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {}
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H) where Self: Sized, {}
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{number}"),
            Self::Vector(vector, _) => write!(f, "( {} )", vector.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Set(set, _) => write!(f, "{{ {} }}", set.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number, Vector(Box<Type>), Set(Box<Type>)
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number => write!(f, "number"),
            Self::Vector(typ) => write!(f, "vector of {typ}"),
            Self::Set(typ) => write!(f, "set of {typ}"),
        }
    }
}
use std::{collections::HashSet, hash::Hash, fmt::Display};

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
            Self::Number(v) => write!(f, "{v}"),
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
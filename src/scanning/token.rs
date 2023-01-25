use std::fmt::Display;

use crate::logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {

    #[regex(r"[a-zA-Z_]([a-zA-Z_0-9])*", |lex| lex.slice().to_string())]
    ID(String),
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Int(i64),
    #[regex(r"\.[0-9]+|[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[token("[")]
    #[token("(")]
    EvalIn,
    #[token("]")]
    #[token(")")]
    EvalOut,
    #[token("{")]
    SetIn,
    #[token("}")]
    SetOut,
    
    #[token(",")]
    Seperator,
    #[token(":")]
    Represent,
    #[token("@")]
    Address,
    #[token("#")]
    Hash,
    #[token("%")]
    Percent,
    #[token("|")]
    Pipe,
    #[token("...")]
    Continue,

    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mult,
    #[token("/")]
    Div,
    #[token("^")]
    Power,
    #[token("!")]
    Fraction,
    #[token("~")]
    Not,

    #[token("=")]
    Equal,
    #[token("<-")]
    Store,
    #[token("->")]
    Into,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,

    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[error]
    Error,
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ID(id) => write!(f, "{id}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::EvalIn => write!(f, "("),
            Self::EvalOut => write!(f, ")"),
            Self::SetIn => write!(f, "{{"),
            Self::SetOut => write!(f, "}}"),
            Self::SetIn => write!(f, "{{"),
            Self::SetOut => write!(f, "}}"),
            Self::Seperator => write!(f, ","),
            Self::Represent => write!(f, ":"),
            Self::Address => write!(f, "@"),
            Self::Hash => write!(f, "#"),
            Self::Percent => write!(f, "%"),
            Self::Pipe => write!(f, "|"),
            Self::Continue => write!(f, "..."),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mult => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Power => write!(f, "^"),
            Self::Fraction => write!(f, "!"),
            Self::Not => write!(f, "~"),
            Self::Equal => write!(f, "="),
            Self::Store => write!(f, "<-"),
            Self::Into => write!(f, "->"),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Error => write!(f, "<ERROR>"),
        }
    }
}
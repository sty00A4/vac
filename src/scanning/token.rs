use crate::logos::Logos;

#[derive(Logos, Debug, PartialEq)]
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
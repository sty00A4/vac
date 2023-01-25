use crate::*;
use logos::Logos;
use scanning::token::Token;

pub type LexerResult = Result<Vec<Token>, String>;

pub fn lex(input: String) -> LexerResult {
    let mut lexer = Token::lexer(input.as_str());
    let mut tokens = vec![];
    loop {
        let Some(token) = lexer.next() else { break };
        if token == Token::Error {
            return error!("unexpected {:?}", lexer.slice().to_string())
        }
        tokens.push(token);
    }
    Ok(tokens)
}
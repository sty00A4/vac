#![allow(unused)]
extern crate logos;
mod scanning;
mod runtime;
use std::{
    env,
    io::{stdin, stdout, Write}
};

#[macro_export]
macro_rules! error {
    ($msg:expr, $($a:expr),*) => {
        Err(format!($msg, $($a),*))
    };
    ($msg:expr) => {
        Err(format!($msg))
    };
}
type Error = String;

fn run(input: String) -> runtime::eval::EvalResult {
    let tokens = scanning::lexer::lex(input)?;
    // println!("{tokens:?}");
    let expr = scanning::parser::parse(tokens)?;
    // println!("{expr}");
    Ok(runtime::eval::eval(&expr)?)
}

fn main() {
    let Ok(program_path) = env::current_exe() else { eprintln!("can't resolve program path"); return };
    let args: Vec<String> = env::args().collect();
    let mut args = args.iter();
    args.next();
    if let Some(arg) = args.next() {
        todo!("execute file?")
    } else {
        println!("This is the Vac shell.");
        loop {
            let mut input = String::new();
            print!("> ");
            stdout().flush();
            let Ok(len) = stdin().read_line(&mut input) else { break };
            match run(input) {
                Ok(ret) => match ret {
                    runtime::eval::Return::Expr(expr) => println!("{expr}"),
                    runtime::eval::Return::Value(value) => println!("{value}"),
                    runtime::eval::Return::None => {},
                }
                Err(e) => eprintln!("{e}")
            }
        }
    }
}
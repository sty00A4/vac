#![allow(unused)]
extern crate logos;
mod scanning;
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

fn run(input: String) -> Result<(), String> {
    let tokens = scanning::lexer::lex(input)?;
    // println!("{tokens:?}");
    let expr = scanning::parser::parse(tokens)?;
    println!("{expr}");
    Ok(())
}

use scanning::*;
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
                Ok(_) => {}
                Err(e) => eprintln!("{e}")
            }
        }
    }
}
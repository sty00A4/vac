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
        Err(String::from($msg))
    };
}

fn run(input: String) -> Result<(), String> {
    let tokens = scanning::lexer::lex(input)?;
    println!("{tokens:?}");
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
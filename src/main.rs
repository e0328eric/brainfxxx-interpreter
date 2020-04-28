use std::env;
use std::fs;

mod interpreter;
mod lexer;
mod modint;
mod parser;
mod tape;

use crate::tape::Tape;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.contains(&String::from("-tape")) {
        &args[2]
    } else {
        &args[1]
    };
    let input = if args.len() > 2 { Some(&args[2]) } else { None };
    let show_tape = args.contains(&String::from("-tape"));
    let script = fs::read_to_string(filename).unwrap();
    let mut tape = Tape::new();
    let parsed = parser::parser(&lexer::lex(&script));
    let result = interpreter::interpreter(&parsed, &mut tape, &mut (input, 0), show_tape);
    println!("{}", result);
}

use std::env::args;

mod lexer;
mod parser;

fn main() {
    let args = args().collect::<Vec<_>>();
    let lex = lexer::lexer(&args[1]);
    eprintln!("DEBUGPRINT[20]: main.rs:9: lex={:#?}", lex);
    let parse = parser::parse(lex);
}

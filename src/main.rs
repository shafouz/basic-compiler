use std::{env::args, str::FromStr};

use lexer::Token;
use parser::factor;

mod lexer;
mod parser;

fn main() {
    let args = args().collect::<Vec<_>>();
    let lex = lexer::lexer(&args[1]);
    eprintln!("DEBUGPRINT[3]: main.rs:13: lex={:#?}", lex);
    let parse = parser::parse(lex).unwrap();
    eprintln!("DEBUGPRINT[1]: main.rs:14: parse={:#?}", parse);
}

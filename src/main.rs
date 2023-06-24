mod lexer;
mod parser;

fn main() {
    let lex = lexer::lexer("1 + 2");
    let parse = parser::parse(lex);
}

mod lexer;
mod parser;

fn main() {
    let lex = lexer::lexer("1 a + 2 + let");
    let parse = parser::parse(lex);
}

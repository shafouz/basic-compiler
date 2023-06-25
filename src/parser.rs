use crate::lexer::Token;

pub fn parse(mut lex: Vec<Token>) {
    // how to identify a var
    // var is any token that matches the regex [A-Z]
    // let token_stream = lex.into_iter();
    let mut tokens = vec![];

    while let Some(next) = lex.pop() {
        tokens.push(next);

        expr(&mut tokens);
        // factor(&tokens);
        // term(&tokens);
    }
}

fn expr(mut tokens: &mut Vec<Token>) -> Option<Expr> {
    // check if first is op
    // remove from vec if it is
    let mut expr = Expr {
        op: None,
        term: None,
    };

    if let Token::Token(c) = tokens.remove(0) {
        expr.op = Some(c);
    }

    let term = term(tokens);

    None
}

struct Expr {
    op: Option<char>,
    term: Option<Term>,
}

struct Term {
    op: Option<char>,
    factor: Option<Factor>,
}

enum Factor {
    Var(char),
    Number(u32),
    Expr(Box<Expr>),
    Nop,
}

fn term(tokens: &[Token]) -> Term {
    // term ::= factor ((*|/) factor)*
    factor(tokens);
}

// should return a Factor
fn factor(tokens: &[Token]) -> Option<Factor> {
    for token in tokens {
        match token {
            // var
            Token::Id(id) => {
                if id.len() == 1 {
                    Some(Factor::Var(id.pop().unwrap()))
                } else {
                    None
                }
            }
            // number
            Token::Number(num) => Some(Factor::Number(*num)),
            // expr
            _ => {
                expr(tokens);
                None
            }
        };
    }

    None
}

enum Grammar {
    Expr,
    Factor,
    Term,
    Number,
    Var,
}

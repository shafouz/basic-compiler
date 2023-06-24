use crate::lexer::Token;

pub fn parse(mut lex: Vec<Token>) {
    // how to identify a var
    // var is any token that matches the regex [A-Z]
    // let token_stream = lex.into_iter();
    let mut tokens = vec![];

    while let Some(next) = lex.pop() {
        tokens.push(next);

        expr(&tokens);
        // term(&tokens);
    }
}

fn expr(token: &[Token]) {
    // expression ::= (+|-|ε) term ((+|-) term)*
    // + term(token)
    // - term(token)
    // term(token)
    // term(token) + term(token)
    // term(token) - term(token)
    // + term(token) + term(token)
    // + term(token) - term(token)
    // - term(token) - term(token)
    // - term(token) + term(token)
    let mut it = token.iter();

    for next in it {
        match next {
            Token::Token('+') | Token::Token('-') => {}
            _ => {}
        }
    }
}

fn term(token: &[Token]) {
    // term ::= factor ((*|/) factor)*
    // match token {
    // factor(token)
    // factor(token) (* factor(token))
    // factor(token) (/ factor(token))
}

fn factor(token: Token) {
    match token {
        Token::Id(_) => {
            //
            "var"
        }
        Token::Number(_) => {
            //
            "number"
        }
        Token::Expression => {
            //
            "expr"
        }
        _ => panic!("factor: unexpected token"),
    };
}

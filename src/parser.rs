use std::str::FromStr;

use crate::lexer::{Reserved, Token};

pub fn parse(mut lex: Vec<Token>) {
    // pop a token
    // if it matches something keep going and pop as much as it needs
    // if at some place there is an error
    // restore from the start and try by poping n+1 tokens

    let mut lookahead = 0;
    let mut index = 0;
    // let amount_of_tokens = 1;

    // let token_pool = &lex[0..amount_of_tokens];

    lex.reverse();
    let token = &lex[index];
    match token {
        Token::Number(_) | Token::Id(_) => {
            // how to recover if it fails?
            if token.is_number() {
                lookahead += 1;
                statement(&lex, index, lookahead);
            } else {
                statement(&lex, index, lookahead);
            }
        }
        _ => (),
    };

    // if let Some(token) = lex.pop() {
    //     match token {
    //         Token::Number(_) | Token::Id(_) => {
    //             // do smth
    //             statement(&lex);
    //         }
    //         _ => (),
    //     };
    // }
    // starts with a number and ends with a CR
    // line
}

fn statement(tokens: &[Token], index: usize, lookahead: usize) {
    let token = &tokens[index];

    if token.is_reserved() {
        // match every reserved value
        // match Reserved
        if let Token::Id(id) = token {
            match Reserved::from_str(id).unwrap() {
                Reserved::PRINT => todo!(),
                Reserved::RUN => todo!(),
                Reserved::LIST => todo!(),
                Reserved::CLEAR => todo!(),
                Reserved::RETURN => todo!(),
                Reserved::GOSUB => todo!(),
                Reserved::LET => todo!(),
                Reserved::INPUT => todo!(),
                Reserved::GOTO => todo!(),
                Reserved::THEN => todo!(),
                Reserved::IF => todo!(),
                Reserved::END => todo!(),
            }
        }
    } else {
        // there has been an error since it needs to be an statement
    }
}

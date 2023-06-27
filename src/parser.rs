use std::{
    io::{self, Error},
    str::FromStr,
};

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
    line(&lex, index, lookahead);

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

fn line(tokens: &[Token], mut index: usize, mut lookahead: usize) {
    let token = &tokens[index + lookahead];

    // line resets the lookahead
    match token {
        Token::Number(_) => {
            let num = token;
            lookahead += 1;
            statement(tokens, index, lookahead);
        }
        _ => statement(tokens, index, lookahead),
    };
}

fn statement(tokens: &[Token], mut index: usize, mut lookahead: usize) {
    let token = &tokens[index + lookahead];

    if let Token::Reserved(keyword) = token {
        match keyword {
            Reserved::END => todo!(),
            Reserved::RUN => todo!(),
            Reserved::LIST => todo!(),
            Reserved::CLEAR => todo!(),
            Reserved::RETURN => todo!(),
            Reserved::GOSUB => todo!(),
            Reserved::LET => todo!(),
            Reserved::INPUT => todo!(),
            Reserved::GOTO => {
                let reserved = token;
                lookahead += 1;
                expression(tokens, index, lookahead);
            }
            Reserved::THEN => todo!(),
            Reserved::IF => todo!(),
            Reserved::PRINT => todo!(),
        }
    } else {
        // throw error, invalid statement
    }
}

// expression ::= (+|-|ε) term ((+|-) term)*
fn expression(
    tokens: &[Token],
    mut index: usize,
    mut lookahead: usize,
) -> Result<Expression, Error> {
    let token = &tokens[index + lookahead];
    let mut terms: Vec<Term> = vec![];

    // return early if initial one fails
    let initial_lookahead = lookahead;
    match token {
        Token::Plus | Token::Minus => {
            let op = token;
            lookahead += 1;

            if let Ok(mut term) = term(tokens, index, lookahead) {
                term.op = Some(op.clone());
                terms.push(term);
            } else {
                lookahead = initial_lookahead;
                return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid term"));
            }
        }
        _ => {
            if let Ok(term) = term(tokens, index, lookahead) {
                terms.push(term);
            } else {
                lookahead = initial_lookahead;
                return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid term"));
            }
        }
    };

    let initial_lookahead = lookahead;
    for token in &tokens[index + lookahead..] {
        match token {
            Token::Plus | Token::Minus => {
                let op = token;
                lookahead += 1;

                if let Ok(mut term) = term(tokens, index, lookahead) {
                    term.op = Some(op.clone());
                    terms.push(term);
                } else {
                    lookahead = initial_lookahead;
                    break;
                }
            }
            _ => {
                if let Ok(term) = term(tokens, index, lookahead) {
                    terms.push(term);
                } else {
                    lookahead = initial_lookahead;
                    break;
                }
            }
        }
    }

    let expression = Expression { child: terms };
    Ok(expression)
}

// term ::= factor ((*|/) factor)*
fn term(tokens: &[Token], mut index: usize, mut lookahead: usize) -> Result<Term, Error> {
    // one or more factors
    // do one, try to do in a loop until an error?

    // case one factor
    let mut factors: Vec<Factor> = vec![factor(tokens, index, lookahead)?];

    // any number of factors
    let initial_lookahead = lookahead;
    for token in &tokens[index + lookahead..] {
        match token {
            Token::Asterisk | Token::Slash => {
                lookahead += 1;
                let op = token;

                if let Ok(mut factor) = factor(tokens, index, lookahead) {
                    factor.op = Some(op.clone());
                    factors.push(factor);
                } else {
                    lookahead = initial_lookahead;
                    break;
                }
            }
            _ => {
                if let Ok(factor) = factor(tokens, index, lookahead) {
                    factors.push(factor);
                } else {
                    lookahead = initial_lookahead;
                    break;
                }
            }
        }
    }

    let term = Term {
        op: None,
        child: factors,
    };
    Ok(term)
}

#[derive(Debug, PartialEq)]
struct Factor {
    op: Option<Token>,
    data: Box<Factors>,
}

#[derive(Debug, PartialEq)]
#[allow(unused)]
enum Factors {
    Number(u32),
    Var(char),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
struct Expression {
    child: Vec<Term>,
}

#[derive(Debug, PartialEq)]
struct Term {
    op: Option<Token>,
    child: Vec<Factor>,
}

// factor ::= var | number | (expression)
fn factor(tokens: &[Token], mut index: usize, mut lookahead: usize) -> Result<Factor, Error> {
    let token = &tokens[index + lookahead];

    match token {
        Token::Number(num) => {
            // terminal number
            Ok(Factor {
                op: None,
                data: Box::new(Factors::Number(*num)),
            })
        }
        Token::Var(var) => {
            // terminal var
            Ok(Factor {
                op: None,
                data: Box::new(Factors::Var(*var)),
            })
        }
        _ => {
            // expression
            // None == could not create expression
            // maybe an error is better?
            let expr_res = expression(tokens, index, lookahead);

            if let Ok(expr) = expr_res {
                Ok(Factor {
                    op: None,
                    data: Box::new(Factors::Expression(expr)),
                })
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "could not create expression",
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor() {
        let tokens = vec![Token::Number(1)];
        let index = 0;
        let lookahead = 0;

        let res = factor(&tokens, index, lookahead);
        eprintln!("DEBUGPRINT[4]: parser.rs:263: res={:#?}", res);
    }

    #[test]
    fn test_factor_expr() {
        let tokens = vec![Token::Plus, Token::Number(1)];
        let index = 0;
        let lookahead = 0;

        let res = factor(&tokens, index, lookahead);
        eprintln!("DEBUGPRINT[5]: parser.rs:273: res={:#?}", res);

        // if let Factors::Expression(_) = *res.data {
        // } else {
        //     panic!("should be an expression");
        // }
    }

    #[test]
    fn mut_index_slice() {
        let mut i = 0;
        let mut v = vec![];

        for j in i..100 {
            v.push(j);
            i = j;
        }

        assert!(i == 99);
        assert_eq!(v.len(), 100);
    }
}

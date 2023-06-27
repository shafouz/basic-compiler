use std::io::{self, Error};

use crate::lexer::{Relop, Reserved, Token};

#[derive(Debug, PartialEq)]
pub struct Line {
    number: Option<u32>,
    statement: Statement,
}

#[derive(Debug, PartialEq)]
struct Statement {
    reserved: Reserved,
    child: StatementType,
}

#[derive(Debug, PartialEq)]
struct VarList {}

#[derive(Debug, PartialEq)]
struct ExprList {}

#[derive(Debug, PartialEq)]
enum StatementType {
    Expression(Expression),
    VarList(VarList),
    ExprList(ExprList),
    IfThen((Expression, Relop, Expression, Box<Statement>)),
    Singleton,
}

#[derive(Debug, PartialEq)]
pub struct Factor {
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

pub fn parse(mut lex: Vec<Token>) -> Result<Line, Error> {
    // pop a token
    // if it matches something keep going and pop as much as it needs
    // if at some place there is an error
    // restore from the start and try by poping n+1 tokens

    let mut lookahead = 0;
    let mut index = 0;

    let line = line(&lex, &mut index, &mut lookahead)?;
    // make sure every token is consumed
    // lookahead is used as the lower bound -> lookahead..
    // so 2..2 is valid and doesnt loop
    assert_eq!(lookahead, lex.len());
    // index = lookahead;

    Ok(line)
}

fn line(tokens: &[Token], index: &mut usize, lookahead: &mut usize) -> Result<Line, Error> {
    let token = &tokens[*index + *lookahead];

    // line resets the lookahead
    match token {
        Token::Number(number) => {
            *lookahead += 1;
            let _statement = statement(tokens, index, lookahead)?;
            Ok(Line {
                number: Some(*number),
                statement: _statement,
            })
        }
        _ => {
            // None
            let _statement = statement(tokens, index, lookahead)?;

            Ok(Line {
                number: None,
                statement: _statement,
            })
        }
    }
}

fn statement(
    tokens: &[Token],
    index: &mut usize,
    lookahead: &mut usize,
) -> Result<Statement, Error> {
    let token = &tokens[*index + *lookahead];

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
                *lookahead += 1;

                // unrecoverable error
                let expr = expression(tokens, index, lookahead)?;

                Ok(Statement {
                    reserved: *keyword,
                    child: StatementType::Expression(expr),
                })
            }
            Reserved::THEN => todo!(),
            Reserved::IF => todo!(),
            Reserved::PRINT => todo!(),
        }
    } else {
        // throw error, invalid statement
        Err(Error::new(io::ErrorKind::InvalidInput, "Invalid statement"))
    }
}

// expression ::= (+|-|ε) term ((+|-) term)*
fn expression(
    tokens: &[Token],
    index: &mut usize,
    lookahead: &mut usize,
) -> Result<Expression, Error> {
    let token = &tokens[*index + *lookahead];
    let mut terms: Vec<Term> = vec![];

    // return early if initial one fails
    let initial_lookahead = *lookahead;
    match token {
        Token::Plus | Token::Minus => {
            let op = token;
            *lookahead += 1;

            if let Ok(mut term) = term(tokens, index, lookahead) {
                term.op = Some(op.clone());
                terms.push(term);
            } else {
                *lookahead = initial_lookahead;
                return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid term"));
            }
        }
        _ => {
            if let Ok(term) = term(tokens, index, lookahead) {
                terms.push(term);
            } else {
                *lookahead = initial_lookahead;
                return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid term"));
            }
        }
    };

    let initial_lookahead = *lookahead;
    for token in &tokens[*index + *lookahead..] {
        match token {
            Token::Plus | Token::Minus => {
                let op = token;
                *lookahead += 1;

                if let Ok(mut term) = term(tokens, index, lookahead) {
                    term.op = Some(op.clone());
                    terms.push(term);
                } else {
                    *lookahead = initial_lookahead;
                    break;
                }
            }
            _ => {
                if let Ok(term) = term(tokens, index, lookahead) {
                    terms.push(term);
                } else {
                    *lookahead = initial_lookahead;
                    break;
                }
            }
        }
    }

    let expression = Expression { child: terms };
    Ok(expression)
}

// term ::= factor ((*|/) factor)*
fn term(tokens: &[Token], index: &mut usize, lookahead: &mut usize) -> Result<Term, Error> {
    // one or more factors
    // do one, try to do in a loop until an error?

    // case one factor
    let mut factors: Vec<Factor> = vec![factor(tokens, index, lookahead)?];

    // any number of factors
    let initial_lookahead = *lookahead;
    for token in &tokens[*index + *lookahead..] {
        match token {
            Token::Asterisk | Token::Slash => {
                *lookahead += 1;
                let op = token;

                if let Ok(mut factor) = factor(tokens, index, lookahead) {
                    factor.op = Some(op.clone());
                    factors.push(factor);
                } else {
                    *lookahead = initial_lookahead;
                    break;
                }
            }
            _ => {
                if let Ok(factor) = factor(tokens, index, lookahead) {
                    factors.push(factor);
                } else {
                    *lookahead = initial_lookahead;
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

// factor ::= var | number | (expression)
pub fn factor(tokens: &[Token], index: &mut usize, lookahead: &mut usize) -> Result<Factor, Error> {
    if *lookahead >= tokens.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "EOF reached"));
    }

    let token = &tokens[*index + *lookahead];

    #[allow(clippy::let_and_return)]
    let factor = match token {
        Token::Number(num) => {
            // terminal number
            *lookahead += 1;

            Ok(Factor {
                op: None,
                data: Box::new(Factors::Number(*num)),
            })
        }
        Token::Var(var) => {
            // terminal var
            *lookahead += 1;

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
                    "Could not create expression",
                ))
            }
        }
    };

    factor
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    use super::*;

    #[test]
    fn goto_works() {
        let lex = lexer::lexer("1 goto 1");
        let parse = parse(lex);

        if let Ok(line) = parse {
            assert!(line.number.is_some());
            let statement = line.statement;
            assert_eq!(statement.reserved, Reserved::GOTO);

            if let StatementType::Expression(expr) = statement.child {
                assert_eq!(expr.child.len(), 1);

                let term = &expr.child[0];
                assert_eq!(term.op, None);

                assert_eq!(term.child.len(), 1);
                let factor = &term.child[0];

                assert_eq!(factor.op, None);
                assert_eq!(factor.data, Box::new(Factors::Number(1)));
            } else {
                panic!("Wrong Expression type")
            }
        } else {
            panic!("Parse failed")
        }
    }

    #[test]
    fn goto_works_without_line_number() {
        let lex = lexer::lexer("goto 1");
        let parse = parse(lex);

        if let Ok(line) = parse {
            assert!(line.number.is_none());
            let statement = line.statement;
            assert_eq!(statement.reserved, Reserved::GOTO);

            if let StatementType::Expression(expr) = statement.child {
                assert_eq!(expr.child.len(), 1);

                let term = &expr.child[0];
                assert_eq!(term.op, None);

                assert_eq!(term.child.len(), 1);
                let factor = &term.child[0];

                assert_eq!(factor.op, None);
                assert_eq!(factor.data, Box::new(Factors::Number(1)));
            } else {
                panic!("Wrong Expression type")
            }
        } else {
            panic!("Parse failed")
        }
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

    #[test]
    fn zero_loops_slice() {
        let i = vec![0; 2];

        for x in &i[2..] {
            eprintln!("DEBUGPRINT[1]: parser.rs:317: x={:#?}", x);
            panic!("aaaaa");
        }
    }
}

use std::{fmt::Display, str::FromStr};

use strum::VariantNames;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Other(char),
    Var(char),
    Relop(Relop),
    Number(u32),
    String(String),
    ClosingBracket(char),
    OpeningBracket(char),
    Reserved(Reserved),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(
    Debug, strum::Display, strum::EnumString, strum::EnumVariantNames, PartialEq, Clone, Copy,
)]
pub enum Reserved {
    END,
    RUN,
    LIST,
    CLEAR,
    RETURN,
    GOSUB,
    LET,
    INPUT,
    GOTO,
    THEN,
    IF,
    PRINT,
}

impl Token {
    pub fn is_number(&self) -> bool {
        matches!(self, Token::Number(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Relop {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

pub fn lexer(expr: &str) -> Vec<Token> {
    let mut char_stream = expr.chars().peekable();
    let mut line = 0;
    let mut tokens = vec![];

    let mut i = 0;
    while let Some(next) = char_stream.next() {
        if next == '\n' {
            line += 1;
        } else if next == '\t' || next == ' ' {
            continue;
        }

        // eprintln!("DEBUGPRINT[15]: lexer.rs:49: peek={:#?}, i={}", next, i);
        // i += 1;

        let token = match next {
            // can be <> <= <
            '<' => {
                match char_stream.peek() {
                    // <=
                    Some('=') => {
                        char_stream.next();
                        Token::Relop(Relop::LessThanOrEqual)
                    }
                    // <>
                    Some('>') => {
                        char_stream.next();
                        Token::Relop(Relop::NotEqual)
                    }
                    // <
                    Some(_) => Token::Relop(Relop::LessThan),
                    _ => panic!("EOF should not have gotten here?"),
                }
            }
            '>' => {
                match char_stream.peek() {
                    // >=
                    Some('=') => {
                        char_stream.next();
                        Token::Relop(Relop::GreaterThanOrEqual)
                    }
                    // <>
                    Some('<') => {
                        char_stream.next();
                        Token::Relop(Relop::NotEqual)
                    }
                    // >
                    Some(_) => Token::Relop(Relop::GreaterThan),
                    _ => panic!("EOF should not have gotten here?"),
                }
            }
            '=' => Token::Relop(Relop::Equal),
            'a'..='z' | 'A'..='Z' => {
                let mut string = String::from(next);

                while let Some(peek) = char_stream.peek() {
                    if peek.is_ascii_alphabetic() {
                        string.push(*peek);
                        char_stream.next();
                    } else {
                        break;
                    }
                }

                if string.len() == 1 {
                    Token::Var(string.chars().next().unwrap())
                } else if Reserved::VARIANTS.contains(&string.to_ascii_uppercase().as_str()) {
                    Token::Reserved(Reserved::from_str(&string.to_ascii_uppercase()).unwrap())
                } else {
                    panic!("Unknown identifier: {}", string);
                }
            }
            '0'..='9' => {
                let mut num = next.to_digit(10).unwrap();

                while let Some(peek) = char_stream.peek() {
                    if peek.is_ascii_digit() {
                        num = 10 * num + peek.to_digit(10).unwrap();
                        char_stream.next();
                    } else {
                        break;
                    }
                }

                Token::Number(num)
            }
            '"' => {
                let mut string = String::new();

                while let Some(peek) = char_stream.peek() {
                    if *peek == '"' {
                        char_stream.next();
                        break;
                    } else {
                        string.push(*peek);
                        char_stream.next();
                    }
                }

                Token::String(string)
            }
            '(' | ')' => {
                if next == '(' {
                    Token::OpeningBracket(next)
                } else {
                    Token::ClosingBracket(next)
                }
            }
            '+' | '-' => {
                if next == '+' {
                    Token::Plus
                } else {
                    Token::Minus
                }
            }
            '*' | '/' => {
                if next == '*' {
                    Token::Asterisk
                } else {
                    Token::Slash
                }
            }
            _ => panic!("Unknown character: {}", next),
        };

        tokens.push(token);
    }

    tokens
}

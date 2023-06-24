#[derive(Debug)]
pub enum Token {
    Nop,
    Token(char),
    Id(String),
    Relop(Relop),
    Number(u32),
    String(String),
    Expression,
}

impl Token {
    fn is_reserved(&self) -> bool {
        let reserved = [
            "print", "if", "then", "goto", "input", "let", "gosub", "return", "clear", "list",
            "run", "end",
        ]
        .to_vec();

        match self {
            Token::Id(string) => reserved.contains(&string.as_str()),
            _ => false,
        }
    }
}

#[derive(Debug)]
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

    while let Some(peek) = char_stream.next() {
        if peek == '\n' {
            line += 1;
        } else if peek == '\t' || peek == ' ' {
            continue;
        }

        let token = match peek {
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
                let mut string = String::from(peek);

                while let Some(next) = char_stream.peek() {
                    if next.is_ascii_alphabetic() {
                        string.push(*next);
                        char_stream.next();
                    } else {
                        char_stream.next();
                        break;
                    }
                }

                Token::Id(string)
            }
            '0'..='9' => {
                let mut num = peek.to_digit(10).unwrap();

                while let Some(next) = char_stream.peek() {
                    if next.is_ascii_digit() {
                        num = 10 * num + next.to_digit(10).unwrap();
                        char_stream.next();
                    } else {
                        break;
                    }
                }

                Token::Number(num)
            }
            '"' => {
                let mut string = String::new();

                while let Some(next) = char_stream.peek() {
                    if next == &'"' {
                        char_stream.next();
                        break;
                    }

                    string.push(*next);
                    char_stream.next();
                }

                Token::String(string)
            }
            _ => Token::Token(peek),
        };

        tokens.push(token);
    }

    eprintln!(
        "DEBUGPRINT[2]: main.rs:78: tokens={:#?}, line={}",
        tokens, line
    );
    // relops
    tokens
}

#[derive(Debug, Clone)]
pub enum Token {
    Literal(char),
    LParen,
    RParen,
    Concat,
    Alt,
    Star,
}

fn operator_precedence(token: &Token) -> u8 {
    match token {
        Token::Star => 3,
        Token::Concat => 2,
        Token::Alt => 1,
        _ => 0,
    }
}

fn tokenize(needle: String) -> Vec<Token> {
    let mut infix: Vec<Token> = Vec::new();
    let mut infix_with_concat: Vec<Token> = Vec::new();

    for c in needle.chars() {
        if c == '(' {
            infix.push(Token::LParen);
        } else if c == ')' {
            infix.push(Token::RParen);
        } else if c == '|' {
            infix.push(Token::Alt);
        } else if c == '*' {
            infix.push(Token::Star);
        } else {
            infix.push(Token::Literal(c));
        }
    }

    println!("Infix: {:?}", infix);
    for pair in infix.windows(2) {
        match pair {
            [Token::Literal(_), Token::Literal(_) | Token::LParen]
            | [Token::RParen | Token::Star, Token::Literal(_)] => {
                println!("Pair needing concat: {:?}", pair);
                infix_with_concat.push(pair[0].clone());
                infix_with_concat.push(Token::Concat);
            }
            _ => infix_with_concat.push(pair[0].clone()),
        }
    }
    infix_with_concat.push(infix.last().unwrap().clone());
    println!("Infix with concats: {:?}", infix_with_concat);
    infix_with_concat
}

pub fn parse(needle: String) -> Vec<Token> {
    let infix: Vec<Token> = tokenize(needle);
    let mut postfix: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in infix {
        match token {
            Token::Literal(_) => postfix.push(token),
            Token::LParen => stack.push(token),
            Token::RParen => loop {
                match stack.pop() {
                    Some(Token::LParen) => break,
                    Some(other) => postfix.push(other),
                    None => panic!("Unmatched parantheses!"),
                }
            },
            _ => {
                while let Some(operator) = stack.last() {
                    if matches!(operator, Token::LParen) {
                        break;
                    }
                    if operator_precedence(operator) >= operator_precedence(&token) {
                        postfix.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }
        }
    }

    while let Some(operator) = stack.pop() {
        postfix.push(operator);
    }

    println!("Postfix: {:?}", postfix);
    postfix
}

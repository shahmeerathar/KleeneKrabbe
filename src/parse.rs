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

fn tokenize(needle: &str) -> Vec<Token> {
    let mut infix: Vec<Token> = Vec::new();
    let mut prev: Option<Token> = None;

    for c in needle.chars() {
        let token = match c {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '|' => Token::Alt,
            '*' => Token::Star,
            _ => Token::Literal(c),
        };

        if let Some(prev_token) = &prev {
            match (prev_token, &token) {
                (Token::Literal(_), Token::Literal(_) | Token::LParen)
                | (Token::RParen | Token::Star, Token::Literal(_) | Token::LParen) => {
                    infix.push(Token::Concat);
                }
                _ => {}
            }
        }

        infix.push(token.clone());
        prev = Some(token);
    }

    println!("Infix: {:?}", infix);
    infix
}

pub fn parse(needle: &str) -> Vec<Token> {
    let mut postfix: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();

    for token in tokenize(needle) {
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

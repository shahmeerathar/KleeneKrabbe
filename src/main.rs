#![warn(clippy::pedantic)]

use std::char;

#[derive(Debug, Clone)]
enum Token {
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

fn parse(needle: String) -> Vec<Token> {
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

enum Transition {
    Character(char),
    Epsilon,
}

struct State {
    transition: Transition,
    out1: Option<usize>,
    out2: Option<usize>,
}

struct Fragment {
    start: usize,
    accept: usize,
}

struct NFA {
    states: Vec<State>,
    start: usize,
    accept: usize,
}

fn compile(postfix: &Vec<Token>) -> NFA {
    let mut states: Vec<State> = Vec::new();
    let mut stack: Vec<Fragment> = Vec::new();
    for token in postfix {
        match token {
            Token::Literal(c) => {
                let s1 = State {
                    transition: Transition::Character(*c),
                    out1: Some(states.len() + 1),
                    out2: None,
                };
                let s2 = State {
                    transition: Transition::Epsilon,
                    out1: None,
                    out2: None,
                };
                let frag = Fragment {
                    start: states.len(),
                    accept: states.len() + 1,
                };
                states.push(s1);
                states.push(s2);
                stack.push(frag);
            }
            Token::Concat => {
                let f2 = stack.pop().expect("Expected existing fragment.");
                let f1 = stack.pop().expect("Expected existing fragment.");
                states[f1.accept].out1 = Some(f2.start);
                let frag = Fragment {
                    start: f1.start,
                    accept: f2.accept,
                };
                stack.push(frag);
            }
            Token::Alt => {
                let f2 = stack.pop().expect("Expected existing fragment.");
                let f1 = stack.pop().expect("Expected existing fragment.");
                let s1 = State {
                    transition: Transition::Epsilon,
                    out1: Some(f1.start),
                    out2: Some(f2.start),
                };
                let s2 = State {
                    transition: Transition::Epsilon,
                    out1: None,
                    out2: None,
                };
                let frag = Fragment {
                    start: states.len(),
                    accept: states.len() + 1,
                };
                states[f1.accept].out1 = Some(states.len() + 1);
                states[f2.accept].out1 = Some(states.len() + 1);
                states.push(s1);
                states.push(s2);
                stack.push(frag);
            }
            Token::Star => {
                let f1 = stack.pop().expect("Expected existing fragment.");
                let s1 = State {
                    transition: Transition::Epsilon,
                    out1: Some(f1.start),
                    out2: Some(states.len() + 1),
                };
                let s2 = State {
                    transition: Transition::Epsilon,
                    out1: None,
                    out2: None,
                };
                let frag = Fragment {
                    start: states.len(),
                    accept: states.len() + 1,
                };
                states[f1.accept].out1 = Some(states.len());
                states.push(s1);
                states.push(s2);
                stack.push(frag);
            }
            _ => {}
        }
    }

    let fragment = stack.pop().expect("Expected a fragment");
    NFA {
        states: states,
        start: fragment.start,
        accept: fragment.accept,
    }
}

fn main() {
    let haystack =
        String::from("This string should match: abbbbbbbba\nThis string should not match: abbbbba");
    let needle = String::from("a(bb)*a");
    println!("Finding pattern {needle} in:\n{haystack}");

    let postfix = parse(needle);
    let nfa = compile(&postfix);
}

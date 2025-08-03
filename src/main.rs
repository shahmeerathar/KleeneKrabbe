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

#[derive(Debug)]
enum Transition {
    Character(char),
    Epsilon,
}

#[derive(Debug)]
struct State {
    transition: Transition,
    out1: Option<usize>,
    out2: Option<usize>,
}

struct Fragment {
    start: usize,
    accept: usize,
}

#[derive(Debug)]
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

fn follow_epsilons(state: &State, states: &mut Vec<usize>, nfa: &NFA) {
    match state.out1 {
        None => {}
        Some(idx) => {
            let s = &nfa.states[idx];
            match s.transition {
                Transition::Character(_) => states.push(idx),
                Transition::Epsilon => follow_epsilons(s, states, nfa),
            }
        }
    };
    match state.out2 {
        None => {}
        Some(idx) => {
            let s = &nfa.states[idx];
            match s.transition {
                Transition::Character(_) => states.push(idx),
                Transition::Epsilon => follow_epsilons(s, states, nfa),
            }
        }
    };
}

fn match_pattern(haystack: &String, nfa: &NFA) -> Option<String> {
    for i in 0..haystack.len() {
        let substr = &haystack[i..haystack.len()];
        println!("\nSubstring: {:?}", substr);

        let mut curr_states: Vec<usize> = Vec::new();
        let mut next_states: Vec<usize> = Vec::new();
        let start_state = &nfa.states[nfa.start];
        match start_state.transition {
            Transition::Character(_) => curr_states.push(nfa.start),
            Transition::Epsilon => {
                follow_epsilons(start_state, &mut curr_states, nfa);
            }
        }

        for (j, char) in substr.chars().enumerate() {
            if curr_states.is_empty() {
                break;
            };

            println!("\nChar: {:?}", char);
            while !curr_states.is_empty() {
                let state_idx = curr_states.pop().expect("Expected a state in curr_states");
                if state_idx == nfa.accept {
                    return Some(substr.to_string());
                }
                let state = &nfa.states[state_idx];
                println!("State {:?}: {:?}", state_idx, state);
                match state.transition {
                    Transition::Character(c) => {
                        if char == c {
                            match state.out1 {
                                None => {}
                                Some(s) => {
                                    if s == nfa.accept {
                                        return Some(substr[..j + 1].to_string());
                                    }
                                    next_states.push(s)
                                }
                            };
                        }
                    }
                    Transition::Epsilon => {}
                }
            }

            while !next_states.is_empty() {
                let next_state_idx = next_states.pop().expect("Expected a state in next_states");
                let next_state = &nfa.states[next_state_idx];
                match next_state.transition {
                    Transition::Character(_) => curr_states.push(next_state_idx),
                    Transition::Epsilon => follow_epsilons(next_state, &mut curr_states, nfa),
                }
            }
        }
    }

    None
}

fn main() {
    let haystack =
        String::from("This string should match: abbbbbbbba. This string should not match: abbbbba");
    let needle = String::from("a(bb)*a");
    println!("Finding pattern {needle} in:\n{haystack}\n");

    let postfix = parse(needle);
    let nfa = compile(&postfix);
    println!("\nNFA:");
    for (i, state) in nfa.states.iter().enumerate() {
        println!("{:?}: {:?}", i, state);
    }
    let matched_substring = match_pattern(&haystack, &nfa);
    match matched_substring {
        None => println!("\nNo match found"),
        Some(substr) => println!("\nMatch found: {:?}", substr),
    }
}

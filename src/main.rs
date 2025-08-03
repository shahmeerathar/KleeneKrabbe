#![warn(clippy::pedantic)]
mod nfa;
mod parse;

use nfa::{compile, match_pattern};
use parse::parse;

fn main() {
    let haystack = String::from(
        "This string should not match: abbcbbba. This string should match: abbbbccbba.",
    );
    let needle = String::from("a((bb)|(cc))*a");
    println!("Finding pattern {needle} in:\n{haystack}");

    let postfix = parse(&needle);
    let nfa = compile(&postfix);
    if let Some(matched_substring) = match_pattern(&haystack, &nfa) {
        println!("Match found: {}", matched_substring);
    } else {
        println!("No match found");
    }
}

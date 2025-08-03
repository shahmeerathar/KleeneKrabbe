#![warn(clippy::pedantic)]
mod nfa;
mod parse;

use nfa::{compile, match_pattern};
use parse::parse;

fn main() {
    let haystack =
        String::from("This string should match: abbbbbbbba. This string should not match: abbbbba");
    let needle = String::from("a(bb)*a");
    println!("Finding pattern {needle} in:\n{haystack}");

    let postfix = parse(&needle);
    let nfa = compile(&postfix);
    let matched_substring = match_pattern(&haystack, &nfa);
    match matched_substring {
        None => println!("No match found"),
        Some(substr) => println!("Match found: {:?}", substr),
    }
}

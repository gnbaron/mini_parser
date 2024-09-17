use lexer::lexer;
use winnow::{Located, Parser};

mod error;
mod lexer;
mod types;

fn main() {
    let input = "3 - 1 + / |";
    let tokens = lexer.parse(Located::new(input));
    dbg!(tokens);
}

use std::{cell::RefCell, rc::Rc};

use state::ParserState;

mod ast;
mod error;
mod lexer;
mod parser;
mod state;
mod token;

fn main() {
    let input = "1 * 2 + 3";
    let state = Rc::new(RefCell::new(ParserState {
        input_text: input,
        errors: vec![],
    }));
    let tokens = lexer::lex(input, state.clone()).unwrap();
    dbg!(&tokens);
    let ast = parser::parse(&tokens, state.clone()).unwrap();
    dbg!(&ast);
    dbg!(&state.borrow().errors);
}

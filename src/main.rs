use state::ParserState;

mod error;
mod lexer;
mod state;
mod token;

fn main() {
    let input = "1 + 2 && 2.5";
    let mut errors = vec![];
    let state = ParserState {
        errors: &mut errors,
    };
    let tokens = lexer::lex(input, state);

    dbg!(tokens.unwrap());
    dbg!(errors);
}

use lexer::lexer;
use state::{ParserState, ParserStream};
use winnow::{Located, Parser};

mod error;
mod lexer;
mod state;
mod token;

fn main() {
    let input = "3 * 2.5 | 1";
    let mut errors = vec![];
    let stream = ParserStream {
        input: Located::new(&input),
        state: ParserState {
            errors: &mut errors,
        },
    };
    let tokens = lexer.parse(stream);

    dbg!(tokens.unwrap());
    dbg!(errors);
}

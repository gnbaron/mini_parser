use winnow::{
    ascii::{digit1, multispace0},
    combinator::{alt, delimited, dispatch, eof, fail, opt, peek, preceded, repeat_till, trace},
    stream::{Location, Stream},
    token::any,
    PResult, Parser,
};

use crate::{
    error::{expected, ParserError, ParserErrorType},
    state::ParserStream,
    token::{Token, TokenType},
};

pub fn lexer(input: &mut ParserStream) -> PResult<Vec<Token>> {
    repeat_till(0.., token, eof)
        .parse_next(input)
        .and_then(|(tokens, _)| Ok(tokens))
}

fn token(input: &mut ParserStream) -> PResult<Token> {
    let token = alt((number, operator, fail.context(expected("a valid token"))));

    match trace("token", delimited(multispace0, token, multispace0)).parse_next(input) {
        Ok(token) => Ok(token),
        _ => {
            let offset = input.location()..input.location() + 1;
            let token = input.next_slice(1);
            let message = format!("Unexpected token: {token}");

            let error = ParserError::new(ParserErrorType::Lex, message, offset.clone());
            input.state.add_error(error);

            Ok(Token::from((TokenType::Unknown, offset)))
        }
    }
}

fn number(input: &mut ParserStream) -> PResult<Token> {
    trace(
        "number",
        (digit1, opt(preceded('.', digit1)))
            .context(expected("a number"))
            .take()
            .value(TokenType::Number)
            .with_span()
            .map(Token::from),
    )
    .parse_next(input)
}

fn operator(input: &mut ParserStream) -> PResult<Token> {
    trace(
        "operator",
        dispatch! {peek(any);
            '+' => '+'.value(TokenType::Add),
            '-' => '-'.value(TokenType::Sub),
            '*' => '*'.value(TokenType::Mul),
            '/' => '/'.value(TokenType::Div),
            _ => fail,
        }
        .context(expected("an operator (+, -, /, *)"))
        .with_span()
        .map(Token::from),
    )
    .parse_next(input)
}

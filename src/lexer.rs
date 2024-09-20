use std::{cell::RefCell, rc::Rc};

use winnow::{
    ascii::{digit1, multispace0},
    combinator::{alt, delimited, dispatch, eof, fail, opt, peek, preceded, repeat_till, trace},
    stream::Location,
    token::any,
    Located, PResult, Parser, Stateful,
};

use crate::{
    error::{expected, ParserError, ParserErrorType},
    state::ParserState,
    token::{Token, TokenType},
};

type Stream<'a> = Stateful<Located<&'a str>, Rc<RefCell<ParserState<'a>>>>;

pub fn lex<'a>(input: &'a str, state: Rc<RefCell<ParserState<'a>>>) -> PResult<Vec<Token>> {
    let mut stream = Stream {
        input: Located::new(&input),
        state,
    };

    repeat_till(0.., token, eof)
        .parse_next(&mut stream)
        .and_then(|(tokens, _)| Ok(tokens))
}

fn token(input: &mut Stream) -> PResult<Token> {
    match trace("token", known_token).parse_next(input) {
        Ok(token) => Ok(token),
        Err(_) => {
            let start = input.location();
            let unexpected_chars = unknown_token.parse_next(input)?;
            let end = input.location();

            let offset = start..end;

            let message = format!("Unexpected character(s): {unexpected_chars}");
            let error = ParserError::new(ParserErrorType::Lex, message, offset.clone());
            input.state.borrow_mut().add_error(error);

            Ok(Token::from((TokenType::Unknown, offset)))
        }
    }
}

fn known_token(input: &mut Stream) -> PResult<Token> {
    let token = alt((number, operator, fail.context(expected("a valid token"))));
    trace("known_token", delimited(multispace0, token, multispace0)).parse_next(input)
}

fn unknown_token(input: &mut Stream) -> PResult<String> {
    let token_or_eof = alt((peek(known_token).void(), eof.void()));

    trace("unknown_token", repeat_till(0.., any, token_or_eof))
        .parse_next(input)
        .and_then(|(unexpected_chars, _)| Ok(unexpected_chars))
}

fn number(input: &mut Stream) -> PResult<Token> {
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

fn operator(input: &mut Stream) -> PResult<Token> {
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

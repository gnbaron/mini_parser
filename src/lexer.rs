use winnow::{
    ascii::{digit1, multispace0},
    combinator::{alt, delimited, dispatch, eof, fail, opt, peek, preceded, repeat_till, trace},
    token::any,
    Located, PResult, Parser,
};

use crate::{
    error::expected,
    types::{Token, TokenType},
};

pub fn lexer<'a>(input: &mut Located<&'a str>) -> PResult<Vec<Token>> {
    repeat_till(0.., token, eof)
        .parse_next(input)
        .and_then(|(tokens, _)| Ok(tokens))
}

fn token<'a>(input: &mut Located<&'a str>) -> PResult<Token> {
    let token = alt((number, operator, fail.context(expected("a valid token"))));
    trace("token", delimited(multispace0, token, multispace0)).parse_next(input)
}

fn number<'a>(input: &mut Located<&'a str>) -> PResult<Token> {
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

fn operator<'a>(input: &mut Located<&'a str>) -> PResult<Token> {
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

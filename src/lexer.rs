use winnow::{
    ascii::{digit1, multispace0},
    combinator::{alt, delimited, dispatch, eof, fail, opt, peek, preceded, repeat_till, trace},
    error::{StrContext, StrContextValue},
    token::any,
    Located, PResult, Parser,
};

use crate::types::{Token, TokenType};

pub fn lexer<'a>(input: &mut Located<&'a str>) -> PResult<Vec<Token>> {
    repeat_till(0.., token, eof)
        .parse_next(input)
        .and_then(|(tokens, _)| Ok(tokens))
}

fn token<'a>(input: &mut Located<&'a str>) -> PResult<Token> {
    let token = alt((
        number,
        operator,
        fail.context(StrContext::Label("token"))
            .context(StrContext::Expected(StrContextValue::Description(
                "valid number or operator",
            ))),
    ));
    trace("token", delimited(multispace0, token, multispace0)).parse_next(input)
}

fn number<'a>(input: &mut Located<&'a str>) -> PResult<Token> {
    trace(
        "number",
        (digit1, opt(preceded('.', digit1)))
            .context(StrContext::Label("number"))
            .context(StrContext::Expected(StrContextValue::Description(
                "a valid number",
            )))
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
            _ => fail,
        }
        .context(StrContext::Label("operator"))
        .context(StrContext::Expected(StrContextValue::Description(
            "a valid operator",
        )))
        .with_span()
        .map(Token::from),
    )
    .parse_next(input)
}

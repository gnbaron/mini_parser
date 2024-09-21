use std::{cell::RefCell, rc::Rc};

use winnow::{
    ascii::{digit1, multispace0},
    combinator::{alt, delimited, dispatch, eof, fail, opt, peek, preceded, repeat_till, trace},
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
    match trace("token", delimited(multispace0, known_token, multispace0)).parse_next(input) {
        Ok(token) => Ok(token),
        Err(_) => {
            let (unexpected, offset) =
                delimited(multispace0, unknown_token.with_span(), multispace0).parse_next(input)?;
            let message = format!("Unexpected token: {unexpected}");
            let error = ParserError::new(ParserErrorType::Lex, message, offset.clone());
            input.state.borrow_mut().add_error(error);

            Ok(Token::new((TokenType::Unknown, offset)))
        }
    }
}

fn known_token(input: &mut Stream) -> PResult<Token> {
    let token = alt((number, operator, fail.context(expected("a valid token"))));
    trace("known_token", token).parse_next(input)
}

fn unknown_token(input: &mut Stream) -> PResult<String> {
    let token_or_eof = peek(preceded(multispace0, alt((known_token.void(), eof.void()))));

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
            .map(Token::new),
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
        .map(Token::new),
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use pretty_assertions::assert_eq;

    fn assert_lex(input: &str, tokens: Vec<Token>) {
        let state = Rc::new(RefCell::new(ParserState::new(input)));
        let result = lex(input, state.clone());
        assert_eq!(result, Ok(tokens));
    }

    fn assert_error(input: &str, message: &str, offset: Range<usize>) {
        let state = Rc::new(RefCell::new(ParserState::new(input)));
        let _ = lex(input, state.clone());
        let expected = ParserError::new(ParserErrorType::Lex, String::from(message), offset);
        let actual = &state.borrow().errors;
        assert!(
            actual.contains(&expected),
            "expected {:?} to contain {:?}",
            actual,
            expected
        );
    }

    #[test]
    fn test_addition() {
        assert_lex(
            "2 + 1",
            vec![
                Token::new((TokenType::Number, 0..1)),
                Token::new((TokenType::Add, 2..3)),
                Token::new((TokenType::Number, 4..5)),
            ],
        );
    }

    #[test]
    fn test_subtraction() {
        assert_lex(
            "2.5 - 1",
            vec![
                Token::new((TokenType::Number, 0..3)),
                Token::new((TokenType::Sub, 4..5)),
                Token::new((TokenType::Number, 6..7)),
            ],
        );
    }

    #[test]
    fn test_multiplication() {
        assert_lex(
            "3 * 0.5",
            vec![
                Token::new((TokenType::Number, 0..1)),
                Token::new((TokenType::Mul, 2..3)),
                Token::new((TokenType::Number, 4..7)),
            ],
        );
    }

    #[test]
    fn test_division() {
        assert_lex(
            "4.5 / 0.5",
            vec![
                Token::new((TokenType::Number, 0..3)),
                Token::new((TokenType::Div, 4..5)),
                Token::new((TokenType::Number, 6..9)),
            ],
        );
    }

    #[test]
    fn test_unknown_token() {
        assert_lex(
            "1 + a",
            vec![
                Token::new((TokenType::Number, 0..1)),
                Token::new((TokenType::Add, 2..3)),
                Token::new((TokenType::Unknown, 4..5)),
            ],
        );
        assert_lex(
            "1 & 2",
            vec![
                Token::new((TokenType::Number, 0..1)),
                Token::new((TokenType::Unknown, 2..3)),
                Token::new((TokenType::Number, 4..5)),
            ],
        );
        assert_lex(
            "1.5 || 2",
            vec![
                Token::new((TokenType::Number, 0..3)),
                Token::new((TokenType::Unknown, 4..6)),
                Token::new((TokenType::Number, 7..8)),
            ],
        );
        assert_lex(
            "0.5 %% ",
            vec![
                Token::new((TokenType::Number, 0..3)),
                Token::new((TokenType::Unknown, 4..6)),
            ],
        );
        assert_lex("%", vec![Token::new((TokenType::Unknown, 0..1))]);
    }

    #[test]
    fn test_error_message() {
        assert_error("1 + a", "Unexpected token: a", 4..5);
        assert_error("1 & 2", "Unexpected token: &", 2..3);
        assert_error("1.5 || 2", "Unexpected token: ||", 4..6);
        assert_error("0.5 %% ", "Unexpected token: %%", 4..6);
        assert_error("%", "Unexpected token: %", 0..1);
    }

    #[test]
    fn test_unary() {
        assert_lex("1", vec![Token::new((TokenType::Number, 0..1))]);
        assert_lex("+", vec![Token::new((TokenType::Add, 0..1))]);
        assert_lex("1.5", vec![Token::new((TokenType::Number, 0..3))]);
    }
}

use std::{cell::RefCell, rc::Rc};

use winnow::{
    combinator::{preceded, repeat, trace},
    error::ParserError,
    token::one_of,
    PResult, Parser, Stateful,
};

use crate::{
    ast::{ASTMetadata, AST},
    state::ParserState,
    token::{Token, TokenType},
};

type Stream<'a> = Stateful<&'a [Token], Rc<RefCell<ParserState<'a>>>>;

pub fn parse<'a>(tokens: &'a [Token], state: Rc<RefCell<ParserState<'a>>>) -> PResult<AST> {
    let mut stream = Stream {
        input: tokens,
        state,
    };

    expression.parse_next(&mut stream)
}

fn expression(input: &mut Stream) -> PResult<AST> {
    trace("expression", addition).parse_next(input)
}

fn addition(input: &mut Stream) -> PResult<AST> {
    trace(
        "addition",
        binary_expression(subtraction, TokenType::Add, subtraction),
    )
    .parse_next(input)
}

fn subtraction(input: &mut Stream) -> PResult<AST> {
    trace(
        "subtraction",
        binary_expression(multiplication, TokenType::Sub, multiplication),
    )
    .parse_next(input)
}

fn multiplication(input: &mut Stream) -> PResult<AST> {
    trace(
        "multiplication",
        binary_expression(division, TokenType::Mul, division),
    )
    .parse_next(input)
}

fn division(input: &mut Stream) -> PResult<AST> {
    trace(
        "division",
        binary_expression(number, TokenType::Div, number),
    )
    .parse_next(input)
}

fn number(input: &mut Stream) -> PResult<AST> {
    let input_text = input.state.borrow().input_text;

    trace("number", one_of(TokenType::Number))
        .map(|token: Token| {
            let text = token.text(input_text);
            let value = text.parse::<f32>().unwrap();

            AST::Number {
                metadata: ASTMetadata {
                    start_offset: token.start_offset,
                    end_offset: token.end_offset,
                },
                value,
            }
        })
        .parse_next(input)
}

pub fn binary_expression<'a, P, E>(
    mut lhs_parser: P,
    operator_token: TokenType,
    mut rhs_parser: P,
) -> impl Parser<Stream<'a>, AST, E>
where
    P: Parser<Stream<'a>, AST, E>,
    E: ParserError<Stream<'a>>,
{
    move |input: &mut Stream<'a>| {
        (
            lhs_parser.by_ref(),
            repeat(0.., preceded(one_of(operator_token), rhs_parser.by_ref())),
        )
            .map(|(lhs, rhs_vec): (AST, Vec<AST>)| {
                rhs_vec
                    .into_iter()
                    .fold(lhs, |new_lhs, rhs| AST::BinaryExpression {
                        metadata: ASTMetadata {
                            start_offset: new_lhs.metadata().start_offset,
                            end_offset: rhs.metadata().end_offset,
                        },
                        lhs: Box::new(new_lhs),
                        token_type: operator_token,
                        rhs: Box::new(rhs),
                    })
            })
            .parse_next(input)
    }
}

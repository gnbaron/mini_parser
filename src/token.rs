use std::ops::Range;

use winnow::stream::ContainsToken;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Add,
    Sub,
    Mul,
    Div,
    Number,
    Unknown,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub start_offset: usize,
    pub end_offset: usize,
}

impl Token {
    pub fn new((token_type, span): (TokenType, Range<usize>)) -> Token {
        Token {
            token_type,
            start_offset: span.start,
            end_offset: span.end,
        }
    }

    pub fn text<'a>(&self, input: &'a str) -> &'a str {
        &input[self.start_offset as usize..self.end_offset as usize]
    }
}

impl ContainsToken<Token> for TokenType {
    #[inline(always)]
    fn contains_token(&self, token: Token) -> bool {
        *self == token.token_type
    }
}

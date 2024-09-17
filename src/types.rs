use std::ops::Range;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    start_offset: usize,
    end_offset: usize,
}

impl Token {
    pub fn from((token_type, span): (TokenType, Range<usize>)) -> Token {
        Token {
            token_type,
            start_offset: span.start,
            end_offset: span.end,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Add,
    Sub,
    Mul,
    Div,
    Number,
}

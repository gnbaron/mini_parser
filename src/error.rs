use std::{ops::Range, usize};

use winnow::error::{StrContext, StrContextValue};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserErrorType {
    Lex,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParserError {
    error_type: ParserErrorType,
    message: String,
    offset: Range<usize>,
}

impl ParserError {
    pub fn new(error_type: ParserErrorType, message: String, offset: Range<usize>) -> Self {
        Self {
            error_type,
            message,
            offset,
        }
    }
}

pub fn expected(what: &'static str) -> StrContext {
    StrContext::Expected(StrContextValue::Description(what))
}

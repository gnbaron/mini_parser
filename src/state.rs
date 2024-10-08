use crate::error::ParserError;

#[derive(Debug)]
pub struct ParserState<'a> {
    pub input_text: &'a str,
    pub errors: Vec<ParserError>,
}

impl<'a> ParserState<'a> {
    pub fn new(input_text: &'a str) -> Self {
        Self {
            input_text,
            errors: vec![],
        }
    }

    pub fn add_error(&mut self, error: ParserError) {
        self.errors.push(error);
    }
}

use crate::error::ParserError;

#[derive(Debug)]
pub struct ParserState<'a> {
    pub errors: &'a mut Vec<ParserError>,
}

impl<'a> ParserState<'a> {
    pub fn add_error(&mut self, error: ParserError) {
        self.errors.push(error);
    }
}

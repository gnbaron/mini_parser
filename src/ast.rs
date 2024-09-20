use crate::token::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub struct ASTMetadata {
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AST {
    BinaryExpression {
        metadata: ASTMetadata,
        lhs: Box<AST>,
        token_type: TokenType,
        rhs: Box<AST>,
    },
    Number {
        metadata: ASTMetadata,
        value: f32,
    },
}

impl AST {
    pub fn metadata(&self) -> &ASTMetadata {
        match self {
            AST::BinaryExpression { metadata, .. } => metadata,
            AST::Number { metadata, .. } => metadata,
        }
    }
}

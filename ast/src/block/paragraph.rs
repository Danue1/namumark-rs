use crate::Span;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct Paragraph {
    pub spans: Vec<Span>,
    pub node: SyntaxNode,
}

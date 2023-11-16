use crate::Paragraph;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct SemanticSpan {
    pub kind: SemanticSpanKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum SemanticSpanKind {
    Delete(Paragraph),
    Emphasis(Paragraph),
    Strong(Paragraph),
    Subscript(Paragraph),
    Superscript(Paragraph),
    Underline(Paragraph),
    NewLine,
}

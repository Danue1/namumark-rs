use crate::FontSizeLevel;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct SizeBracketSpan {
    pub down: bool,
    pub level: FontSizeLevel,
    pub node: SyntaxNode,
}

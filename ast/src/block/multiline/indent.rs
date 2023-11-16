use crate::MultilineBlock;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct IndentBlock {
    pub blocks: Vec<MultilineBlock>,
    pub node: SyntaxNode,
}

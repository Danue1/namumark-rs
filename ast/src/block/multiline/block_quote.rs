use crate::MultilineBlock;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct BlockQuoteBlock {
    pub blocks: Vec<MultilineBlock>,
    pub node: SyntaxNode,
}

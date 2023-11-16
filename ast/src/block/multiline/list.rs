use crate::MultilineBlock;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct ListBlock {
    pub blocks: Vec<MultilineBlock>,
    pub index: Option<ListIndex>,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub struct ListIndex {
    pub node: SyntaxNode,
}

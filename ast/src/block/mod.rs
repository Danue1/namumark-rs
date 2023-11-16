mod multiline;
mod paragraph;
mod singleline;

pub use multiline::*;
pub use paragraph::*;
pub use singleline::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct Block {
    pub kind: BlockKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum BlockKind {
    Singleline(SinglelineBlock),
    Multiline(MultilineBlock),
}

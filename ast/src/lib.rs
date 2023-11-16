mod block;
mod span;

pub use block::*;
pub use span::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct Ast {
    pub blocks: Vec<Block>,
    pub node: SyntaxNode,
}

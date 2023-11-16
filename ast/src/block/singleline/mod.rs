mod comment;
mod heading;

pub use comment::*;
pub use heading::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct SinglelineBlock {
    pub kind: SinglelineBlockKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum SinglelineBlockKind {
    Comment(CommentBlock),
    Heading(HeadingBlock),
}

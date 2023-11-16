mod block_quote;
mod indent;
mod list;

use crate::Paragraph;
pub use block_quote::*;
pub use indent::*;
pub use list::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct MultilineBlock {
    pub kind: MultilineBlockKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum MultilineBlockKind {
    HorizontalRule,
    BlockQuote(BlockQuoteBlock),
    Indent(IndentBlock),
    List(ListBlock),
    Paragraph(Paragraph),
}

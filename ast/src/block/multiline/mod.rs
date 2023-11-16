mod block_quote;
mod horizontal_rule;
mod indent;
mod list;

use crate::Paragraph;
pub use block_quote::*;
pub use horizontal_rule::*;
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
    HorizontalRule(HorizontalRuleBlock),
    BlockQuote(BlockQuoteBlock),
    Indent(IndentBlock),
    Paragraph(Paragraph),
    List(ListBlock),
}

mod color;
mod folding;
mod inline;
mod size;
mod syntax_highlight;

pub use color::*;
pub use folding::*;
pub use inline::*;
pub use size::*;
pub use syntax_highlight::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct BracketSpan {
    pub kind: BracketSpanKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum BracketSpanKind {
    Color(ColorBracketSpan),
    Folding(FoldingBracketSpan),
    Inline(InlineBracketSpan),
    Size(SizeBracketSpan),
    SyntaxHighlight(SyntaxHighlightBracketSpan),
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum FontSizeLevel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

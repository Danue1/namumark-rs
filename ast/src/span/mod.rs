mod bracket;
mod command;
mod inline;
mod macros;
mod semantic;

pub use bracket::*;
pub use command::*;
pub use inline::*;
pub use macros::*;
pub use semantic::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct Span {
    pub kind: SpanKind,
    pub nod: SyntaxNode,
}

#[derive(Debug)]
pub enum SpanKind {
    Semantic(SemanticSpan),
    Bracket(BracketSpan),
    Macro(MacroSpan),
    Command(CommandSpan),
    Inline(InlineSpan),
}

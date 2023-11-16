mod age;
mod anchor;
mod comment;
mod date;
mod date_time;
mod dday;
mod footnote;
mod include;
mod latex;
mod page_count;
mod ruby;

pub use age::*;
pub use anchor::*;
pub use comment::*;
pub use date::*;
pub use date_time::*;
pub use dday::*;
pub use footnote::*;
pub use include::*;
pub use latex::*;
pub use page_count::*;
pub use ruby::*;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct MacroSpan {
    pub kind: MacroSpanKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum MacroSpanKind {
    Age(AgeMacroSpan),
    Anchor(AnchorMacroSpan),
    Comment(CommentMacroSpan),
    Date(DateMacroSpan),
    DateTime(DateTimeMacroSpan),
    DDay(DDayMacroSpan),
    Footnote(FootnoteMacroSpan),
    Include(IncludeMacroSpan),
    Latex(LatexMacroSpan),
    PageCount(PageCountMacroSpan),
    Ruby(RubyMacroSpan),
    TableOfContents,
    NewLIne,
}

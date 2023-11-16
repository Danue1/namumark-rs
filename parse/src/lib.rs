#[macro_use]
mod context;
mod ast;

use context::{Context, State};
use syntax_kind::{SyntaxKind, SyntaxNode};

pub fn parse(tokens: impl Iterator<Item = (SyntaxKind, String)>) -> SyntaxNode {
    let mut context = Context::new(tokens);
    context.ast();
    context.finish()
}

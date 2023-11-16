mod block_quote_block;
mod horizontal_block;
mod indent_block;
mod list_block;

use crate::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn multiline_block(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if one_of!(
            self.horizontal_block(),
            self.block_quote_block(),
            self.indent_block(),
            self.list_block(),
            self.paragraph(),
        ) == State::Continue
        {
            State::Continue
        } else {
            self.start_node_at(checkpoint, SyntaxKind::MULTILINE_BLOCK_NODE);
            self.finish_node();
            State::Stop
        }
    }
}

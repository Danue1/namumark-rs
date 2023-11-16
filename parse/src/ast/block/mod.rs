mod multiline;
mod paragraph;
mod singleline;

use crate::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn block(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if one_of!(self.singleline_block(), self.multiline_block(),) == State::Continue {
            State::Continue
        } else {
            self.start_node_at(checkpoint, SyntaxKind::BLOCK_NODE);
            self.finish_node();
            State::Stop
        }
    }
}

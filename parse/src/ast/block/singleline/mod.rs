mod comment;
mod heading;

use crate::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn singleline_block(&mut self) -> State {
        let checkpoint = self.checkpoint();
        if one_of!(self.comment_block(), self.heading_block(),) == State::Continue {
            State::Continue
        } else {
            self.start_node_at(checkpoint, SyntaxKind::SINGLELINE_BLOCK_NODE);
            self.finish_node();
            State::Stop
        }
    }
}

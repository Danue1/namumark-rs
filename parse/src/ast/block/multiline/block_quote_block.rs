use crate::{Context, State};

impl Context {
    pub fn block_quote_block(&mut self) -> State {
        State::Continue
    }
}

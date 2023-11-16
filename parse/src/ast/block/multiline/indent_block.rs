use crate::{Context, State};

impl Context {
    pub fn indent_block(&mut self) -> State {
        State::Continue
    }
}

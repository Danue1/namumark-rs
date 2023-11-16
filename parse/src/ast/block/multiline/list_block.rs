use crate::{Context, State};

impl Context {
    pub fn list_block(&mut self) -> State {
        State::Continue
    }
}

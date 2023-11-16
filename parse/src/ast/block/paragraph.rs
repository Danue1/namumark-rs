use crate::{Context, State};

impl Context {
    pub fn paragraph(&mut self) -> State {
        State::Continue
    }
}

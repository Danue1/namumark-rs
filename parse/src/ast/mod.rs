mod block;

use crate::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn ast(&mut self) {
        self.start_node(SyntaxKind::AST_NODE);
        if self.block() == State::Continue {
            self.text();
        } else {
            self.new_line();
        }
        self.finish_node();
    }

    pub fn text(&mut self) {
        self.start_node(SyntaxKind::TEXT_NODE);
        self.bump();
        self.finish_node();
    }

    pub fn new_line(&mut self) {
        while !self.is_empty() {
            if self.peek() == SyntaxKind::NEW_LINE {
                self.bump();
            }
        }
    }

    pub fn skip_line(&mut self) -> bool {
        let mut skipped = false;
        while !self.is_empty() {
            if self.peek() != SyntaxKind::NEW_LINE {
                skipped = true;
                self.bump();
            }
        }
        skipped
    }
}

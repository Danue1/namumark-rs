use crate::{Context, State};
use syntax_kind::SyntaxKind;

impl Context {
    pub fn comment_block(&mut self) -> State {
        if self.peek() == SyntaxKind::HASH
            && self.nth(1) == SyntaxKind::HASH
            && self.nth(2) == SyntaxKind::HASH
        {
            self.start_node(SyntaxKind::COMMENT_BLOCK_NODE);
            self.bump();
            self.bump();
            self.bump();
            self.skip_whitespace();
            let checkpoint = self.checkpoint();
            if self.skip_line() {
                self.start_node_at(checkpoint, SyntaxKind::TEXT_NODE);
                self.finish_node();
            }
            self.finish_node();
            State::Stop
        } else {
            State::Continue
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn comment() {
        let source = "### comment";
        let tokens = lex::lex(source);
        let node = crate::parse(tokens);
        insta::assert_debug_snapshot!(node);
    }
}

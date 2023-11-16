use rowan::{Checkpoint, GreenNodeBuilder};
use syntax_kind::{SyntaxKind, SyntaxNode};

#[derive(Debug)]
pub struct Context {
    tokens: Vec<(SyntaxKind, String)>,
    builder: GreenNodeBuilder<'static>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Continue,
    Stop,
}

#[allow(unused)]
impl Context {
    pub fn new(tokens: impl Iterator<Item = (SyntaxKind, String)>) -> Self {
        let mut tokens: Vec<_> = tokens.collect();
        tokens.reverse();
        Self {
            tokens,
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn peek(&self) -> SyntaxKind {
        match self.tokens.last() {
            Some((kind, _)) => *kind,
            None => SyntaxKind::EOF,
        }
    }

    pub fn peek_str(&self) -> &str {
        match self.tokens.last() {
            Some((_, text)) => text.as_str(),
            None => "",
        }
    }

    pub fn nth(&self, size: usize) -> SyntaxKind {
        match self.tokens.get(self.tokens.len() - size) {
            Some((kind, _)) => *kind,
            None => SyntaxKind::EOF,
        }
    }

    pub fn bump(&mut self) {
        if let Some((kind, text)) = self.tokens.pop() {
            self.builder.token(kind.into(), text.as_str());
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek() == SyntaxKind::WHITESPACE {
            self.bump();
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    #[inline]
    pub fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    #[inline]
    pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    #[inline]
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    #[inline]
    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    #[inline]
    pub fn finish(self) -> SyntaxNode {
        SyntaxNode::new_root(self.builder.finish())
    }
}

#[macro_export]
macro_rules! one_of {
    ($head:expr,) => {
        $head
    };
    ($head:expr, $($tail:expr,)+) => {
        if $head == crate::State::Continue {
            one_of!($($tail,)+)
        } else {
            crate::State::Stop
        }
    };
}

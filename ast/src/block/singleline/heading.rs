use crate::Paragraph;
use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct HeadingBlock {
    pub level: HeadingLevel,
    pub folding: bool,
    pub paragraph: Paragraph,
    pub node: SyntaxNode,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HeadingLevel {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

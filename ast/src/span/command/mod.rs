mod category;
mod image;
mod link;
mod video;

pub use category::*;
pub use image::*;
pub use link::*;
use syntax_kind::SyntaxNode;
pub use video::*;

#[derive(Debug)]
pub struct CommandSpan {
    pub kind: CommandSpanKind,
    pub node: SyntaxNode,
}

#[derive(Debug)]
pub enum CommandSpanKind {
    Category(CategoryCommandSpan),
    Image(ImageCommandSpan),
    Link(LinkCommandSpan),
    Video(VideoCommandSpan),
}

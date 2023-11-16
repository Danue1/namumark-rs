use syntax_kind::SyntaxNode;

#[derive(Debug)]
pub struct CommentBlock {
    pub text: String,
    pub node: SyntaxNode,
}

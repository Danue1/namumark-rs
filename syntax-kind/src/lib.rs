pub type SyntaxNode = rowan::SyntaxNode<SyntaxKind>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    EOF,
    UNKNOWN,

    // for Tokens

    // punctuation
    LEFT_PAREN,    // (
    RIGHT_PAREN,   // )
    LEFT_BRACE,    // {
    RIGHT_BRACE,   // }
    LEFT_BRACKET,  // [
    RIGHT_BRACKET, // ]
    LEFT_CHEVRON,  // <
    RIGHT_CHEVRON, // >
    COMMA,         // ,
    DOT,           // .
    MINUS,         // -
    PLUS,          // +
    COLON,         // :
    SEMICOLON,     // ;
    EQUAL,         // =
    SLASH,         // /
    BACKSLASH,     // \
    STAR,          // *
    HASH,          // #
    AT,            // @
    DOLLAR,        // $
    PERCENT,       // %
    CARET,         // ^
    AMPERSAND,     // &
    PIPE,          // |
    TILDE,         // ~
    UNDERSCORE,    // _
    QUESTION,      // ?
    EXCLAMATION,   // !
    SINGLE_QUOTE,  // '
    DOUBLE_QUOTE,  // "
    BACKTICK,      // `
    WHITESPACE,    // ' '
    TAB,           // \t

    // literal
    NUMERIC, // /[0-9]+/
    TEXT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

impl rowan::Language for SyntaxKind {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

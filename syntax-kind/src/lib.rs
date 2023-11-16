pub type SyntaxNode = rowan::SyntaxNode<SyntaxKind>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
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
    NEW_LINE,      // \n

    // literal
    NUMERIC, // /[0-9]+/
    TEXT,

    // for Nodes
    EOF,

    // AST
    // = BLOCK*
    AST_NODE,

    // BLOCK
    // = SINGLELINE_BLOCK
    // | MULTILINE_BLOCK
    BLOCK_NODE,

    // SINGLELINE_BLOCK
    // = COMMENT_BLOCK
    // | HEADING_BLOCK
    SINGLELINE_BLOCK_NODE,

    // MULTILINE_BLOCK
    // = HORIZONTAL_RULE_BLOCK
    // | BLOCK_QUOTE_BLOCK
    // | INDENT_BLOCK
    // | LIST_BLOCK
    // | PARAGRAPH
    MULTILINE_BLOCK_NODE,

    // COMMENT_BLOCK
    // = '##' [^\n]*
    COMMENT_BLOCK_NODE,

    // HEADING_BLOCK
    // = '=' PARAGRAPH '='
    // | '==' PARAGRAPH '=='
    // | '===' PARAGRAPH '==='
    // | '====' PARAGRAPH '===='
    // | '=====' PARAGRAPH '====='
    // | '======' PARAGRAPH '======'
    /// | '=#' PARAGRAPH '#='
    /// | '==#' PARAGRAPH '#=='
    /// | '===#' PARAGRAPH '#==='
    /// | '====#' PARAGRAPH '#===='
    /// | '=====#' PARAGRAPH '#====='
    /// | '======#' PARAGRAPH '#======'
    HEADING_BLOCK_NODE,

    // HORIZONTAL_RULE_BLOCK
    // = '---'
    // | '----'
    // | '-----'
    // | '------'
    // | '-------'
    // | '--------'
    // | '---------'
    // | '----------'
    HORIZONTAL_RULE_BLOCK_NODE,

    // BLOCK_QUOTE_BLOCK
    // = '>' WHITESPACE? MULTI_LINE_BLOCK*
    BLOCK_QUOTE_BLOCK_NODE,

    // INDENT_BLOCK
    // = ' ' WHITESPACE? MULTI_LINE_BLOCK*
    INDENT_BLOCK_NODE,

    // LIST_BLOCK
    // = ' *' WHITESPACE? MULTI_LINE_BLOCK*
    // | ' ' NUMERIC '.' WHITESPACE? MULTI_LINE_BLOCK*
    LIST_BLOCK_NODE,

    // PARAGRAPH
    // = SPAN*
    PARAGRAPH_NODE,

    // SPAN
    // = SEMANTIC_SPAN
    // | BRACKET_SPAN
    // | MACRO_SPAN
    // | COMMAND_SPAN
    // | INLINE_SPAN
    SPAN_NODE,

    // SEMANTIC_SPAN
    // = DELETE_SPAN
    // | EMPHASIS_SPAN
    // | STRONG_SPAN
    // | SUBSCRIPT_SPAN
    // | SUPERSCRIPT_SPAN
    // | UNDERLINE_SPAN
    // | NEW_LINE_SPAN
    SEMANTIC_SPAN_NODE,

    // BRACKET_SPAN
    BRACKET_SPAN_NODE,

    // MACRO_SPAN
    MACRO_SPAN_NODE,

    // COMMAND_SPAN
    COMMAND_SPAN_NODE,

    // DELETE_SPAN
    // = '~~' SPAN '~~'
    // | '--' SPAN '--'
    DELETE_SEMANTIC_SPAN_NODE,

    // EMPHASIS_SPAN
    // = '\'\'' SPAN '\'\''
    EMPHASIS_SEMANTIC_SPAN_NODE,

    // STRONG_SPAN
    // = '\'\'\' SPAN '\'\'\''
    STRONG_SEMANTIC_SPAN_NODE,

    // SUBSCRIPT_SPAN
    // = ',,' SPAN ',,'
    SUBSCRIPT_SEMANTIC_SPAN_NODE,

    // SUPERSCRIPT_SPAN
    // = '^^' SPAN '^^'
    SUPERSCRIPT_SEMANTIC_SPAN_NODE,

    // NEW_LINE_SPAN
    // = '\n'
    NEW_LINE_SEMANTIC_SPAN_NODE,

    TEXT_NODE,

    // WHITESPACE
    // = ' '+
    WHITESPACE_NODE,
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

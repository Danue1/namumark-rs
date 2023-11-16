mod context;

use context::Context;
use syntax_kind::SyntaxKind;

pub fn lex(source: &str) -> impl Iterator<Item = (SyntaxKind, String)> + '_ {
    let mut context = Context::new(source);
    std::iter::from_fn(move || {
        let start = context.index();
        let token = lex_token(&mut context)?;
        let text = context.slice(start);
        Some((token, text))
    })
}

fn lex_token(context: &mut Context) -> Option<SyntaxKind> {
    context.bump().map(|c| {
        if let Some(kind) = punctuation(c) {
            kind
        } else if c.is_numeric() {
            numeric(context)
        } else {
            text(context)
        }
    })
}

const fn punctuation(c: char) -> Option<SyntaxKind> {
    let kind = match c {
        '(' => SyntaxKind::LEFT_PAREN,
        ')' => SyntaxKind::RIGHT_PAREN,
        '{' => SyntaxKind::LEFT_BRACE,
        '}' => SyntaxKind::RIGHT_BRACE,
        '[' => SyntaxKind::LEFT_BRACKET,
        ']' => SyntaxKind::RIGHT_BRACKET,
        '<' => SyntaxKind::LEFT_CHEVRON,
        '>' => SyntaxKind::RIGHT_CHEVRON,
        ',' => SyntaxKind::COMMA,
        '.' => SyntaxKind::DOT,
        '-' => SyntaxKind::MINUS,
        '+' => SyntaxKind::PLUS,
        ':' => SyntaxKind::COLON,
        ';' => SyntaxKind::SEMICOLON,
        '=' => SyntaxKind::EQUAL,
        '/' => SyntaxKind::SLASH,
        '\\' => SyntaxKind::BACKSLASH,
        '*' => SyntaxKind::STAR,
        '#' => SyntaxKind::HASH,
        '@' => SyntaxKind::AT,
        '$' => SyntaxKind::DOLLAR,
        '%' => SyntaxKind::PERCENT,
        '^' => SyntaxKind::CARET,
        '&' => SyntaxKind::AMPERSAND,
        '|' => SyntaxKind::PIPE,
        '~' => SyntaxKind::TILDE,
        '_' => SyntaxKind::UNDERSCORE,
        '?' => SyntaxKind::QUESTION,
        '!' => SyntaxKind::EXCLAMATION,
        '\'' => SyntaxKind::SINGLE_QUOTE,
        '"' => SyntaxKind::DOUBLE_QUOTE,
        '`' => SyntaxKind::BACKTICK,
        ' ' => SyntaxKind::WHITESPACE,
        '\t' => SyntaxKind::TAB,
        '\n' => SyntaxKind::NEW_LINE,
        _ => return None,
    };
    Some(kind)
}

fn numeric(context: &mut Context) -> SyntaxKind {
    while let Some(c) = context.peek() {
        if !c.is_numeric() {
            break;
        }
        context.bump();
    }
    SyntaxKind::NUMERIC
}

fn text(context: &mut Context) -> SyntaxKind {
    while let Some(c) = context.peek() {
        if punctuation(c).is_some() || c.is_numeric() {
            break;
        }
        context.bump();
    }
    SyntaxKind::TEXT
}

#[cfg(test)]
mod tests {
    use syntax_kind::SyntaxKind;

    macro_rules! assert_snapshot {
        ($source:expr) => {
            let source = $source;
            let tokens: Vec<(SyntaxKind, String)> = super::lex(source).collect();
            insta::assert_debug_snapshot!(tokens);
        };
    }

    #[test]
    fn punctuation() {
        assert_snapshot!("(){}[]<>,.-+:;=/\\*#@$%^&|~_?!'\"` \t\n");
    }

    #[test]
    fn numeric() {
        assert_snapshot!("1234567890");
    }

    #[test]
    fn text_alphabet() {
        assert_snapshot!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    #[test]
    fn text_hangul() {
        assert_snapshot!("가나다라마바사아자차카타파하");
    }
}

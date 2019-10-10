#[macro_export]
macro_rules! tok {
    (..) => {
        $crate::syntax::SyntaxKind::TOK_DOT_DOT
    };
    ( - ) => {
        $crate::syntax::SyntaxKind::TOK_HYPHEN
    };
    (; ) => {
        $crate::syntax::SyntaxKind::TOK_SEMICOLON
    };

    ( if ) => {
        $crate::syntax::SyntaxKind::TOK_IF_KW
    };
    ( else ) => {
        $crate::syntax::SyntaxKind::TOK_ELSE_KW
    };
    (true) => {
        $crate::syntax::SyntaxKind::TOK_TRUE
    };

    (false) => {
        $crate::syntax::SyntaxKind::TOK_FALSE
    };
    ('(') => {
        $crate::syntax::SyntaxKind::TOK_OPEN_PARENTHESIS
    };

    (')') => {
        $crate::syntax::SyntaxKind::TOK_CLOSE_PARENTHESIS
    };
    ('{') => {
        $crate::syntax::SyntaxKind::TOK_OPEN_BRACE
    };
    ('}') => {
        $crate::syntax::SyntaxKind::TOK_CLOSE_BRACE
    };
    (; ) => {
        $crate::syntax::SyntaxKind::TOK_SEMICOLON
    };
    (.) => {
        $crate::syntax::SyntaxKind::TOK_DOT
    };

    (..) => {
        $crate::syntax::SyntaxKind::TOK_DOT_DOT
    };
    (: ) => {
        $crate::syntax::SyntaxKind::TOK_COLON
    };
    ( - ) => {
        $crate::syntax::SyntaxKind::TOK_HYPHEN
    };
    (, ) => {
        $crate::syntax::SyntaxKind::TOK_COMMA
    };
    ( = ) => {
        $crate::syntax::SyntaxKind::TOK_EQUALS
    };
    ( & ) => {
        $crate::syntax::SyntaxKind::TOK_AMPERSAND
    };
    ( | ) => {
        $crate::syntax::SyntaxKind::TOK_PIPE
    };
    ( ^ ) => {
        $crate::syntax::SyntaxKind::TOK_CARET
    };
    (~) => {
        $crate::syntax::SyntaxKind::TOK_TILDE
    };
    ( && ) => {
        $crate::syntax::SyntaxKind::TOK_DOUBLE_AMPERSAND
    };
    ( || ) => {
        $crate::syntax::SyntaxKind::TOK_DOUBLE_PIPE
    };
    ( ! ) => {
        $crate::syntax::SyntaxKind::TOK_EXCLAMATION
    };
    ( |= ) => {
        $crate::syntax::SyntaxKind::TOK_PIPE_EQUALS
    };
}

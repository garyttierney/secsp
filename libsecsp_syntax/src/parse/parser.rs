use codespan_reporting::Severity;
use std::borrow::Borrow;

use crate::ast::{
    ContainerType, ExpressionNode, Ident, Module, NodeId, Path, StatementKind, StatementNode,
    SymbolType,
};
use crate::keywords;
use crate::lex::token::{DelimiterType, Token};
use crate::lex::token_cursor::TokenCursor;
use crate::lex::token_tree::TokenTree;
use crate::lex::{ByteIndex, ByteSpan};
use crate::parse::expr::ExprContext;
use crate::parse::parser_from_source;
use crate::parse::stmt::InitializerKind;
use crate::parse::stmt::StatementType;
use crate::parse::ParseResult;
use crate::session::ParseSession;

pub struct ParseError {}

pub struct Parser<'sess> {
    cursor: TokenCursor,
    session: &'sess ParseSession,
    token: Token,
    span: ByteSpan,
    last_span: ByteSpan,
}

macro_rules! keyword {
    ($kw:ident) => {
        Token::Name(keywords::$kw.into())
    };
}

impl<'sess> Parser<'sess> {
    pub fn new(session: &'sess ParseSession, trees: Vec<TokenTree>) -> Self {
        let mut cursor = TokenCursor::new(trees);
        let (token, span) = cursor.advance().consume();

        Parser {
            cursor,
            session,
            token,
            span,
            last_span: span,
        }
    }

    pub fn parse_module(&mut self) -> ParseResult<Module> {
        let mut statements: Vec<StatementNode> = vec![];

        while !self.eat(Token::Eof) {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(mut e) => {
                    e.emit();
                    // @todo - recover
                }
            }
        }

        unimplemented!()
    }

    pub fn parse_statement(&mut self) -> ParseResult<StatementNode> {
        use self::Token::*;

        let start = self.span;
        let stmt_ty = if let Name(ref val) = self.token {
            StatementType::from_keyword(val)
        } else {
            return Err(self
                .session
                .diagnostic(Severity::Error, "unrecognized keyword")
                .span_err(start, "found here"));
        };

        let requires_semicolon = stmt_ty.as_ref().map(|ty| !ty.is_block()).unwrap_or(true);

        let kind = Box::new(match stmt_ty {
            Some(StatementType::ContainerDeclaration) => self.parse_container_decl()?,
            Some(StatementType::MacroDeclaration) => self.parse_macro()?,
            Some(StatementType::SymbolDeclaration(ty, init)) => self.parse_symbol_decl(ty, init)?,
            None => {
                let path = self.parse_path()?;
                let stmt = match self.token {
                    Token::OpenDelimiter(DelimiterType::Parenthesis) => self.parse_arg_list(path)?,
                    Token::SetModifier => self.parse_set_modifier(path)?,
                    _ => return Err(self.session.diagnostic(Severity::Error, "unexpected token")),
                };

                stmt
            }
            _ => unimplemented!(),
        });

        if requires_semicolon && !self.eat(Token::Semicolon) {
            self.session
                .diagnostic(Severity::Error, "missing semicolon")
                .span_err(self.span, "expected here")
                .emit();
        }

        Ok(StatementNode {
            node_id: NodeId(0),
            kind,
            span: start.to(self.last_span),
        })
    }

    fn parse_arg_list(&mut self, name: Path) -> ParseResult<StatementKind> {
        let params = vec![];

        self.expect(Token::OpenDelimiter(DelimiterType::Parenthesis))?;
        while !self.eat(Token::CloseDelimiter(DelimiterType::Parenthesis)) {
            let _param = self.parse_expr(ExprContext::Any);

            if !self.eat(Token::Comma)
                && self.token != Token::CloseDelimiter(DelimiterType::Parenthesis)
            {}
        }

        Ok(StatementKind::MacroCall(name, params))
    }

    fn parse_statement_block(&mut self) -> ParseResult<Vec<StatementNode>> {
        self.expect(Token::OpenDelimiter(DelimiterType::Brace))?;

        let mut stmts = vec![];
        while !self.eat(Token::CloseDelimiter(DelimiterType::Brace)) {
            match self.parse_statement() {
                Err(mut err) => {
                    err.emit();
                    // self.recover_stmt_(SemiColonMode::Ignore, BlockMode::Ignore);
                    // self.eat(&token::CloseDelim(token::Brace));
                    // recovered = true;
                    break;
                }
                Ok(stmt) => stmts.push(stmt),
            };

            if self.token == Token::Eof {
                break;
            }
        }

        Ok(stmts)
    }

    fn parse_container_decl(&mut self) -> ParseResult<StatementKind> {
        let is_abstract = self.eat(keyword!(ABSTRACT));
        let ty = self.expect_one_of(vec![
            (keyword!(BLOCK), ContainerType::Block(is_abstract)),
            (keyword!(IN), ContainerType::Extends),
            (keyword!(OPTIONAL), ContainerType::Optional),
        ])?;

        let name = self.parse_ident()?;
        let parents: Vec<Path> = if self.eat(keyword!(INHERITS_FROM)) {
            let mut paths = vec![];

            while let Token::Name(_) = self.token {
                paths.push(self.parse_path()?);

                if !self.eat(Token::Comma) {
                    break;
                }
            }

            paths
        } else {
            vec![]
        };

        let block = self.parse_statement_block()?;

        Ok(StatementKind::ContainerDeclaration(
            ty, name, parents, block,
        ))
    }

    fn parse_expr(&mut self, _ctx: ExprContext) -> ParseResult<ExpressionNode> {
        unimplemented!()
    }

    fn parse_ident(&mut self) -> ParseResult<Ident> {
        let span = self.span;
        let token = self.token.clone();

        match token {
            Token::Name(value) => {
                self.bump();

                Ok(Ident { value, span })
            }
            _ => Err(self.session.diagnostic(Severity::Error, "invalid token")),
        }
    }

    fn parse_macro(&mut self) -> ParseResult<StatementKind> {
        self.bump();

        let _macro_name = self.parse_ident()?;
        unimplemented!()
    }

    fn parse_path(&mut self) -> ParseResult<Path> {
        let start = self.span;
        let mut segments = vec![self.parse_ident()?];

        while self.eat(Token::Dot) {
            segments.push(self.parse_ident()?);
        }

        let end = segments.last().map(|seg| seg.span).unwrap_or(start);

        Ok(Path {
            span: start.to(end),
            segments,
        })
    }

    fn parse_set_modifier(&mut self, _name: Path) -> ParseResult<StatementKind> {
        unimplemented!()
    }

    fn parse_symbol_decl(
        &mut self,
        ty: SymbolType,
        initializer: InitializerKind,
    ) -> ParseResult<StatementKind> {
        // Advance past the symbol type identifier.
        self.bump();

        let name = self.parse_ident()?;
        let expr: Option<ExpressionNode> = match initializer {
            InitializerKind::Invalid => None,
            InitializerKind::Required(..) | InitializerKind::Optional(..) => unimplemented!(),
        };

        Ok(StatementKind::SymbolDeclaration(ty, name, expr))
    }

    /// Advance from the cursor and consume the next `Token`.
    fn bump(&mut self) {
        let (token, span) = self.cursor.advance().consume();

        self.last_span = self.span;
        self.token = token;
        self.span = span;
    }

    /// Check if the token at the current position matches the passed `tok` value, consuming
    /// the token if true.
    fn eat<T: Borrow<Token>>(&mut self, tok: T) -> bool {
        if &self.token == tok.borrow() {
            self.bump();
            true
        } else {
            false
        }
    }

    fn expect<T: Borrow<Token>>(&mut self, expected: T) -> ParseResult<()> {
        if &self.token == expected.borrow() {
            self.bump();
            return Ok(());
        }

        Err(self
            .session
            .diagnostic(Severity::Error, "expected token_name"))
    }

    fn expect_one_of<T: Sized>(&mut self, expected: Vec<(Token, T)>) -> ParseResult<T> {
        let mut token_names = vec![];

        for (tok, value) in expected {
            if self.token == tok {
                self.bump();
                return Ok(value);
            }

            token_names.push("token_name");
        }

        let err = self.session.diagnostic(
            Severity::Error,
            format!("expected one of: {}", token_names.join(", ")),
        );

        Err(err)
    }
}

//use std::cell::RefCell;
///// The parser for CSP is implemented as a recursive descent parser over the simple LL(1) grammar.
//use std::rc::Rc;
//use std::str::FromStr;
//use super::ast::*;
//use super::codemap::Span;
//use super::keywords;
//use super::lex::{DelimiterType, TextRange, Token, TokenAndSpan, Tokenizer};
//use super::lex::Token::*;
//use super::ParseSession;
//
//
//#[derive(Debug)]
//pub struct ParseError {
//    kind: ParseErrorKind,
//    location: Span,
//}
//
//#[derive(Debug)]
//pub enum ParseErrorKind {
//    Msg(&'static str),
//    Eof,
//    InvalidKeyword,
//}
//
//type ParseResult<T> = Result<T, ParseError>;
//
//macro_rules! parser_error {
//    ($span:expr, $ty:expr) => {
//        Err(ParseError {
//            kind: $ty,
//            location: $span,
//        })
//    };
//}
//
//pub struct Parser<'a, 'sess> {
//    module_name: Option<String>,
//    parse_session: Rc<RefCell<ParseSession<'sess>>>,
//    node_id_counter: u64,
//    current: TokenAndSpan<'a>,
//}
//
//impl<'a, 'sess> Parser<'a, 'sess> {
//    pub fn new(module_name: Option<String>, tokenizer: Tokenizer<'a, 'sess>) -> Self {
//        let mut tokenizer = tokenizer;
//        let first_token = tokenizer.next();
//
//        Parser {
//            module_name,
//            tokenizer: tokenizer.peekable(),
//            node_id_counter: 0,
//            current: first_token.expect("No input available"),
//        }
//    }
//
//    /// Generate a new unique identifier for a node in the AST.
//    fn new_node_id(&mut self) -> NodeId {
//        self.node_id_counter += 1;
//        NodeId(self.node_id_counter)
//    }
//
//    fn advance(&mut self) -> ParseResult<()> {
//        if let Some(tok) = self.tokenizer.next() {
//            self.current = tok;
//        }
//
//        Ok(())
//    }
//
//    fn consume(&mut self, tok: Token) -> ParseResult<bool> {
//        if self.current.token == tok {
//            self.advance()?;
//
//            return Ok(true);
//        }
//
//        return Ok(false);
//    }
//
//    fn consume_if<F, T>(&mut self, predicate: F) -> ParseResult<T>
//    where
//        F: Fn(&TokenAndSpan) -> ParseResult<T>,
//    {
//        let result = predicate(&self.current);
//
//        if result.is_ok() {
//            self.advance()?;
//        }
//
//        return result;
//    }
//
//    fn expect(&mut self, tok: Token) -> ParseResult<()> {
//        if self.consume(tok)? {
//            Ok(())
//        } else {
//            parser_error!(self.current.span, ParseErrorKind::InvalidKeyword) // @todo - specific error
//        }
//    }
//
//    fn lookahead(&mut self, tok: Token) -> bool {
//        match self.peek() {
//            Some(t) => t == &tok,
//            _ => false,
//        }
//    }
//
//    fn peek(&mut self) -> Option<&Token<'a>> {
//        match self.tokenizer.peek() {
//            Some(tok) => Some(&tok.token),
//            None => None,
//        }
//    }
//
//    /// Parse the input from the given `Tokenizer`, consuming all tokens until `EOF`
//    /// and returning a `Module` as a result.
//    ///
//    /// Example:
//    /// /// @todo
//    pub fn parse(&mut self) -> ParseResult<Module> {
//        let mut statements = vec![];
//        let start_span = TextRange::default();
//
//        loop {
//            if let Some(TokenAndSpan {
//                token: Token::Eof, ..
//            }) = self.tokenizer.peek()
//            {
//                break;
//            }
//
//            match self.parse_statement() {
//                Ok(stmt) => statements.push(stmt),
//                Err(e) => return Err(e),
//            };
//        }
//
//        let end_span = &self.current.span;
//
//        Ok(Module::new(
//            self.module_name.clone(),
//            statements,
//            start_span.join(end_span),
//        ))
//    }
//
//    fn parse_container_decl(&mut self) -> ParseResult<StatementKind> {
//        let is_abstract = self.consume(Token::Name(keywords::ABSTRACT))?;
//        let ty = self.consume_if(|tok| match tok.token {
//            Token::Name(keywords::BLOCK) => Ok(ContainerType::Block(is_abstract)),
//            Token::Name(keywords::OPTIONAL) => Ok(ContainerType::Optional),
//            Token::Name(keywords::IN) => Ok(ContainerType::Extends),
//            _ => parser_error!(tok.span, ParseErrorKind::InvalidKeyword),
//        })?;
//
//        let name = self.parse_ident()?;
//        self.expect(Token::OpenDelimiter(DelimiterType::Brace))?;
//
//        let mut body: Vec<StatementNode> = vec![];
//
//        while !self.consume(Token::CloseDelimiter(DelimiterType::Brace))? {
//            body.push(self.parse_statement()?);
//        }
//
//        Ok(StatementKind::ContainerDeclaration(ty, name, body))
//    }
//
//    fn parse_macro_call(&mut self, name: Ident) -> ParseResult<StatementKind> {
//        unimplemented!()
//    }
//
//    fn parse_set_modifier(&mut self, name: Ident) -> ParseResult<StatementKind> {
//        unimplemented!()
//    }
//
//    fn parse_statement(&mut self) -> ParseResult<StatementNode> {
//        let start_span = self.current.span;
//
//        let kind = Box::new(match self.current.token {
//            Name(keywords::OPTIONAL) | Name(keywords::ABSTRACT) | Name(keywords::BLOCK) => {
//                self.parse_container_decl()?
//            }
//            Name(kw) if SymbolType::from_str(kw).is_ok() => self.parse_symbol_decl()?,
//            Name(_) => {
//                let name = self.parse_ident()?;
//
//                match self.peek() {
//                    Some(&OpenDelimiter(DelimiterType::Parenthesis)) => {
//                        self.parse_macro_call(name)?
//                    }
//                    Some(&SetModifier) => self.parse_set_modifier(name)?,
//                    _ => return parser_error!(self.current.span, ParseErrorKind::InvalidKeyword),
//                }
//            }
//            _ => return parser_error!(self.current.span, ParseErrorKind::InvalidKeyword),
//        });
//
//        let stmt_span = start_span.join(&self.current.span);
//
//        Ok(StatementNode {
//            kind,
//            node_id: self.new_node_id(),
//            span: stmt_span,
//        })
//    }
//
//    fn parse_symbol_decl(&mut self) -> ParseResult<StatementKind> {
//        let ty = self.consume_if(|tok| match tok.token {
//            Token::Name(val) => Ok(SymbolType::from_str(val).expect("invalid symbol type")),
//            _ => parser_error!(tok.span, ParseErrorKind::InvalidKeyword),
//        })?;
//
//        let name = self.parse_ident()?;
//        let terminated = self.lookahead(Token::Semicolon);
//
//        let initializer = match ty.initializer_kind() {
//            SymbolInitializerKind::Optional | SymbolInitializerKind::Invalid if terminated => {
//                self.advance()?;
//                None
//            }
//
//            SymbolInitializerKind::Required if terminated => {
//                // @todo - report error.
//                self.advance()?;
//                None
//            }
//
//            SymbolInitializerKind::Invalid if !terminated => {
//                //@todo - skip until semicolon and report an error on the spanning input
//                None
//            }
//
//            SymbolInitializerKind::Optional | SymbolInitializerKind::Required if !terminated => {
//                let expr = self.parse_expr()?;
//                self.advance()?; //@todo -expect semi
//
//                Some(expr)
//            }
//            _ => None,
//        };
//
//        Ok(StatementKind::SymbolDeclaration(ty, name, initializer))
//    }
//
//    fn parse_expr(&mut self) -> ParseResult<ExpressionNode> {
//        unimplemented!()
//    }
//
//    fn parse_ident(&mut self) -> ParseResult<Ident> {
//        match self.current.token {
//            Token::Name(val) => {
//                let value = String::from(val);
//                let span = self.current.span;
//
//                self.advance()?;
//
//                Ok(Ident { value, span })
//            }
//            _ => parser_error!(self.current.span, ParseErrorKind::InvalidKeyword),
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    fn parser_with_input(input: &str) -> Parser {
//        Parser::new(None, Tokenizer::new(input))
//    }
//
//    #[test]
//    pub fn parse_container_decl() {
//        let mut parser = parser_with_input("block b {}");
//        let result = parser
//            .parse_statement()
//            .expect("unable to parse container decl");
//
//        assert_eq!(Span::from(0, 9), result.span);
//        assert_eq!(
//            StatementKind::ContainerDeclaration(
//                ContainerType::Block(false),
//                Ident {
//                    value: String::from("b"),
//                    span: Span::at(6),
//                },
//                vec![],
//            ),
//            *result.kind
//        );
//    }
//
//    #[test]
//    pub fn parse_simple_decl() {
//        let mut parser = parser_with_input("type t;");
//        let result = parser.parse_statement().expect("unable to parse decl");
//
//        assert_eq!(Span::from(0, 6), result.span);
//        assert_eq!(
//            StatementKind::SymbolDeclaration(
//                SymbolType::Type,
//                Ident {
//                    value: String::from("t"),
//                    span: Span::at(5),
//                },
//                None,
//            ),
//            *result.kind
//        );
//    }
//}

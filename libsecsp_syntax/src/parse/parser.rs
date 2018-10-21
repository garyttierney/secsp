use codespan::{ByteIndex, ByteSpan};
use codespan_reporting::Severity;
use crate::ast::{
    BinOp, BinOpKind, ContainerType, ExpressionKind, ExpressionNode, Ident, MacroParameter, Module, NodeId, Path,
    StatementKind, StatementNode, SymbolType, UnaryOp, UnaryOpKind,
};
use crate::keywords;
use crate::lex::token::{DelimiterType, Token};
use crate::lex::token_cursor::TokenCursor;
use crate::lex::token_tree::TokenTree;
use crate::parse::expr::ExprContext;
use crate::parse::parser_from_source;
use crate::parse::ParseResult;
use crate::parse::stmt::InitializerKind;
use crate::parse::stmt::StatementType;
use crate::session::ParseSession;
use std::borrow::Borrow;
use std::str::FromStr;

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
                    self.recover_stmt();
                    self.eat(Token::CloseDelimiter(DelimiterType::Brace));
                }
            }
        }

        Ok(Module::new(statements))
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

        let kind = match stmt_ty {
            Some(StatementType::ContainerDeclaration) => self.parse_container_decl()?,
            Some(StatementType::MacroDeclaration) => self.parse_macro()?,
            Some(StatementType::SymbolDeclaration(ty, init)) => self.parse_symbol_decl(ty, init)?,
            None => {
                let path = self.parse_path()?;
                let stmt = match self.token {
                    Token::OpenDelimiter(DelimiterType::Parenthesis) => {
                        self.parse_arg_list(path)?
                    }
                    Token::SetModifier => self.parse_set_modifier(path)?,
                    _ => return Err(self.session.diagnostic(Severity::Error, "unexpected token")),
                };

                stmt
            }
            _ => unimplemented!(),
        };

        if requires_semicolon && !self.eat(Token::Semicolon) {
            self.session
                .diagnostic(Severity::Error, "missing semicolon")
                .span_err(self.span, "expected here")
                .emit();
        }

        Ok(StatementNode {
            node_id: NodeId(0),
            kind: Box::from(kind),
            span: start.to(self.last_span),
        })
    }

    fn recover_stmt(&mut self) {
        let mut brace_depth = 0;

        loop {
            match self.token {
                Token::OpenDelimiter(DelimiterType::Brace) => {
                    self.bump();
                    brace_depth += 1;
                }
                Token::CloseDelimiter(DelimiterType::Brace) => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        return;
                    }

                    self.bump();
                }
                Token::Semicolon => {
                    self.bump();
                    return;
                }
                Token::Eof => {
                    return;
                }

                _ => self.bump()
            }
        }
    }

    fn parse_arg_list(&mut self, name: Path) -> ParseResult<StatementKind> {
        let mut params = vec![];

        self.expect(Token::OpenDelimiter(DelimiterType::Parenthesis))?;
        while !self.eat(Token::CloseDelimiter(DelimiterType::Parenthesis)) {
            params.push(self.parse_expr(None)?);

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
                    self.recover_stmt();
                    self.eat(Token::CloseDelimiter(DelimiterType::Brace));
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
        let parents: Vec<Path> = if self.eat(keyword!(EXTENDS)) {
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

    fn parse_expr(&mut self, ctx: Option<ExprContext>) -> ParseResult<ExpressionNode> {
        let start_span = self.span;
        let ctx = ctx.unwrap_or_default();

        let kind = Box::from(match self.token.clone() {
            Token::Name(_) => {
                let first_id = self.parse_path_to_expr_node()?;

                match self.token {
                    Token::Colon if ctx != ExprContext::NoSecLiteral => {
                        self.parse_security_context_expr_tail(first_id, ctx)?
                    }
                    Token::Hyphen if ctx != ExprContext::NoSecLiteral => {
                        self.parse_level_range_expr_tail(first_id, ctx)?
                    }
                    Token::DotDot => self.parse_category_range_expr_tail(first_id)?,
                    Token::BitwiseAnd
                    | Token::BitwiseOr
                    | Token::BitwiseXor
                    | Token::LogicalAnd
                    | Token::LogicalOr => self.parse_binary_expr_tail(first_id, ctx)?,
                    _ => return Ok(first_id),
                }
            }
            Token::OpenDelimiter(DelimiterType::Parenthesis) => {
                self.bump();
                let expr = self.parse_expr(Some(ctx))?;
                self.expect(Token::CloseDelimiter(DelimiterType::Parenthesis))?;

                return Ok(expr);
            }
            Token::LogicalNot | Token::BitwiseNot => self.parse_unary_expr_tail(ctx)?,
            _ => unreachable!(),
        });

        let span = start_span.to(self.last_span);

        Ok(ExpressionNode {
            node_id: NodeId(0),
            span,
            kind,
        })
    }

    fn parse_path_to_expr_node(&mut self) -> ParseResult<ExpressionNode> {
        let path = self.parse_path()?;

        Ok(ExpressionNode {
            span: path.span,
            kind: Box::from(ExpressionKind::Variable(path)),
            node_id: NodeId(0),
        })
    }

    fn parse_binary_expr_tail(
        &mut self,
        lhs: ExpressionNode,
        ctx: ExprContext,
    ) -> ParseResult<ExpressionKind> {
        let op = self.parse_binary_op()?;
        let rhs = self.parse_expr(Some(ctx))?;

        Ok(ExpressionKind::BinaryOp { lhs, op, rhs })
    }

    fn parse_category_range_expr_tail(
        &mut self,
        first_id: ExpressionNode,
    ) -> ParseResult<ExpressionKind> {
        self.bump();
        let hi = self.parse_path_to_expr_node()?;

        Ok(ExpressionKind::CategoryRange { lo: first_id, hi })
    }

    fn parse_level_range_expr_tail(
        &mut self,
        first_id: ExpressionNode,
        ctx: ExprContext,
    ) -> ParseResult<ExpressionKind> {
        self.expect(Token::Hyphen)?;
        let l1 = self.parse_expr(Some(ExprContext::LevelRange))?;

        Ok(ExpressionKind::LevelRange(first_id, l1))
    }

    fn parse_security_context_expr_tail(
        &mut self,
        first_id: ExpressionNode,
        ctx: ExprContext,
    ) -> ParseResult<ExpressionKind> {
        self.expect(Token::Colon)?;
        let second_id = self.parse_expr(Some(ExprContext::NoSecLiteral))?;

        if self.token == Token::Hyphen {
            // Just parsed a sensitivity:category literal and are at a hyphen,
            // so we must be at the start of a level-range expression.

            let level_node = ExpressionNode {
                span: first_id.span.to(second_id.span),
                kind: Box::from(ExpressionKind::Level(first_id, second_id)),
                node_id: NodeId(0),
            };

            self.parse_level_range_expr_tail(level_node, ExprContext::LevelRange)
        } else if self.eat(Token::Colon) {
            // Not a level-range expression, so must be a context literal
            // expression.

            let ty = self.parse_expr(Some(ExprContext::NoSecLiteral))?;
            let level_range = if self.eat(Token::Colon) {
                Some(self.parse_expr(Some(ExprContext::LevelRange))?)
            } else {
                None
            };

            Ok(ExpressionKind::Context {
                user: first_id,
                role: second_id,
                ty,
                level_range,
            })
        } else {
            // We only have two expressions delimited by colons,
            // so this must be a level expression.
            Ok(ExpressionKind::Level(first_id, second_id))
        }
    }

    fn parse_unary_expr_tail(&mut self, ctx: ExprContext) -> ParseResult<ExpressionKind> {
        let op = self.parse_unary_op()?;
        let expr = self.parse_expr(Some(ctx))?;

        Ok(ExpressionKind::UnaryOp(op, expr))
    }

    fn parse_binary_op(&mut self) -> ParseResult<BinOp> {
        let pos = self.span;
        let op = match self.token {
            Token::LogicalAnd => BinOpKind::LogicalAnd,
            Token::LogicalOr => BinOpKind::LogicalOr,
            Token::BitwiseAnd => BinOpKind::BitwiseAnd,
            Token::BitwiseXor => BinOpKind::BitwiseXor,
            Token::BitwiseOr => BinOpKind::BitwiseOr,
            _ => {
                let err = self
                    .session
                    .diagnostic(Severity::Error, "expected an operator");

                return Err(err);
            }
        };

        self.bump();
        Ok(BinOp::new(op, pos))
    }

    fn parse_unary_op(&mut self) -> ParseResult<UnaryOp> {
        let pos = self.span;
        let op = match self.token {
            Token::LogicalNot => UnaryOpKind::LogicalNot,
            Token::BitwiseNot => UnaryOpKind::BitwiseNot,
            _ => {
                let err = self
                    .session
                    .diagnostic(Severity::Error, "expected unary operator");
                return Err(err);
            }
        };

        self.bump();
        Ok(UnaryOp::new(op, pos))
    }

    /// Consume a single identifier token and return a Symbol
    /// containing its value and location.
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

        let macro_name = self.parse_ident()?;
        let mut macro_parameters = vec![];

        self.expect(Token::OpenDelimiter(DelimiterType::Parenthesis))?;

        while let Token::Name(_) = self.token {
            let start = self.span;
            let qualifier = self.parse_ident().and_then(|v| {
                SymbolType::from_str(&v.value)
                    .map_err(|()| self.session.diagnostic(Severity::Error, "invalid keyword"))
            })?;

            let name = self.parse_ident()?;

            macro_parameters.push(MacroParameter {
                qualifier,
                name,
                span: start.to(self.span),
            });

            if !self.eat(Token::Comma) {
                break;
            }
        }

        self.expect(Token::CloseDelimiter(DelimiterType::Parenthesis))?;

        Ok(StatementKind::MacroDeclaration(
            macro_name,
            macro_parameters,
            self.parse_statement_block()?,
        ))
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
            InitializerKind::Optional(ctx) => {
                if self.eat(Token::Equals) {
                    Some(self.parse_expr(Some(ctx))?)
                } else {
                    None
                }
            }
            InitializerKind::Required(ctx) => {
                self.expect(Token::Equals)?;
                Some(self.parse_expr(Some(ctx))?)
            }
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
            .diagnostic(
                Severity::Error,
                format!("expected {}", expected.borrow().description()),
            ).span_err(self.span, format!("found keyword")))
    }

    fn expect_one_of<T: Sized>(&mut self, expected: Vec<(Token, T)>) -> ParseResult<T> {
        let mut token_names = vec![];

        for (tok, value) in expected {
            if self.token == tok {
                self.bump();
                return Ok(value);
            }

            token_names.push(tok.description());
        }

        let err = self.session.diagnostic(
            Severity::Error,
            format!("expected one of: {}", token_names.join(", ")),
        );

        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::parser_test::{decl, expr, symbol, variable};
    use super::*;

    fn parse_expr<S: Into<String>>(src: S) -> ExpressionNode {
        let sess = ParseSession::default();
        let mut parser = parser_from_source(&sess, src.into()).expect("couldn't initialize parser");

        parser.parse_expr(None).expect("parsing failed")
    }

    fn parse_stmt<S: Into<String>>(src: S) -> StatementNode {
        let sess = ParseSession::default();
        let mut parser = parser_from_source(&sess, src.into()).expect("couldn't initialize parser");

        parser.parse_statement().expect("parsing failed")
    }

    #[test]
    fn parse_context_decl() {
        assert_eq!(
            decl(
                SymbolType::Context,
                "c",
                Some(expr(ExpressionKind::Context {
                    user: variable("user"),
                    role: variable("role"),
                    ty: variable("type"),
                    level_range: None,
                })),
            ),
            parse_stmt("context c = user:role:type;")
        );
    }

    #[test]
    fn parse_context_expr() {
        assert_eq!(
            expr(ExpressionKind::Context {
                user: variable("user"),
                role: variable("role"),
                ty: variable("type"),
                level_range: None,
            }),
            parse_expr("user:role:type")
        );
    }

    #[test]
    fn parse_mls_context_expr() {
        assert_eq!(
            expr(ExpressionKind::Context {
                user: variable("user"),
                role: variable("role"),
                ty: variable("type"),
                level_range: Some(variable("levelrange")),
            }),
            parse_expr("user:role:type:levelrange")
        );
    }

    #[test]
    fn parse_mls_context_inline_expr() {
        assert_eq!(
            expr(ExpressionKind::Context {
                user: variable("user"),
                role: variable("role"),
                ty: variable("type"),
                level_range: Some(expr(ExpressionKind::LevelRange(
                    variable("l0"),
                    variable("l1"),
                ))),
            }),
            parse_expr("user:role:type:l0-l1")
        );
    }

    #[test]
    fn parse_mcs_context_inline_expr() {
        assert_eq!(
            expr(ExpressionKind::Context {
                user: variable("user"),
                role: variable("role"),
                ty: variable("type"),
                level_range: Some(expr(ExpressionKind::LevelRange(
                    expr(ExpressionKind::Level(variable("s0"), variable("c0"))),
                    variable("l1"),
                ))),
            }),
            parse_expr("user:role:type:s0:c0-l1")
        );
    }

    #[test]
    fn parse_unary_expr() {
        assert_eq!(
            expr(ExpressionKind::UnaryOp(
                symbol(UnaryOpKind::LogicalNot),
                variable("a"),
            )),
            parse_expr("!a")
        );
    }

    #[test]
    fn parse_bin_expr() {
        assert_eq!(
            expr(ExpressionKind::BinaryOp {
                lhs: variable("a"),
                op: symbol(BinOpKind::LogicalAnd),
                rhs: variable("b"),
            }),
            parse_expr("a && b")
        );
    }
}

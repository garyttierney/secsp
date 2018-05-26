use super::ast::*;
use super::codemap::Span;
use super::keywords;
use super::lex::Token::*;
use super::lex::{DelimiterType, Token, TokenAndSpan, Tokenizer};
/// The parser for CSP is implemented as a recursive descent parser over the simple LL(1) grammar.
use std::iter;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    location: Span,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    Msg(&'static str),
    Eof,
    InvalidKeyword,
}

type ParseResult<T> = Result<T, ParseError>;

macro_rules! parser_error {
    ($span:expr, $ty:expr) => {
        Err(ParseError {
            kind: $ty,
            location: $span,
        })
    };
}

pub struct Parser<'a> {
    module_name: Option<String>,
    tokenizer: iter::Peekable<Tokenizer<'a>>,
    node_id_counter: u64,
    current: TokenAndSpan<'a>,
    at_eof: bool,
}

impl<'a> Parser<'a> {
    pub fn new(module_name: Option<String>, tokenizer: Tokenizer<'a>) -> Self {
        let mut tokenizer = tokenizer;
        let first_token = tokenizer.next();

        Parser {
            module_name,
            tokenizer: tokenizer.peekable(),
            node_id_counter: 0,
            current: first_token.expect("No input available"),
            at_eof: false,
        }
    }

    /// Generate a new unique identifier for a node in the AST.
    fn new_node_id(&mut self) -> NodeId {
        self.node_id_counter += 1;
        NodeId(self.node_id_counter)
    }

    fn advance(&mut self) -> ParseResult<()> {
        if self.at_eof {
            return parser_error!(self.current.span, ParseErrorKind::Eof);
        }

        if let Some(tok) = self.tokenizer.next() {
            self.current = tok;
        } else {
            self.at_eof = true;
        }

        Ok(())
    }

    fn consume(&mut self, tok: Token) -> ParseResult<bool> {
        if self.current.token == tok {
            self.advance()?;

            return Ok(true);
        }

        return Ok(false);
    }

    fn consume_if<F, T>(&mut self, predicate: F) -> ParseResult<T>
    where
        F: Fn(&TokenAndSpan) -> ParseResult<T>,
    {
        let result = predicate(&self.current);

        if result.is_ok() {
            self.advance()?;
        }

        return result;
    }

    fn expect(&mut self, tok: Token) -> ParseResult<()> {
        if self.consume(tok)? {
            Ok(())
        } else {
            parser_error!(self.current.span, ParseErrorKind::InvalidKeyword) // @todo - specific error
        }
    }

    fn lookahead(&mut self, tok: Token) -> bool {
        match self.peek() {
            Some(t) => t == &tok,
            _ => false,
        }
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        match self.tokenizer.peek() {
            Some(tok) => Some(&tok.token),
            None => None,
        }
    }

    /// Parse the input from the given `Tokenizer`, consuming all tokens until `EOF`
    /// and returning a `Module` as a result.
    ///
    /// Example:
    /// /// @todo
    pub fn parse(&mut self) -> ParseResult<Module> {
        let mut statements = vec![];
        let start_span = Span::at(0);

        loop {
            if self.tokenizer.peek().is_none() {
                break;
            }

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e),
            };
        }

        let end_span = &self.current.span;

        Ok(Module::new(
            self.module_name.clone(),
            statements,
            start_span.join(end_span),
        ))
    }

    fn parse_container_decl(&mut self) -> ParseResult<StatementKind> {
        let is_abstract = self.consume(Token::Name(keywords::ABSTRACT))?;
        let ty = self.consume_if(|tok| match tok.token {
            Token::Name(keywords::BLOCK) => Ok(ContainerType::Block(is_abstract)),
            Token::Name(keywords::OPTIONAL) => Ok(ContainerType::Optional),
            Token::Name(keywords::IN) => Ok(ContainerType::Extends),
            _ => parser_error!(tok.span, ParseErrorKind::InvalidKeyword),
        })?;

        let name = self.parse_ident()?;
        self.expect(Token::OpenDelimiter(DelimiterType::Brace))?;

        let mut body: Vec<StatementNode> = vec![];

        while !self.consume(Token::CloseDelimiter(DelimiterType::Brace))? {
            body.push(self.parse_statement()?);
        }

        Ok(StatementKind::ContainerDeclaration(ty, name, body))
    }

    fn parse_macro_call(&mut self, name: Ident) -> ParseResult<StatementKind> {
        unimplemented!()
    }

    fn parse_set_modifier(&mut self, name: Ident) -> ParseResult<StatementKind> {
        unimplemented!()
    }

    fn parse_statement(&mut self) -> ParseResult<StatementNode> {
        let start_span = self.current.span;

        let kind = Box::new(match self.current.token {
            Name(keywords::OPTIONAL) | Name(keywords::ABSTRACT) | Name(keywords::BLOCK) => {
                self.parse_container_decl()?
            }
            Name(kw) if SymbolType::from_str(kw).is_ok() => self.parse_symbol_decl()?,
            Name(_) => {
                let name = self.parse_ident()?;

                match self.peek() {
                    Some(&OpenDelimiter(DelimiterType::Parenthesis)) => {
                        self.parse_macro_call(name)?
                    }
                    Some(&SetModifier) => self.parse_set_modifier(name)?,
                    _ => return parser_error!(self.current.span, ParseErrorKind::InvalidKeyword),
                }
            }
            _ => return parser_error!(self.current.span, ParseErrorKind::InvalidKeyword),
        });

        let stmt_span = start_span.join(&self.current.span);

        Ok(StatementNode {
            kind,
            node_id: self.new_node_id(),
            span: stmt_span,
        })
    }

    fn parse_symbol_decl(&mut self) -> ParseResult<StatementKind> {
        let ty = self.consume_if(|tok| match tok.token {
            Token::Name(val) => Ok(SymbolType::from_str(val).expect("invalid symbol type")),
            _ => parser_error!(tok.span, ParseErrorKind::InvalidKeyword),
        })?;

        let name = self.parse_ident()?;
        let terminated = self.lookahead(Token::Semicolon);

        let initializer = match ty.initializer_kind() {
            SymbolInitializerKind::Optional | SymbolInitializerKind::Invalid if terminated => {
                self.advance()?;
                None
            }

            SymbolInitializerKind::Required if terminated => {
                // @todo - report error.
                self.advance()?;
                None
            }

            SymbolInitializerKind::Invalid if !terminated => {
                //@todo - skip until semicolon and report an error on the spanning input
                None
            }

            SymbolInitializerKind::Optional | SymbolInitializerKind::Required if !terminated => {
                let expr = self.parse_expr()?;
                self.advance()?; //@todo -expect semi

                Some(expr)
            }
            _ => None,
        };

        Ok(StatementKind::SymbolDeclaration(ty, name, initializer))
    }

    fn parse_expr(&mut self) -> ParseResult<ExpressionNode> {
        unimplemented!()
    }

    fn parse_ident(&mut self) -> ParseResult<Ident> {
        match self.current.token {
            Token::Name(val) => {
                let value = String::from(val);
                let span = self.current.span;

                self.advance()?;

                Ok(Ident { value, span })
            }
            _ => parser_error!(self.current.span, ParseErrorKind::InvalidKeyword),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser_with_input(input: &str) -> Parser {
        Parser::new(None, Tokenizer::new(input))
    }

    #[test]
    pub fn parse_container_decl() {
        let mut parser = parser_with_input("block b {}");
        let result = parser
            .parse_statement()
            .expect("unable to parse container decl");

        assert_eq!(Span::from(0, 9), result.span);
        assert_eq!(
            StatementKind::ContainerDeclaration(
                ContainerType::Block(false),
                Ident {
                    value: String::from("b"),
                    span: Span::at(6),
                },
                vec![],
            ),
            *result.kind
        );
    }

    #[test]
    pub fn parse_simple_decl() {
        let mut parser = parser_with_input("type t;");
        let result = parser.parse_statement().expect("unable to parse decl");

        assert_eq!(Span::from(0, 6), result.span);
        assert_eq!(
            StatementKind::SymbolDeclaration(
                SymbolType::Type,
                Ident {
                    value: String::from("t"),
                    span: Span::at(5),
                },
                None,
            ),
            *result.kind
        );
    }
}

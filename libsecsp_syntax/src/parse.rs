use super::ast;
use super::codemap::Span;
use super::keywords;
use super::lex::{DelimiterType, Token, TokenAndSpan, Tokenizer};
use super::lex::Token::*;

use std::iter;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    Msg(&'static str),
    Eof,
    InvalidKeyword,
}

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a> {
    module_name: Option<String>,
    tokenizer: iter::Peekable<Tokenizer<'a>>,
    node_id_counter: u64,
    current: TokenAndSpan<'a>,
}

impl<'a> Parser<'a> {
    fn new(module_name: Option<String>, tokenizer: Tokenizer<'a>) -> Self {
        let mut tokenizer = tokenizer;
        let first_token = tokenizer.next().expect("no input available");

        Parser {
            module_name,
            tokenizer: tokenizer.peekable(),
            node_id_counter: 0,
            current: first_token,
        }
    }

    /// Generate a new unique identifier for a node in the AST.
    fn new_node_id(&mut self) -> ast::NodeId {
        self.node_id_counter += 1;
        ast::NodeId(self.node_id_counter)
    }

    fn advance(&mut self) -> ParseResult<&TokenAndSpan<'a>> {
        match self.tokenizer.next() {
            Some(token) => {
                self.current = token;
                Ok(&self.current)
            }
            None => Err(ParseError::Eof),
        }
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        match self.tokenizer.peek() {
            Some(tok) => Some(&tok.token),
            None => None,
        }
    }

    fn lookahead(&mut self, tok: Token) -> bool {
        match self.peek() {
            Some(t) => t == &tok,
            _ => false,
        }
    }

    /// Parse the input from the given `Tokenizer`, consuming all tokens until `EOF`
    /// and returning a `Module` as a result.
    ///
    /// Example:
    /// /// @todo
    fn parse(&mut self) -> ParseResult<ast::Module> {
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

        Ok(ast::Module::new(
            self.module_name.clone(),
            statements,
            start_span.join(end_span),
        ))
    }

    fn parse_container_decl(&mut self) -> ParseResult<ast::StatementKind> {
        unimplemented!()
    }

    fn parse_macro_call(&mut self, name: ast::Ident) -> ParseResult<ast::StatementKind> {
        unimplemented!()
    }

    fn parse_set_modifier(&mut self, name: ast::Ident) -> ParseResult<ast::StatementKind> {
        unimplemented!()
    }

    fn parse_statement(&mut self) -> ParseResult<ast::StatementNode> {
        self.advance()
            .expect("peek() returned a valid token, next() returned None");

        let start_span = self.current.span;

        let kind = Box::new(match self.current.token {
            Name(keywords::OPTIONAL) | Name(keywords::ABSTRACT) | Name(keywords::BLOCK) => {
                self.parse_container_decl()?
            }
            Name(kw) if ast::SymbolType::from_str(kw).is_ok() => self.parse_symbol_decl()?,
            Name(ident) => {
                let name = self.parse_ident()?;

                match self.peek() {
                    Some(&OpenDelimiter(DelimiterType::Parenthesis)) => {
                        self.parse_macro_call(name)?
                    }
                    Some(&SetModifier) => self.parse_set_modifier(name)?,
                    _ => return Err(ParseError::InvalidKeyword),
                }
            }
            _ => return Err(ParseError::InvalidKeyword),
        });

        let stmt_span = start_span.join(&self.current.span);

        Ok(ast::StatementNode {
            kind,
            node_id: self.new_node_id(),
            span: stmt_span,
        })
    }

    fn parse_symbol_decl(&mut self) -> ParseResult<ast::StatementKind> {
        let ty = if let Name(ty) = self.current.token {
            ast::SymbolType::from_str(ty).map_err(|_| ParseError::InvalidKeyword)?
        } else {
            return Err(ParseError::InvalidKeyword);
        };

        let name = self.parse_ident()?;
        let terminated = self.lookahead(Token::Semicolon);

        let initializer_kind = ty.initializer_kind();
        let initializer = match initializer_kind {
            ast::SymbolInitializerKind::Required => if terminated {
                None
            } else {
                Some(self.parse_expr()?)
            },
            ast::SymbolInitializerKind::Optional if !terminated => Some(self.parse_expr()?),
            ast::SymbolInitializerKind::Invalid if !terminated => None, // @todo - skip until end and log error
            _ => None,
        };

        Ok(ast::StatementKind::SymbolDeclaration(ty, name, initializer))
    }

    fn parse_expr(&mut self) -> ParseResult<ast::ExpressionNode> {
        unimplemented!()
    }

    fn parse_ident(&mut self) -> ParseResult<ast::Ident> {
        self.advance().expect("eof");

        match self.current.token {
            Token::Name(val) => Ok(ast::Ident {
                value: val.to_owned(),
                span: self.current.span,
            }),
            _ => Err(ParseError::InvalidKeyword),
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
    pub fn parse_simple_decl() {
        let mut parser = parser_with_input("type t;");
        let result = parser.parse_symbol_decl().expect("unable to parse decl");

        assert_eq!(
            ast::StatementKind::SymbolDeclaration(
                ast::SymbolType::Type,
                ast::Ident {
                    value: "t".to_owned(),
                    span: Span::at(5),
                },
                None,
            ),
            result
        );
    }
}

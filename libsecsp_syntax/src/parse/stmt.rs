use crate::ast::SymbolType;
use crate::keywords::*;
use crate::parse::expr::ExprContext;

pub enum InitializerKind {
    Invalid,
    Required(ExprContext),
    Optional(ExprContext),
}

pub enum StatementType {
    Call,
    Conditional,
    ContainerDeclaration,
    MacroDeclaration,
    SymbolDeclaration(SymbolType, InitializerKind),
}

impl StatementType {
    pub fn from_keyword(kw: &str) -> Option<StatementType> {
        use self::InitializerKind::*;
        use self::StatementType::*;
        use self::SymbolType::*;

        let ty = match kw {
            TYPE_ATTRIBUTE => SymbolDeclaration(TypeAttribute, Optional(ExprContext::Type)),
            TYPE => SymbolDeclaration(Type, Invalid),
            ROLE => SymbolDeclaration(Role, Invalid),
            CONTEXT => SymbolDeclaration(Context, Required(ExprContext::Context)),
            ABSTRACT | BLOCK | OPTIONAL | IN => ContainerDeclaration,
            MACRO => MacroDeclaration,
            _ => return None,
        };

        Some(ty)
    }

    pub fn is_block(&self) -> bool {
        match *self {
            StatementType::ContainerDeclaration
            | StatementType::MacroDeclaration
            | StatementType::Conditional => true,
            _ => false,
        }
    }

    pub fn is_declaration(&self) -> bool {
        match *self {
            StatementType::ContainerDeclaration
            | StatementType::MacroDeclaration
            | StatementType::SymbolDeclaration(..) => true,
            _ => false,
        }
    }
}

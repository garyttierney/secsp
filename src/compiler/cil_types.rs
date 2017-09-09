extern crate symbolic_expressions;

use secsp::ast::*;
use self::symbolic_expressions::Sexp;

macro_rules! cil {
    ($x:expr) => {
        $x.into_sexp()
    }
}

macro_rules! cil_list {
    [ $( $x:expr ),* ] => {
        {
            let mut temp_vec : Vec<Sexp> = Vec::new();
            $(
                temp_vec.push($x.into());
            )*

            temp_vec.into()
        }
    };
}

pub trait ToCil: Sized + Clone {
    fn into_sexp(&self) -> Sexp;
}

impl ToCil for Statement {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Statement::Declaration(ref decl) => decl.into_sexp(),
            Statement::MacroCall(ref id, ref params) => {
                let params_sexpr: Vec<Sexp> =
                    params.iter().map(|ref p: &Expr| p.into_sexp()).collect();

                cil_list!["call", id, params_sexpr]
            }
            _ => Sexp::Empty,
        }
    }
}

impl ToCil for BlockType {
    fn into_sexp(&self) -> Sexp {
        self.to_cil().into()
    }
}

impl ToCil for SymbolType {
    fn into_sexp(&self) -> Sexp {
        self.to_cil().into()
    }
}

impl ToCil for Declaration {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Declaration::Block {
                ref is_abstract,
                ref qualifier,
                ref name,
                ref statements,
            } => {
                let mut body: Vec<Sexp> = statements.iter().map(|it| it.into_sexp()).collect();

                if *is_abstract {
                    body.insert(0, cil_list!["blockabstract", name]);
                }

                cil_list![qualifier.into_sexp(), name, body]
            }
            Declaration::Symbol {
                ref qualifier,
                ref name,
                ref initializer,
            } => cil_list![qualifier.into_sexp(), name],
            _ => Sexp::Empty,
        }
    }
}

impl ToCil for Expr {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Expr::Binary(ref lhs, ref op, ref rhs) => {
                cil_list![cil!(lhs.as_ref()), cil!(op), cil!(rhs.as_ref())]
            }
            Expr::Unary(ref op, ref expr) => cil_list![cil!(op), cil!(expr.as_ref())],
            Expr::Variable(ref id) => id.into(),
            _ => Sexp::Empty, //@TODO
        }
    }
}

impl ToCil for BinaryOp {
    fn into_sexp(&self) -> Sexp {
        let result = match *self {
            BinaryOp::BitwiseAnd | BinaryOp::ConditionalAnd => "and",
            BinaryOp::BitwiseOr | BinaryOp::ConditionalOr => "or",
            BinaryOp::BitwiseXor | BinaryOp::ConditionalXor => "xor",
        };

        result.into()
    }
}

impl ToCil for UnaryOp {
    fn into_sexp(&self) -> Sexp {
        let result = match *self {
            UnaryOp::ConditionalNot | UnaryOp::BitwiseNot => "not",
        };

        result.into()
    }
}

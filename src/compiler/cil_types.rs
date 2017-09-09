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

pub trait CilType: TypeSpecifier {
    fn requires_initializer(&self) -> bool;
}

impl CilType for SymbolType {
    fn requires_initializer(&self) -> bool {
        match *self {
            SymbolType::Context => true,
            _ => false,
        }
    }
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
            Statement::IfElse {
                ref condition,
                ref then_block,
                ref else_ifs,
                ref else_block,
            } => {
                let body_sexpr: Vec<Sexp> = then_block.iter().map(|s| s.into_sexp()).collect();
                let mut statement_sexpr: Sexp = cil_list!["booleanif", condition.into_sexp()];
                let mut branches: Sexp = cil_list!["true", body_sexpr];

                if let Some(ref else_body) = *else_block {
                    let else_block_sexpr: Vec<Sexp> =
                        else_body.iter().map(|s| s.into_sexp()).collect();
                    let else_branch: Sexp = cil_list!["false", else_block_sexpr];

                    branches.push(else_branch);
                }

                statement_sexpr.push(branches);
                statement_sexpr
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

impl ToCil for MacroParameter {
    fn into_sexp(&self) -> Sexp {
        cil_list![self.qualifier.into_sexp(), self.name.clone()]
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
            } => {
                let mut declaration: Sexp = cil_list![qualifier.into_sexp(), name];

                if qualifier.requires_initializer() {
                    if let Some(ref expr) = *initializer {
                        declaration.push(expr.into_sexp());
                    } else {
                        //@todo - raise error
                    }
                }

                declaration
            }
            Declaration::Macro {
                ref name,
                ref parameters,
                ref statements,
            } => {
                let body: Vec<Sexp> = statements.iter().map(|it| it.into_sexp()).collect();
                let params: Vec<Sexp> = parameters.iter().map(|it| it.into_sexp()).collect();

                cil_list!["macro", name, params, body]
            }

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
            Expr::LevelRange(ref low, ref high) => cil_list![cil!(low), cil!(high)],
            Expr::Level {
                ref sensitivity,
                ref categories,
            } => {
                let categories_sexpr: Vec<Sexp> = cil_list![categories.into_sexp()];

                cil_list![sensitivity, categories_sexpr]
            }
            Expr::Context {
                ref user_id,
                ref role_id,
                ref type_id,
                ref level_range,
            } => {
                let mut context_sexp: Sexp = cil_list![user_id, type_id, role_id];

                if let Some(ref expr) = *level_range {
                    context_sexp.push(expr.as_ref().into_sexp());
                }

                context_sexp
            }
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

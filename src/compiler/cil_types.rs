extern crate symbolic_expressions;

use secsp::ast::*;
use self::symbolic_expressions::Sexp;

macro_rules! cil {
    ($x:expr) => {
        $x.into_sexp()
    }
}

macro_rules! cil_list {
    [ $( $x:expr ),* $(,)* ] => {
        {
            let mut temp_vec : Vec<Sexp> = Vec::new();
            $(
                temp_vec.push($x.into());
            )*

            let sexp : Sexp = temp_vec.into();
            sexp
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

fn compile_if(condition: &Expr, block: &Vec<Statement>) -> Sexp {
    let mut statement_sexpr: Sexp = cil_list!["booleanif", condition.into_sexp()];
    let mut true_branch: Sexp = cil_list!["true"];

    for stmt in block {
        true_branch.push(stmt.into_sexp());
    }

    statement_sexpr.push(true_branch);
    statement_sexpr
}

fn compile_if_else_if(
    condition: &Expr,
    block: &Vec<Statement>,
    else_ifs: &Vec<(Expr, Vec<Statement>)>,
    else_block: Option<&Vec<Statement>>,
) -> Sexp {
    let mut if_stmt = compile_if(condition, block);
    let mut false_branch = cil_list!["false"];

    let mut else_if_iter = else_ifs.into_iter().rev();
    let mut last = match else_if_iter.next() {
        Some(else_if) => {
            let (ref expr, ref statements) = *else_if;
            compile_if_else(expr, statements, else_block)
        }
        _ => panic!("compile_if_else_if called with no else_ifs"),
    };

    for else_if in else_if_iter {
        let (ref condition, ref statements) = *else_if;

        let mut stmt = compile_if(condition, statements);
        let else_branch = cil_list!["false", last];
        stmt.push(else_branch);

        last = stmt;
    }

    false_branch.push(last);
    if_stmt.push(false_branch);

    if_stmt
}

fn compile_if_else(
    condition: &Expr,
    block: &Vec<Statement>,
    else_block: Option<&Vec<Statement>>,
) -> Sexp {
    let mut if_stmt = compile_if(condition, block);

    if let Some(statements) = else_block {
        let mut false_branch = cil_list!["false"];

        for stmt in statements {
            false_branch.push(stmt.into_sexp());
        }

        if_stmt.push(false_branch);
    }

    if_stmt
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
                if else_ifs.is_empty() {
                    compile_if_else(condition, then_block, else_block.as_ref())
                } else {
                    compile_if_else_if(condition, then_block, else_ifs, else_block.as_ref())
                }
            }
            Statement::AccessVectorRule {
                ref rule_type,
                ref source,
                ref target,
                ref access_vector,
            } => {
                let mut rule_sexp =
                    cil_list![rule_type.to_cil(), source.into_sexp(), target.into_sexp()];

                match *access_vector {
                    AccessVector::Permission(ref expr) => {
                        rule_sexp.push(expr.into_sexp());
                    }
                    AccessVector::ClassAndPermissions(ref class, ref expr) => {
                        rule_sexp.push(cil_list![class.into_sexp(), expr.into_sexp()]);
                    }
                }

                rule_sexp
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
                ref extends,
            } => {
                let mut statement: Sexp = cil_list![qualifier.into_sexp(), name];
                if *is_abstract {
                    let blockabstract_sexpr: Sexp = cil_list!["blockabstract", name];
                    statement.push(blockabstract_sexpr);
                }

                if let Some(ref extends_list) = *extends {
                    for id in extends_list {
                        statement.push(cil_list!["blockinherit", id]);
                    }
                }

                for stmt in statements {
                    statement.push(stmt.into_sexp());
                }

                statement
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
                let params: Vec<Sexp> = parameters.iter().map(|it| it.into_sexp()).collect();
                let mut decl: Sexp = cil_list!["macro", name, params];

                for stmt in statements {
                    decl.push(stmt.into_sexp());
                }

                decl
            }

            _ => Sexp::Empty,
        }
    }
}



impl ToCil for Expr {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Expr::Binary(ref lhs, ref op, ref rhs) => {
                cil_list![cil!(op), cil!(lhs.as_ref()), cil!(rhs.as_ref())]
            }
            Expr::Unary(ref op, ref expr) => cil_list![cil!(op), cil!(expr.as_ref())],
            Expr::Variable(ref id) => id.into(),
            Expr::VariableList(ref list) => {
                let mut list_sexpr = cil_list![];

                for identifier in list {
                    list_sexpr.push(identifier);
                }

                list_sexpr
            }
            Expr::LevelRange(ref low, ref high) => cil_list![cil!(low), cil!(high)],
            Expr::Level {
                ref sensitivity,
                ref categories,
            } => {
                let categories_sexpr: Sexp = cil_list![categories.into_sexp()];

                cil_list![sensitivity, categories_sexpr]
            }
            Expr::Context {
                ref user_id,
                ref role_id,
                ref type_id,
                ref level_range,
            } => {
                let mut context_sexp: Sexp = cil_list![user_id, role_id, type_id];

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

#[cfg(test)]
mod testing {

    use super::*;
    use secsp;

    pub fn parse_and_compile_stmt(input: &str) -> Sexp {
        let stmt = match secsp::parse_from_slice(input.as_bytes()) {
            secsp::ParseResult::Ok(stmts) => stmts[0].clone(),
            _ => panic!("Unable to parse"),
        };

        stmt.into_sexp()
    }

    #[test]
    pub fn compile_block_decl() {
        let expected: Sexp = cil_list!["block", "my_block"];
        let actual = parse_and_compile_stmt("block my_block {}");

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_abstract_block_decl() {
        let expected: Sexp = cil_list!["block", "my_block", cil_list!["blockabstract", "my_block"]];
        let actual = parse_and_compile_stmt("abstract block my_block {}");

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_block_decl_with_extends() {
        let expected =
            cil_list![
            "block",
            "my_block",
            cil_list!["blockinherit", "my_other_block"],
            cil_list!["blockinherit", "my_other_other_block"],
        ];
        let actual = parse_and_compile_stmt(
            "block my_block extends my_other_block, my_other_other_block {}",
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_if_else() {
        let expected =
            cil_list![
            "booleanif",
            "my_bool",
            cil_list!["true"],
            cil_list!["false"],
        ];

        let actual = parse_and_compile_stmt(
            "
           if (my_bool) {

           } else {

           }",
        );

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_macro_call() {
        let expected = cil_list!["call", "my_macro", cil_list!["a"]];
        let actual = parse_and_compile_stmt("my_macro(a);");

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_context_decl() {
        let expected = cil_list!["context", "my_context", cil_list!["user", "role", "type"]];
        let actual = parse_and_compile_stmt("context my_context = user:role:type;");

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_access_vector_rule() {
        let expected = cil_list!["allow", "src", "target", "permission_set"];
        let actual = parse_and_compile_stmt("allow src target : permission_set;");

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn compile_access_vector_rule_anonymous_perms() {
        let expected =
            cil_list![
            "dontaudit",
            "src",
            "target",
            cil_list!["security_class", cil_list!["or", cil_list!["perm1", "perm2"], cil_list!["perm3", "perm4"]]],
        ];

        let actual = parse_and_compile_stmt(
            "dontaudit src target : security_class ((perm1 perm2) | (perm3 perm4));",
        );

        assert_eq!(expected, actual);
    }
}
use secsp_syntax_derive::AstEnum;

mod labeling;
mod macro_call;
pub mod te_rule;

#[derive(AstEnum)]
pub enum Statement {
    #[ast(kind = "NODE_MACRO_CALL")]
    MacroCall(macro_call::MacroCall),

    #[ast(kind = "NODE_TE_RULE")]
    TeRule(te_rule::TeRule),
}

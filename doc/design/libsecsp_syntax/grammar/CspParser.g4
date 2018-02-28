/*
 * The CSP grammar is an easy to parse C-style LL(1) language.
 */
parser grammar CspParser;

options {
	tokenVocab = CspLexer;
}

module: statement*;
statement: statement_kind;
statement_block: OPEN_BRACE statement* CLOSE_BRACE;

statement_kind:
	symbol_declaration
	| container_declaration
	| list_declaration
	| macro_declaration
	| macro_call
	| conditional;

expression:
	OPEN_PAREN expression CLOSE_PAREN						                     # parenthesized_expr
	| user = IDENT COLON role = IDENT COLON type = IDENT (COLON expression)?	 # context_expr
	| s0 = IDENT (COLON c0=expression)? HYPHEN s1 = IDENT (COLON c1=expression)? # level_range_expr
	| lo = expression DOTDOT hi = expression                                     # range_expr
	| op = unary_op expression								                     # unary_expr
	| left = expression logical_op right = expression		                     # binary_expr
	| left = expression bitwise_op right = expression		                     # binary_expr
	| IDENT													                     # reference_expr;

unary_op: BITWISE_NOT | LOGICAL_NOT;
logical_op: LOGICAL_AND | LOGICAL_OR;
bitwise_op: BITWISE_AND | BITWISE_OR | BITWISE_XOR;

/**
 * A declaration of a single named symbol with an optional initializer expression.
 *
 * ```
 * type t;
 * type t1;
 * type_alias ta = t;
 * type_attribute ta1;
 * type_attribute ta2 = [t, t1];
 * ```
 */
symbol_declaration:
	SYMBOL_TY IDENT (EQUALS expression)? SEMICOLON;

// tag:container[]
/**
 * A declaration of a container, optionally abstract with zero or more parents.
 *
 * ```
 * abstract block abc {}
 * block abc1 extends abc {}
 * ```
 */
container_declaration:
	ABSTRACT? CONTAINER_TY IDENT container_extends_list? statement_block;

/**
 * A comma-separated list of abstract blocks that a container extends from, prefixed by the
 * `inherits_from` keyword.
 */
container_extends_list: INHERITS_FROM (IDENT (COMMA IDENT)*);
// end::container[]
/**
 * A declaration of a named list, such as those used to declare a security class.
 *
 * ``` class my_class { perm1, perm2 } ```
 */
list_declaration:
	LIST_TY IDENT OPEN_BRACE (list_declaration_items COMMA?)? CLOSE_BRACE;

list_declaration_items: IDENT (COMMA IDENT)*;

// tag::macro_declaration[]
/**
 * A declaration of a macro, with an optional parameter list.
 * ``` macro my_macro(type t) { ... } ```
 */
macro_declaration:
	MACRO IDENT macro_parameter_declaration_list statement_block;

macro_parameter_declaration_list:
	OPEN_PAREN macro_param_decl_list_items CLOSE_PAREN;

macro_param_decl_list_items:
	macro_param_decl (COMMA macro_param_decl)*;

macro_param_decl: SYMBOL_TY IDENT;
// end::macro_declaration[]

conditional:
	(IF expression statement_block) conditional_else_branches;

conditional_else_branches:
	(ELSEIF expression statement_block)* (ELSE statement_block)?;

macro_call: IDENT macro_call_argument_list SEMICOLON;

macro_call_argument_list:
	OPEN_PAREN expression (COMMA expression)* CLOSE_PAREN;

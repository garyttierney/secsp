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
	OPEN_PAREN expression CLOSE_PAREN						# parenthesized_expr
	| user = IDENT COLON role = IDENT COLON type = IDENT	# context_expr /* @todo - MLS range */
	| op = unary_op expression								# unary_expr
	| left = expression logical_op right = expression		# binary_expr
	| left = expression bitwise_op right = expression		# binary_expr
	| IDENT													# reference_expr;

unary_op: BITWISE_NOT | LOGICAL_NOT;
logical_op: LOGICAL_AND | LOGICAL_OR;
bitwise_op: BITWISE_AND | BITWISE_OR | BITWISE_XOR;

symbol_declaration:
	SYMBOL_TY IDENT (EQUALS expression)? SEMICOLON;

container_declaration:
	ABSTRACT? CONTAINER_TY IDENT container_extends_list? statement_block;

container_extends_list:
    INHERITS_FROM (IDENT (COMMA IDENT)*);

list_declaration:
    LIST_TY IDENT OPEN_BRACE (list_declaration_items COMMA?)? CLOSE_BRACE;

list_declaration_items: IDENT (COMMA IDENT)*;

macro_declaration:
	MACRO IDENT macro_parameter_declaration_list statement_block;

macro_parameter_declaration_list:
	OPEN_PAREN
	    macro_parameter_declaration (COMMA macro_parameter_declaration)*
    CLOSE_PAREN;

macro_parameter_declaration: SYMBOL_TY IDENT;

conditional:
	(IF expression
	    statement_block
    )
    (ELSEIF expression
        statement_block
    )*
    (ELSE
        statement_block
    )?;

macro_call:
    IDENT macro_call_argument_list SEMICOLON;

macro_call_argument_list:
    OPEN_PAREN
        expression (COMMA expression)*
    CLOSE_PAREN;

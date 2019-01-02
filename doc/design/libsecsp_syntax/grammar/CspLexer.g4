lexer grammar CspLexer;

// tag::keywords[]
/*
 * Statement and language keywords
 */
ABSTRACT: 'abstract';
INHERITS_FROM: 'inherits_from';
IF: 'if';
ELSEIF: 'elseif';
ELSE: 'else';
MACRO: 'macro';
// end::keywords[]

// tag::container_types[]
/**
 * Built in container types.
 */
CONTAINER_TY: 'block' | 'optional' | 'in';
// end::container_types[]

// tag::list_types[]
/**
 * Built in list types.
 */
LIST_TY: 'class' | 'common';
// end::list_types[]

// tag::symbol_types[]
/**
 * Built in symbol types.
 */
SYMBOL_TY:
	'type'
	| 'type_attribute'
	| 'role'
	| 'role_attribute'
	| 'user'
	| 'user_attribute'
	| 'sensitivity'
	| 'context';
// end::symbol_types[]

// tag:ident[]
/**
 * Identifiers can be prefixed with a '$' to indicate the variable referenced is a build-time
 * tunable option. This is used to infer whether a conditional should be a tunableif or booleanif
 */
IDENT: '$'? [a-z_][a-z_0-9]*;
// end:ident[]

// tag::logical_operators[]
/*
 * Logical and set/bitwise operators
 */
BITWISE_AND: '&';
BITWISE_OR: '|';
BITWISE_XOR: '^';
BITWISE_NOT: '~';

LOGICAL_NOT: '!';
LOGICAL_OR: '||';
LOGICAL_AND: '&&';
// end::logical_operators[]


// tag::terminals[]
/*
 * Delimiters, separators, and other misc. tokens
 */
OPEN_PAREN: '(';
CLOSE_PAREN: ')';

OPEN_BRACE: '{';
CLOSE_BRACE: '}';

COLON: ':';
DOT: '.';
SEMICOLON: ';';
EQUALS: '=';
COMMA: ',';
HYPHEN: '-';
// end::terminals[]

/*
 * Comments and whitespace (or otherwise hidden tokens).
 */
WS: [ \t\n] -> channel(HIDDEN);

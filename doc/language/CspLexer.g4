lexer grammar CspLexer;

/*
 * Statement and language keywords
 */

ABSTRACT: 'abstract';
INHERITS_FROM: 'inherits_from';
IF: 'if';
ELSEIF: 'elseif';
ELSE: 'else';
MACRO: 'macro';


CONTAINER_TY:
    'block'
    | 'optional'
    | 'in';

LIST_TY:
    'class'
    | 'common';

SYMBOL_TY:
	'type'
	| 'type_attribute'
	| 'role'
	| 'role_attribute'
	| 'user'
	| 'user_attribute'
	| 'sensitivity'
	| 'context';

/**
 * Identifiers can be prefixed with a '$' to indicate the variable referenced
 * is a build-time tunable option.  This is used to infer whether a conditional should
 * be a tunableif or booleanif
 */

IDENT: '$'?[a-z_][a-z_0-9]*;

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

/*
 * Comments and whitespace (or otherwise hidden tokens).
 */
WS: [ \t\n] -> channel(HIDDEN);

/* make parser reentrant */
%define api.pure full

/* use prefix 'foo_' instead of 'yy' */
%define api.prefix {foo_}

/* yylex and yyparse parameters */
%param {struct foo *data} {void *yyscanner}

/***********************************************************
* top block (only in foo-parser.c)
***********************************************************/

%code top {

#define TOP_BLOCK_STATEMENTS

}

/***********************************************************
* requires block (in foo-parser.c and foo-parser.h)
***********************************************************/

%code requires {

#include "foo-struct.h"

}

/***********************************************************
* provides block (in foo-parser.c and foo-parser.h)
***********************************************************/

%code provides {

extern int foo_lex(int *yylval, struct foo *data, void *yyscanner);
extern int foo_error(struct foo *data, void *yyscanner, const char *msg);
#define YY_DECL int foo_lex(int *yylval, struct foo *data, void *yyscanner)

}

%%

root:
	;

%%

int foo_error(struct foo *data, void *yyscanner, const char *msg)
{
	return 0;
}

%{
#include "foo-parser.h"
%}

%option reentrant
%option prefix = "foo_"
%option outfile = "foo-lexer.c"

%%

%%

int yywrap(yyscan_t yyscanner)
{
	return 1;
}

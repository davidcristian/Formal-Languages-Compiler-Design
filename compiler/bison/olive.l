%option noyywrap noinput nounput

%{ /* Declarations */
    #include <stdio.h>
    #include <stdlib.h>

    #include <stdint.h>
    #include <inttypes.h>

    #include "olive.tab.h"

    uint64_t line_number = 1;
    void lexical_error(const uint64_t *line, const char *token);   
%}

/* Definitions */
nonzero         [1-9]
digit           0|{nonzero}

uppercase       [A-Z]
lowercase       [a-z]
letter          {uppercase}|{lowercase}

/* ASCII 32-126, excluding 34 (") and 92 (\) */
symbol          [ -!]|[#-\[]|[\]-~]
/* \" and \\ */
escape          \\(\"|\\)
character       {letter}|{digit}|{symbol}|{escape}

number_constant ([+-]?{nonzero}{digit}*)|0
char_constant   '{character}'
string_constant \"{character}*\"

identifier      {letter}+
bad_identifier  {digit}*{letter}+{digit}*

comment         --
newline         \r\n|\n

%% /* Rules */

[ \t]+                  { /* Ignore spaces and tabs */ }
{comment}.*{newline}?   { line_number++; }
{newline}               { line_number++; }

"number"    { return NUMBER; }
"char"      { return CHAR; }
"string"    { return STRING; }
"array"     { return ARRAY; }
"input"     { return INPUT; }
"output"    { return OUTPUT; }
"if"        { return IF; }
"else"      { return ELSE; }
"while"     { return WHILE; }

"+"     { return PLUS; }
"-"     { return MINUS; }
"*"     { return TIMES; }
"/"     { return DIVIDE; }
"%"     { return MODULO; }
"="     { return ASSIGN; }
"=="    { return EQUAL; }
"!="    { return NOT_EQUAL; }
"<"     { return LESS; }
">"     { return GREATER; }
"<="    { return LESS_EQUAL; }
">="    { return GREATER_EQUAL; }
"&&"    { return AND; }
"||"    { return OR; }

"(" { return LEFT_PAREN; }
")" { return RIGHT_PAREN; }
"[" { return LEFT_BRACKET; }
"]" { return RIGHT_BRACKET; }
"{" { return LEFT_BRACE; }
"}" { return RIGHT_BRACE; }
":" { return COLON; }
";" { return SEMICOLON; }

{number_constant}   { return NUMBER_CONSTANT; }
{char_constant}     { return CHAR_CONSTANT; }
{string_constant}   { return STRING_CONSTANT; }

{identifier}        { return IDENTIFIER; }
{bad_identifier}    { lexical_error(&line_number, yytext); }

. { lexical_error(&line_number, yytext); }

%% /* Functions */

void lexical_error(const uint64_t *line, const char *token) {
    printf("Lexical error on line %" PRIu64 ": Unexpected token %s\n", *line, token);
    exit(1);
}

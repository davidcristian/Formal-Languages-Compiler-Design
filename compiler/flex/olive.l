%option noyywrap

%{ /* Declarations */
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>

    #include <stdint.h>
    #include <inttypes.h>

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
char_constant   '{character}*'
string_constant \"{character}*\"

identifier      {letter}+
bad_identifier  {digit}*{letter}+{digit}*

comment         --
newline         \r\n|\n

%% /* Rules */

[ \t]+ {
    /* Ignore spaces and tabs */
}

{comment}.*{newline}? {
    line_number++;
}

{newline} {
    line_number++;
}

"number"|"char"|"string"|"array"|"input"|"output"|"if"|"else"|"while" {
    printf("Reserved word: %s\n", yytext);
}

"+"|"-"|"*"|"/"|"%"|"="|"=="|"!="|"<"|">"|"<="|">="|"&&"|"||" {
    printf("Operator: %s\n", yytext);
}

"("|")"|"["|"]"|"{"|"}"|":"|";" {
    printf("Separator: %s\n", yytext);
}

{number_constant} {
    printf("Number constant: %s\n", yytext);
}

{char_constant} {
    if (strlen(yytext) != 3) {
        lexical_error(&line_number, yytext);
    } else {
        printf("Character constant: %s\n", yytext);
    }
}

{string_constant} {
    printf("String constant: %s\n", yytext);
}

{identifier} {
    printf("Identifier: %s\n", yytext);
}

{bad_identifier} {
    lexical_error(&line_number, yytext);
}

. {
    lexical_error(&line_number, yytext);
}

%% /* Functions */

extern int yylex(void);
extern FILE *yyin;

void lexical_error(const uint64_t *line, const char *token) {
    printf("Lexical error on line %" PRIu64 ": Undefined token %s\n", *line, token);
    
    fclose(yyin);
    exit(1);
}

int main(int argc, char **argv) {
    if (argc > 2) {
        printf("Usage: %s [file]\n", argv[0]);
        exit(1);
    }
    
    if (argc == 2) {
        yyin = fopen(argv[1], "r");
        if (yyin == NULL) {
            printf("Error: Could not open file %s\n", argv[1]);
            exit(1);
        }
    } else {
        yyin = stdin;
    }

    printf("Token list:\n");
    yylex();

    fclose(yyin);
    return 0;
}

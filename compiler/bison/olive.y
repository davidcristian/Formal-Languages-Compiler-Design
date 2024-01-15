%{ /* Declarations */
    #include <stdio.h>
    #include <stdlib.h>

    #include "olive.h"
    #define YYDEBUG 1

    void yyerror(const char *s);
%}

/* Tokens */
%token IDENTIFIER
%token NUMBER_CONSTANT
%token CHAR_CONSTANT
%token STRING_CONSTANT

%token PLUS
%token MINUS
%token TIMES
%token DIVIDE
%token MODULO
%token ASSIGN
%token EQUAL
%token NOT_EQUAL
%token LESS
%token GREATER
%token LESS_EQUAL
%token GREATER_EQUAL
%token AND
%token OR

%token LEFT_PAREN
%token RIGHT_PAREN
%token LEFT_BRACKET
%token RIGHT_BRACKET
%token LEFT_BRACE
%token RIGHT_BRACE
%token COLON
%token SEMICOLON

%token NUMBER
%token CHAR
%token STRING
%token ARRAY
%token INPUT
%token OUTPUT
%token IF
%token ELSE
%token WHILE

%start Program

%% /* Grammar rules */

Constant: NUMBER_CONSTANT { printf("Constant -> NUMBER_CONSTANT\n"); }
        | CHAR_CONSTANT   { printf("Constant -> CHAR_CONSTANT\n"); }
        | STRING_CONSTANT { printf("Constant -> STRING_CONSTANT\n"); }
        ;

Program: StatementList { printf("Program -> StatementList\n"); }
       ;

Variable: IDENTIFIER OptionalArray { printf("Variable -> IDENTIFIER OptionalArray\n"); }
        ;

OptionalArray: LEFT_BRACKET Expression RIGHT_BRACKET { printf("OptionalArray -> [ Expression ]\n"); }
             | /* empty */                           { printf("OptionalArray -> epsilon\n"); }
             ;

Value: Constant { printf("Value -> Constant\n"); }
     | Variable { printf("Value -> Variable\n"); }
     ;

Declaration: Type OptionalAssignment { printf("Declaration -> Type OptionalAssignment\n"); }
           ;

OptionalAssignment: ASSIGN Expression { printf("OptionalAssignment -> = Expression\n"); }
                  | /* empty */       { printf("OptionalAssignment -> epsilon\n"); }
                  ;

Primitive: NUMBER { printf("Primitive -> NUMBER\n"); }
         | CHAR   { printf("Primitive -> CHAR\n"); }
         | STRING { printf("Primitive -> STRING\n"); }
         ;

ArrayDeclaration: ARRAY LEFT_PAREN Primitive RIGHT_PAREN LEFT_BRACKET Expression RIGHT_BRACKET { printf("ArrayDeclaration -> ARRAY ( Primitive ) [ Expression ]\n"); }
                ;

Type: Primitive        { printf("Type -> Primitive\n"); }
    | ArrayDeclaration { printf("Type -> ArrayDeclaration\n"); }
    ;

CompoundStatement: LEFT_BRACE StatementList RIGHT_BRACE { printf("CompoundStatement -> { StatementList }\n"); }
                 ;

StatementList: Statement SEMICOLON StatementListTail { printf("StatementList -> Statement ; StatementListTail\n"); }
             ;

StatementListTail: StatementList { printf("StatementListTail -> StatementList\n"); }
                 | /* empty */   { printf("StatementListTail -> epsilon\n"); }
                 ;

Statement: IDENTIFIER StatementTail { printf("Statement -> IDENTIFIER StatementTail\n"); }
         | IOStatement              { printf("Statement -> IOStatement\n"); }
         | IfStatement              { printf("Statement -> IfStatement\n"); }
         | WhileStatement           { printf("Statement -> WhileStatement\n"); }
         ;

StatementTail: COLON Declaration               { printf("StatementTail -> : Declaration\n"); }
             | OptionalArray ASSIGN Expression { printf("StatementTail -> OptionalArray = Expression\n"); }
             ;

Factor: LEFT_PAREN Expression RIGHT_PAREN { printf("Factor -> ( Expression )\n"); }
      | Value                             { printf("Factor -> Value\n"); }
      ;

Expression: Term ExpressionTail { printf("Expression -> Term ExpressionTail\n"); }
          ;

ExpressionTail: PLUS Term ExpressionTail  { printf("ExpressionTail -> + Term ExpressionTail\n"); }
              | MINUS Term ExpressionTail { printf("ExpressionTail -> - Term ExpressionTail\n"); }
              | /* empty */               { printf("ExpressionTail -> epsilon\n"); }
              ;

Term: Factor TermTail { printf("Term -> Factor TermTail\n"); }
    ;

TermTail: TIMES Factor TermTail  { printf("TermTail -> * Factor TermTail\n"); }
        | DIVIDE Factor TermTail { printf("TermTail -> / Factor TermTail\n"); }
        | MODULO Factor TermTail { printf("TermTail -> %% Factor TermTail\n"); }
        | /* empty */            { printf("TermTail -> epsilon\n"); }
        ;

IOStatement: INPUT LEFT_PAREN Variable RIGHT_PAREN    { printf("IOStatement -> INPUT ( Variable )\n"); }
           | OUTPUT LEFT_PAREN Expression RIGHT_PAREN { printf("IOStatement -> OUTPUT ( Expression )\n"); }
           ;

IfStatement: IF LEFT_PAREN Conditions RIGHT_PAREN CompoundStatement ElseStatement { printf("IfStatement -> IF ( Conditions ) CompoundStatement ElseStatement\n"); }
           ;

ElseStatement: ELSE CompoundStatement { printf("ElseStatement -> ELSE CompoundStatement\n"); }
             | /* empty */            { printf("ElseStatement -> epsilon\n"); }
             ;

WhileStatement: WHILE LEFT_PAREN Conditions RIGHT_PAREN CompoundStatement { printf("WhileStatement -> WHILE ( Conditions ) CompoundStatement\n"); }
              ;

Conditions: Condition ConditionsTail { printf("Conditions -> Condition ConditionsTail\n"); }
          ;

ConditionsTail: OR Condition ConditionsTail { printf("ConditionsTail -> OR Condition ConditionsTail\n"); }
              | /* empty */                 { printf("ConditionsTail -> epsilon\n"); }
              ;

Condition: SimpleCondition ConditionTail { printf("Condition -> SimpleCondition ConditionTail\n"); }
         ;

ConditionTail: AND SimpleCondition ConditionTail { printf("ConditionTail -> AND SimpleCondition ConditionTail\n"); }
             | /* empty */                       { printf("ConditionTail -> epsilon\n"); }
             ;

SimpleCondition: Expression Relation Expression { printf("SimpleCondition -> Expression Relation Expression\n"); }
               ;

Relation: EQUAL         { printf("Relation -> ==\n"); }
        | NOT_EQUAL     { printf("Relation -> !=\n"); }
        | LESS          { printf("Relation -> <\n"); }
        | GREATER       { printf("Relation -> >\n"); }
        | LESS_EQUAL    { printf("Relation -> <=\n"); }
        | GREATER_EQUAL { printf("Relation -> >=\n"); }
        ;

%% /* Functions */

extern FILE *yyin;
extern int yyparse(void);

void yyerror(const char *s) {
    printf("Parser error: %s\n", s);

    fclose(yyin);
    exit(1);
}

int main(int argc, char **argv) {
    if (argc > 2) {
        printf("Usage: %s [file]\n", argv[0]);
        return 1;
    }

    if (argc == 2) {
        yyin = fopen(argv[1], "r");
        if (yyin == NULL) {
            printf("Error: Could not open file %s\n", argv[1]);
            return 1;
        }
    } else {
        yyin = stdin;
    }

    yyparse();
    fclose(yyin);
    return 0;
}

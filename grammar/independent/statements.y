/* Independently authored source and statement syntax; see LANGUAGE.md. */
start:
    file_items
;
file_items:
    %empty | file_items file_item
;
file_item:
    block_item
  | namespace_definition
  | namespace_import
  | constant_declaration
  | attributes constant_declaration
  | file_halt
;
block_items:
    %empty | block_items block_item
;
block_item:
    statement | function_definition | type_definition
  | attributes function_definition | attributes type_definition
;
block:
    '{' block_items '}'
;
statement:
    ';' | T_INLINE_HTML | block
  | expression ';'
  | T_VOID_CAST expression ';'
  | T_ECHO expression_list ';'
  | T_RETURN expression_opt ';'
  | T_BREAK expression_opt ';'
  | T_CONTINUE expression_opt ';'
  | T_GOTO T_STRING ';'
  | T_STRING ':'
  | T_GLOBAL global_variables ';'
  | T_STATIC local_bindings ';'
  | T_UNSET '(' place_list trailing_comma ')' ';'
  | conditional_statement
  | while_statement
  | do_statement
  | for_statement
  | foreach_statement
  | switch_statement
  | try_statement
  | declare_statement
;
global_variables:
    variable | global_variables ',' variable
;
local_bindings:
    local_binding | local_bindings ',' local_binding
;
local_binding:
    T_VARIABLE default_value
;
conditional_statement:
    T_IF '(' expression ')' statement conditional_tail
  | T_IF '(' expression ')' ':' block_items alternative_conditions alternative_else T_ENDIF ';'
;
conditional_tail:
    %empty %prec P_IF
  | T_ELSE statement
  | T_ELSEIF '(' expression ')' statement conditional_tail
;
alternative_conditions:
    %empty
  | alternative_conditions T_ELSEIF '(' expression ')' ':' block_items
;
alternative_else:
    %empty | T_ELSE ':' block_items
;
while_statement:
    T_WHILE '(' expression ')' statement
  | T_WHILE '(' expression ')' ':' block_items T_ENDWHILE ';'
;
do_statement:
    T_DO statement T_WHILE '(' expression ')' ';'
;
for_statement:
    T_FOR '(' discarded_expressions ';' for_condition ';' discarded_expressions ')' statement
  | T_FOR '(' discarded_expressions ';' for_condition ';' discarded_expressions ')' ':' block_items T_ENDFOR ';'
;
discarded_expressions:
    %empty | discarded_list
;
discarded_list:
    discarded_expression | discarded_list ',' discarded_expression
;
discarded_expression:
    expression | T_VOID_CAST expression
;
for_condition:
    %empty | expression | discarded_list ',' expression
;
foreach_statement:
    T_FOREACH '(' expression T_AS foreach_binding ')' statement
  | T_FOREACH '(' expression T_AS foreach_binding ')' ':' block_items T_ENDFOREACH ';'
;
foreach_binding:
    foreach_value | place T_DOUBLE_ARROW foreach_value
;
foreach_value:
    place | reference_mark place | array_literal | list_pattern
;
switch_statement:
    T_SWITCH '(' expression ')' '{' optional_semicolon switch_sections '}'
  | T_SWITCH '(' expression ')' ':' optional_semicolon switch_sections T_ENDSWITCH ';'
;
optional_semicolon:
    %empty | ';'
;
switch_sections:
    %empty
  | switch_sections T_CASE expression case_separator block_items
  | switch_sections T_DEFAULT case_separator block_items
;
case_separator:
    ':' | ';'
;
try_statement:
    T_TRY block catch_clauses finally_opt
  | T_TRY block T_FINALLY block
;
catch_clauses:
    catch_clause | catch_clauses catch_clause
;
catch_clause:
    T_CATCH '(' catch_types catch_variable ')' block
;
catch_types:
    name | catch_types '|' name
;
catch_variable:
    %empty | T_VARIABLE
;
finally_opt:
    %empty | T_FINALLY block
;
declare_statement:
    T_DECLARE '(' directives ')' statement
  | T_DECLARE '(' directives ')' ':' block_items T_ENDDECLARE ';'
;
directives:
    directive | directives ',' directive
;
directive:
    T_STRING '=' expression
;
file_halt:
    T_HALT_COMPILER '(' ')' ';'
;

place_list:
    place | place_list ',' place
;

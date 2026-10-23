/* Copyright 2026 Geraldo Netto
 * SPDX-License-Identifier: Apache-2.0
 */

%start start
%token T_ABSTRACT
%token T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG
%token T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG
%token T_AND_EQUAL
%token T_ARRAY
%token T_ARRAY_CAST
%token T_AS
%token T_ATTRIBUTE
%token T_BAD_CHARACTER
%token T_BOOLEAN_AND
%token T_BOOLEAN_OR
%token T_BOOL_CAST
%token T_BREAK
%token T_CALLABLE
%token T_CASE
%token T_CATCH
%token T_CLASS
%token T_CLASS_C
%token T_CLONE
%token T_CLOSE_TAG
%token T_COALESCE
%token T_COALESCE_EQUAL
%token T_COMMENT
%token T_CONCAT_EQUAL
%token T_CONST
%token T_CONSTANT_ENCAPSED_STRING
%token T_CONTINUE
%token T_CURLY_OPEN
%token T_DEC
%token T_DECLARE
%token T_DEFAULT
%token T_DIR
%token T_DIV_EQUAL
%token T_DNUMBER
%token T_DO
%token T_DOC_COMMENT
%token T_DOLLAR_OPEN_CURLY_BRACES
%token T_DOUBLE_ARROW
%token T_DOUBLE_CAST
%token T_DOUBLE_COLON
%token T_ECHO
%token T_ELLIPSIS
%token T_ELSE
%token T_ELSEIF
%token T_EMPTY
%token T_ENCAPSED_AND_WHITESPACE
%token T_ENDDECLARE
%token T_ENDFOR
%token T_ENDFOREACH
%token T_ENDIF
%token T_ENDSWITCH
%token T_ENDWHILE
%token T_END_HEREDOC
%token T_ENUM
%token T_EVAL
%token T_EXIT
%token T_EXTENDS
%token T_FILE
%token T_FINAL
%token T_FINALLY
%token T_FN
%token T_FOR
%token T_FOREACH
%token T_FUNCTION
%token T_FUNC_C
%token T_GLOBAL
%token T_GOTO
%token T_HALT_COMPILER
%token T_IF
%token T_IMPLEMENTS
%token T_INC
%token T_INCLUDE
%token T_INCLUDE_ONCE
%token T_INLINE_HTML
%token T_INSTANCEOF
%token T_INSTEADOF
%token T_INTERFACE
%token T_INT_CAST
%token T_ISSET
%token T_IS_EQUAL
%token T_IS_GREATER_OR_EQUAL
%token T_IS_IDENTICAL
%token T_IS_NOT_EQUAL
%token T_IS_NOT_IDENTICAL
%token T_IS_SMALLER_OR_EQUAL
%token T_LINE
%token T_LIST
%token T_LNUMBER
%token T_LOGICAL_AND
%token T_LOGICAL_OR
%token T_LOGICAL_XOR
%token T_MATCH
%token T_METHOD_C
%token T_MINUS_EQUAL
%token T_MOD_EQUAL
%token T_MUL_EQUAL
%token T_NAMESPACE
%token T_NAME_FULLY_QUALIFIED
%token T_NAME_QUALIFIED
%token T_NAME_RELATIVE
%token T_NEW
%token T_NS_C
%token T_NS_SEPARATOR
%token T_NULLSAFE_OBJECT_OPERATOR
%token T_NUM_STRING
%token T_OBJECT_CAST
%token T_OBJECT_OPERATOR
%token T_OPEN_TAG
%token T_OPEN_TAG_WITH_ECHO
%token T_OR_EQUAL
%token T_PIPE
%token T_PLUS_EQUAL
%token T_POW
%token T_POW_EQUAL
%token T_PRINT
%token T_PRIVATE
%token T_PRIVATE_SET
%token T_PROPERTY_C
%token T_PROTECTED
%token T_PROTECTED_SET
%token T_PUBLIC
%token T_PUBLIC_SET
%token T_READONLY
%token T_REQUIRE
%token T_REQUIRE_ONCE
%token T_RETURN
%token T_SL
%token T_SL_EQUAL
%token T_SPACESHIP
%token T_SR
%token T_SR_EQUAL
%token T_START_HEREDOC
%token T_STATIC
%token T_STRING
%token T_STRING_CAST
%token T_STRING_VARNAME
%token T_SWITCH
%token T_THROW
%token T_TRAIT
%token T_TRAIT_C
%token T_TRY
%token T_UNSET
%token T_USE
%token T_VAR
%token T_VARIABLE
%token T_VOID_CAST
%token T_WHILE
%token T_WHITESPACE
%token T_XOR_EQUAL
%token T_YIELD
%token T_YIELD_FROM
/* Copyright 2026 Geraldo Netto
 * SPDX-License-Identifier: Apache-2.0
 */

%precedence P_ARROW
%left T_LOGICAL_OR
%left T_LOGICAL_XOR
%left T_LOGICAL_AND
%precedence T_PRINT
%precedence T_YIELD
%precedence T_DOUBLE_ARROW
%precedence T_YIELD_FROM
%precedence T_THROW T_INCLUDE T_INCLUDE_ONCE T_REQUIRE T_REQUIRE_ONCE
%right '=' T_PLUS_EQUAL T_MINUS_EQUAL T_MUL_EQUAL T_DIV_EQUAL T_MOD_EQUAL T_CONCAT_EQUAL T_AND_EQUAL T_OR_EQUAL T_XOR_EQUAL T_SL_EQUAL T_SR_EQUAL T_POW_EQUAL T_COALESCE_EQUAL
%left '?' ':'
%right T_COALESCE
%left T_BOOLEAN_OR
%left T_BOOLEAN_AND
%left '|'
%left '^'
%left P_BIT_AND T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG
%nonassoc T_IS_EQUAL T_IS_NOT_EQUAL T_IS_IDENTICAL T_IS_NOT_IDENTICAL T_SPACESHIP
%nonassoc '<' '>' T_IS_SMALLER_OR_EQUAL T_IS_GREATER_OR_EQUAL
%left T_PIPE
%left '.'
%left T_SL T_SR
%left '+' '-'
%left '*' '/' '%'
%precedence '!'
%left T_INSTANCEOF
%precedence P_UNARY
%right T_POW
%precedence P_CONSTRUCT
%precedence P_IF
%precedence T_ELSE T_ELSEIF
%precedence P_REFERENCE
%precedence P_BARE_CALL P_ATOM
%precedence '(' '[' T_OBJECT_OPERATOR T_NULLSAFE_OBJECT_OPERATOR T_DOUBLE_COLON T_INC T_DEC

%expect 0
%%
/* Copyright 2026 Geraldo Netto
 * SPDX-License-Identifier: Apache-2.0
 */

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

/* Copyright 2026 Geraldo Netto
 * SPDX-License-Identifier: Apache-2.0
 */

/* Independently authored expression syntax; see LANGUAGE.md. No engine grammar input. */
expression:
    postfix %prec P_ATOM
  | place '=' expression
  | place assignment_operator expression %prec '='
  | reference_assignment
  | expression T_LOGICAL_OR expression
  | expression T_LOGICAL_XOR expression
  | expression T_LOGICAL_AND expression
  | expression T_BOOLEAN_OR expression
  | expression T_BOOLEAN_AND expression
  | expression '|' expression
  | expression '^' expression
  | expression reference_mark expression %prec P_BIT_AND
  | expression T_IS_EQUAL expression
  | expression T_IS_NOT_EQUAL expression
  | expression T_IS_IDENTICAL expression
  | expression T_IS_NOT_IDENTICAL expression
  | expression T_SPACESHIP expression
  | expression '<' expression
  | expression '>' expression
  | expression T_IS_SMALLER_OR_EQUAL expression
  | expression T_IS_GREATER_OR_EQUAL expression
  | expression T_PIPE expression
  | expression '.' expression
  | expression T_SL expression
  | expression T_SR expression
  | expression '+' expression
  | expression '-' expression
  | expression '*' expression
  | expression '/' expression
  | expression '%' expression
  | expression T_POW expression
  | expression T_COALESCE expression
  | expression T_INSTANCEOF class_target
  | expression '?' expression ':' expression
  | expression '?' ':' expression
  | '+' expression %prec P_UNARY
  | '-' expression %prec P_UNARY
  | '!' expression
  | '~' expression %prec P_UNARY
  | '@' expression %prec P_UNARY
  | cast_operator expression %prec P_UNARY
  | prefix_mutation
  | place T_INC
  | place T_DEC
  | T_NEW class_target %prec P_CONSTRUCT
  | T_CLONE expression %prec P_CONSTRUCT
  | T_PRINT expression
  | T_THROW expression
  | T_INCLUDE expression
  | T_INCLUDE_ONCE expression
  | T_REQUIRE expression
  | T_REQUIRE_ONCE expression
  | T_YIELD
  | T_YIELD expression
  | T_YIELD expression T_DOUBLE_ARROW expression
  | T_YIELD_FROM expression
  | T_EXIT call_arguments_opt
  | array_literal '=' expression
  | list_pattern '=' expression
;
assignment_operator:
    T_PLUS_EQUAL | T_MINUS_EQUAL | T_MUL_EQUAL | T_DIV_EQUAL | T_MOD_EQUAL
  | T_CONCAT_EQUAL | T_AND_EQUAL | T_OR_EQUAL | T_XOR_EQUAL
  | T_SL_EQUAL | T_SR_EQUAL | T_POW_EQUAL | T_COALESCE_EQUAL
;
reference_mark:
    T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG
  | T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG
;
cast_operator:
    T_INT_CAST | T_DOUBLE_CAST | T_STRING_CAST | T_ARRAY_CAST
  | T_OBJECT_CAST | T_BOOL_CAST
;
postfix:
    atom
  | place
  | postfix T_DOUBLE_COLON member_identifier
  | postfix T_DOUBLE_COLON '{' expression '}'
  | T_STATIC T_DOUBLE_COLON member_identifier
  | T_STATIC T_DOUBLE_COLON '{' expression '}'
;
place:
    variable
  | postfix call_arguments
  | postfix T_NULLSAFE_OBJECT_OPERATOR property_selector
  | postfix '[' expression_opt ']'
  | postfix T_OBJECT_OPERATOR property_selector
  | postfix T_DOUBLE_COLON variable
  | T_STATIC T_DOUBLE_COLON variable
;
property_selector:
    T_STRING | variable | '{' expression '}'
;
atom:
    name
  | literal
  | '(' expression ')'
  | array_literal
  | closure_expression
  | arrow_expression
  | constructed_object
  | match_expression
  | T_ISSET '(' expression_list trailing_comma ')'
  | T_EMPTY '(' expression ')'
  | T_EVAL '(' expression ')'
  | T_CLONE clone_arguments
;
name:
    T_STRING | T_NAME_QUALIFIED | T_NAME_FULLY_QUALIFIED | T_NAME_RELATIVE
;
variable:
    T_VARIABLE
  | '$' variable
  | '$' '{' expression '}'
;
call_arguments_opt:
    %empty %prec P_BARE_CALL | call_arguments
;
call_arguments:
    '(' ')'
  | '(' argument_list trailing_comma ')'
  | '(' T_ELLIPSIS ')'
;
argument_list:
    argument | argument_list ',' argument
;
argument:
    expression
  | member_identifier ':' expression
  | T_ELLIPSIS expression
;
clone_arguments:
    '(' ')'
  | '(' T_ELLIPSIS ')'
  | '(' named_or_spread_list trailing_comma ')'
  | '(' expression ',' ')'
  | '(' expression ',' argument_list trailing_comma ')'
;
named_or_spread_list:
    named_or_spread_argument
  | named_or_spread_list ',' argument
;
named_or_spread_argument:
    member_identifier ':' expression
  | T_ELLIPSIS expression
;
array_literal:
    '[' array_slots ']'
  | T_ARRAY '(' array_slots ')'
;
array_slots:
    array_entries
;
array_entries:
    array_entry | array_entries ',' array_entry
;
array_entry:
    %empty
  | expression
  | reference_mark expression
  | expression T_DOUBLE_ARROW expression
  | expression T_DOUBLE_ARROW reference_mark expression
  | T_ELLIPSIS expression
;
list_pattern:
    T_LIST '(' pattern_slots ')'
;
pattern_slots:
    pattern_entries
;
pattern_entries:
    pattern_entry | pattern_entries ',' pattern_entry
;
pattern_entry:
    %empty | expression | list_pattern | reference_mark expression
  | expression T_DOUBLE_ARROW expression
  | expression T_DOUBLE_ARROW list_pattern
  | expression T_DOUBLE_ARROW reference_mark expression
;
class_name:
    name | T_STATIC
;
class_target:
    class_name %prec P_CONSTRUCT
  | dynamic_class_target %prec P_CONSTRUCT
  | '(' expression ')'
;
dynamic_class_target:
    variable
  | class_name T_DOUBLE_COLON variable
  | dynamic_class_target '[' expression ']'
  | dynamic_class_target T_OBJECT_OPERATOR property_selector
  | dynamic_class_target T_DOUBLE_COLON variable
;
constructed_object:
    T_NEW class_target call_arguments
  | T_NEW anonymous_object
;
anonymous_object:
    attributes_opt anonymous_modifiers T_CLASS call_arguments_opt class_parent interfaces class_body
;
anonymous_modifiers:
    %empty | T_READONLY
;
closure_expression:
    T_FUNCTION reference_opt parameter_clause captures return_annotation block
  | T_STATIC T_FUNCTION reference_opt parameter_clause captures return_annotation block
  | attributes T_FUNCTION reference_opt parameter_clause captures return_annotation block
  | attributes T_STATIC T_FUNCTION reference_opt parameter_clause captures return_annotation block
;
captures:
    %empty | T_USE '(' capture_list trailing_comma ')'
;
capture_list:
    capture | capture_list ',' capture
;
capture:
    reference_opt T_VARIABLE
;
arrow_expression:
    T_FN reference_opt parameter_clause return_annotation T_DOUBLE_ARROW expression %prec P_ARROW
  | T_STATIC T_FN reference_opt parameter_clause return_annotation T_DOUBLE_ARROW expression %prec P_ARROW
  | attributes T_FN reference_opt parameter_clause return_annotation T_DOUBLE_ARROW expression %prec P_ARROW
  | attributes T_STATIC T_FN reference_opt parameter_clause return_annotation T_DOUBLE_ARROW expression %prec P_ARROW
;
match_expression:
    T_MATCH '(' expression ')' '{' match_arms '}'
;
match_arms:
    %empty | match_arm_list trailing_comma
;
match_arm_list:
    match_arm | match_arm_list ',' match_arm
;
match_arm:
    expression_list T_DOUBLE_ARROW expression
  | T_DEFAULT T_DOUBLE_ARROW expression
  | T_DEFAULT ',' T_DOUBLE_ARROW expression
;
literal:
    T_LNUMBER | T_DNUMBER | T_CONSTANT_ENCAPSED_STRING
  | T_LINE | T_FILE | T_DIR | T_CLASS_C | T_TRAIT_C | T_METHOD_C
  | T_FUNC_C | T_NS_C | T_PROPERTY_C
  | '"' interpolated_parts '"'
  | '`' interpolated_parts '`'
  | T_START_HEREDOC interpolated_parts T_END_HEREDOC
;
interpolated_parts:
    %empty | interpolated_parts interpolated_part
;
interpolated_part:
    T_ENCAPSED_AND_WHITESPACE
  | T_VARIABLE
  | T_VARIABLE '[' interpolation_index ']'
  | T_VARIABLE T_OBJECT_OPERATOR T_STRING
  | T_VARIABLE T_NULLSAFE_OBJECT_OPERATOR T_STRING
  | T_CURLY_OPEN expression '}'
  | T_DOLLAR_OPEN_CURLY_BRACES expression '}'
  | T_DOLLAR_OPEN_CURLY_BRACES T_STRING_VARNAME '}'
  | T_DOLLAR_OPEN_CURLY_BRACES T_STRING_VARNAME '[' expression ']' '}'
;
interpolation_index:
    T_STRING | T_NUM_STRING | T_VARIABLE | '-' T_NUM_STRING
;
expression_opt:
    %empty | expression
;
expression_list:
    expression | expression_list ',' expression
;
trailing_comma:
    %empty | ','
;

prefix_mutation:
    T_INC expression %prec P_UNARY
  | T_DEC expression %prec P_UNARY
;
reference_assignment:
    place '=' reference_mark expression %prec P_REFERENCE
;

/* Copyright 2026 Geraldo Netto
 * SPDX-License-Identifier: Apache-2.0
 */

/* Independently authored declarations and types; see LANGUAGE.md. */
function_definition:
    T_FUNCTION reference_opt T_STRING parameter_clause return_annotation block
;
reference_opt:
    %empty | reference_mark
;
parameter_clause:
    '(' ')' | '(' parameters trailing_comma ')'
;
parameters:
    parameter | parameters ',' parameter
;
parameter:
    attributes_opt promotion_modifiers type_opt parameter_reference_opt variadic_opt T_VARIABLE default_value parameter_hooks
;
promotion_modifiers:
    %empty | promotion_modifiers member_modifier
;
parameter_reference_opt:
    %empty | T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG
;
variadic_opt:
    %empty | T_ELLIPSIS
;
default_value:
    %empty | '=' expression
;
parameter_hooks:
    %empty | hook_body
;
return_annotation:
    %empty | ':' return_type
;
return_type:
    return_simple | '?' return_simple | return_intersection | return_union
;
return_simple:
    simple_type | T_STATIC
;
return_intersection:
    return_simple T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG return_simple
  | return_intersection T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG return_simple
;
return_union:
    return_union_part '|' return_union_part | return_union '|' return_union_part
;
return_union_part:
    return_simple | '(' return_intersection ')'
;
type_opt:
    %empty | type
;
type:
    simple_type
  | '?' simple_type
  | intersection_type
  | union_type
;
simple_type:
    name | T_ARRAY | T_CALLABLE
;
intersection_type:
    simple_type T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG simple_type
  | intersection_type T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG simple_type
;
union_type:
    union_part '|' union_part
  | union_type '|' union_part
;
union_part:
    simple_type | '(' intersection_type ')'
;
type_definition:
    class_modifiers T_CLASS T_STRING class_parent interfaces class_body
  | T_INTERFACE T_STRING interface_parents class_body
  | T_TRAIT T_STRING class_body
  | T_ENUM T_STRING enum_backing interfaces class_body
;
class_modifiers:
    %empty | class_modifiers class_modifier
;
class_modifier:
    T_ABSTRACT | T_FINAL | T_READONLY
;
class_parent:
    %empty | T_EXTENDS name
;
interfaces:
    %empty | T_IMPLEMENTS name_list
;
interface_parents:
    %empty | T_EXTENDS name_list
;
name_list:
    name | name_list ',' name
;
enum_backing:
    %empty | ':' T_STRING
;
class_body:
    '{' members '}'
;
members:
    %empty | members member
;
member:
    attributes_opt property_header type_opt property_list ';'
  | attributes_opt property_header type_opt T_VARIABLE default_value hook_body
  | attributes_opt method_modifiers T_FUNCTION reference_opt member_identifier parameter_clause return_annotation method_body
  | attributes_opt method_modifiers T_CONST constant_bindings ';'
  | attributes_opt method_modifiers T_CONST type constant_bindings ';'
  | trait_import
  | attributes_opt T_CASE T_STRING default_value ';'
;
property_header:
    member_modifiers | T_VAR
;
method_modifiers:
    %empty | member_modifiers
;
member_modifiers:
    member_modifier | member_modifiers member_modifier
;
member_modifier:
    T_PUBLIC | T_PROTECTED | T_PRIVATE | T_STATIC | T_ABSTRACT | T_FINAL | T_READONLY
  | T_PUBLIC_SET | T_PROTECTED_SET | T_PRIVATE_SET
;
property_list:
    property_binding | property_list ',' property_binding
;
property_binding:
    T_VARIABLE default_value
;
method_body:
    ';' | block
;
hook_body:
    '{' hooks '}'
;
hooks:
    %empty | hooks hook
;
hook:
    attributes_opt method_modifiers reference_opt T_STRING hook_parameters hook_implementation
;
hook_parameters:
    %empty | parameter_clause
;
hook_implementation:
    ';' | block | T_DOUBLE_ARROW expression ';'
;
trait_import:
    T_USE name_list ';'
  | T_USE name_list '{' trait_adjustments '}'
;
trait_adjustments:
    %empty | trait_adjustments trait_adjustment
;
trait_adjustment:
    trait_method T_INSTEADOF name_list ';'
  | trait_method T_AS alias_identifier ';'
  | trait_method T_AS trait_modifier member_identifier ';'
  | trait_method T_AS trait_modifier ';'
;
trait_modifier:
    T_PUBLIC | T_PRIVATE | T_PROTECTED | T_FINAL
;
trait_method:
    member_identifier | name T_DOUBLE_COLON member_identifier
;
constant_declaration:
    T_CONST constant_bindings ';'
;
constant_bindings:
    constant_binding | constant_bindings ',' constant_binding
;
constant_binding:
    T_STRING '=' expression
;
namespace_definition:
    T_NAMESPACE namespace_path ';'
  | T_NAMESPACE namespace_path '{' file_items '}'
  | T_NAMESPACE '{' file_items '}'
;
namespace_path:
    T_STRING | T_NAME_QUALIFIED
;
namespace_import:
    T_USE import_kind imports ';'
  | T_USE import_kind import_prefix '{' group_imports trailing_comma '}' ';'
;
import_kind:
    %empty | T_FUNCTION | T_CONST
;
imports:
    import_entry | imports ',' import_entry
;
import_entry:
    name import_alias
;
import_alias:
    %empty | T_AS T_STRING
;
import_prefix:
    name T_NS_SEPARATOR
;
group_imports:
    group_import | group_imports ',' group_import
;
group_import:
    import_kind namespace_path import_alias
;
attributes_opt:
    %empty | attributes
;
attributes:
    attribute_group | attributes attribute_group
;
attribute_group:
    T_ATTRIBUTE attribute_list trailing_comma ']'
;
attribute_list:
    attribute | attribute_list ',' attribute
;
attribute:
    name call_arguments_opt
;
member_identifier:
    T_STRING
  | T_ABSTRACT | T_ARRAY | T_AS | T_BREAK | T_CALLABLE | T_CASE | T_CATCH | T_CLASS
  | T_CLONE | T_CONST | T_CONTINUE | T_DECLARE | T_DEFAULT | T_DO | T_ECHO | T_ELSE
  | T_ELSEIF | T_EMPTY | T_ENDDECLARE | T_ENDFOR | T_ENDFOREACH | T_ENDIF
  | T_ENDSWITCH | T_ENDWHILE | T_ENUM | T_EVAL | T_EXIT | T_EXTENDS | T_FINAL
  | T_FINALLY | T_FN | T_FOR | T_FOREACH | T_FUNCTION | T_GLOBAL | T_GOTO
  | T_IF | T_IMPLEMENTS | T_INCLUDE | T_INCLUDE_ONCE | T_INSTANCEOF | T_INSTEADOF
  | T_INTERFACE | T_ISSET | T_LIST | T_LOGICAL_AND | T_LOGICAL_OR | T_LOGICAL_XOR
  | T_MATCH | T_NAMESPACE | T_NEW | T_PRINT | T_PRIVATE | T_PROTECTED | T_PUBLIC
  | T_READONLY | T_REQUIRE | T_REQUIRE_ONCE | T_RETURN | T_STATIC | T_SWITCH
  | T_THROW | T_TRAIT | T_TRY | T_UNSET | T_USE | T_VAR | T_WHILE | T_YIELD
  | T_CLASS_C | T_DIR | T_FILE | T_FUNC_C | T_LINE | T_METHOD_C | T_NS_C | T_TRAIT_C
  | T_PROPERTY_C
;

alias_identifier:
    T_STRING | T_ARRAY | T_AS | T_BREAK | T_CALLABLE | T_CASE | T_CATCH | T_CLASS | T_CLONE | T_CONST | T_CONTINUE | T_DECLARE | T_DEFAULT | T_DO | T_ECHO | T_ELSE | T_ELSEIF | T_EMPTY | T_ENDDECLARE | T_ENDFOR | T_ENDFOREACH | T_ENDIF | T_ENDSWITCH | T_ENDWHILE | T_ENUM | T_EVAL | T_EXIT | T_EXTENDS | T_FINALLY | T_FN | T_FOR | T_FOREACH | T_FUNCTION | T_GLOBAL | T_GOTO | T_IF | T_IMPLEMENTS | T_INCLUDE | T_INCLUDE_ONCE | T_INSTANCEOF | T_INSTEADOF | T_INTERFACE | T_ISSET | T_LIST | T_LOGICAL_AND | T_LOGICAL_OR | T_LOGICAL_XOR | T_MATCH | T_NAMESPACE | T_NEW | T_PRINT | T_REQUIRE | T_REQUIRE_ONCE | T_RETURN | T_SWITCH | T_THROW | T_TRAIT | T_TRY | T_UNSET | T_USE | T_VAR | T_WHILE | T_YIELD | T_CLASS_C | T_DIR | T_FILE | T_FUNC_C | T_LINE | T_METHOD_C | T_NS_C | T_TRAIT_C | T_PROPERTY_C
;

%%

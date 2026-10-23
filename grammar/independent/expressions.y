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
;
place:
    variable
  | postfix call_arguments
  | postfix T_NULLSAFE_OBJECT_OPERATOR property_selector
  | postfix '[' expression_opt ']'
  | postfix T_OBJECT_OPERATOR property_selector
  | postfix T_DOUBLE_COLON variable
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

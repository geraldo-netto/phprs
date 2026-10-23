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

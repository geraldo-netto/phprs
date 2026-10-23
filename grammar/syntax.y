%precedence T_THROW
%precedence PREC_ARROW_FUNCTION
%precedence T_INCLUDE T_INCLUDE_ONCE T_REQUIRE T_REQUIRE_ONCE
%left T_LOGICAL_OR
%left T_LOGICAL_XOR
%left T_LOGICAL_AND
%precedence T_PRINT
%precedence T_YIELD
%precedence T_DOUBLE_ARROW
%precedence T_YIELD_FROM
%precedence '=' T_PLUS_EQUAL T_MINUS_EQUAL T_MUL_EQUAL T_DIV_EQUAL T_CONCAT_EQUAL T_MOD_EQUAL T_AND_EQUAL T_OR_EQUAL T_XOR_EQUAL T_SL_EQUAL T_SR_EQUAL T_POW_EQUAL T_COALESCE_EQUAL
%left '?' ':'
%right T_COALESCE
%left T_BOOLEAN_OR
%left T_BOOLEAN_AND
%left '|'
%left '^'
%left T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG
%nonassoc T_IS_EQUAL T_IS_NOT_EQUAL T_IS_IDENTICAL T_IS_NOT_IDENTICAL T_SPACESHIP
%nonassoc '<' T_IS_SMALLER_OR_EQUAL '>' T_IS_GREATER_OR_EQUAL
%left '.'
%left T_SL T_SR
%left '+' '-'
%left '*' '/' '%'
%precedence '!'
%precedence T_INSTANCEOF
%precedence '~' T_INT_CAST T_DOUBLE_CAST T_STRING_CAST T_ARRAY_CAST T_OBJECT_CAST T_BOOL_CAST T_UNSET_CAST '@'
%right T_POW
%precedence T_CLONE
%precedence T_NOELSE
%precedence T_ELSEIF
%precedence T_ELSE
%token  T_LNUMBER   "integer"
%token  T_DNUMBER   "floating-point number"
%token  T_STRING    "identifier"
%token  T_NAME_FULLY_QUALIFIED "fully qualified name"
%token  T_NAME_RELATIVE "namespace-relative name"
%token  T_NAME_QUALIFIED "namespaced name"
%token  T_VARIABLE  "variable"
%token  T_INLINE_HTML
%token  T_ENCAPSED_AND_WHITESPACE  "string content"
%token  T_CONSTANT_ENCAPSED_STRING "quoted string"
%token  T_STRING_VARNAME "variable name"
%token  T_NUM_STRING "number"
%token  T_INCLUDE       "'include'"
%token  T_INCLUDE_ONCE  "'include_once'"
%token  T_EVAL          "'eval'"
%token  T_REQUIRE       "'require'"
%token  T_REQUIRE_ONCE  "'require_once'"
%token  T_LOGICAL_OR    "'or'"
%token  T_LOGICAL_XOR   "'xor'"
%token  T_LOGICAL_AND   "'and'"
%token  T_PRINT         "'print'"
%token  T_YIELD         "'yield'"
%token  T_YIELD_FROM    "'yield from'"
%token  T_INSTANCEOF    "'instanceof'"
%token  T_NEW           "'new'"
%token  T_CLONE         "'clone'"
%token  T_EXIT          "'exit'"
%token  T_IF            "'if'"
%token  T_ELSEIF        "'elseif'"
%token  T_ELSE          "'else'"
%token  T_ENDIF         "'endif'"
%token  T_ECHO          "'echo'"
%token  T_DO            "'do'"
%token  T_WHILE         "'while'"
%token  T_ENDWHILE      "'endwhile'"
%token  T_FOR           "'for'"
%token  T_ENDFOR        "'endfor'"
%token  T_FOREACH       "'foreach'"
%token  T_ENDFOREACH    "'endforeach'"
%token  T_DECLARE       "'declare'"
%token  T_ENDDECLARE    "'enddeclare'"
%token  T_AS            "'as'"
%token  T_SWITCH        "'switch'"
%token  T_ENDSWITCH     "'endswitch'"
%token  T_CASE          "'case'"
%token  T_DEFAULT       "'default'"
%token  T_MATCH         "'match'"
%token  T_BREAK         "'break'"
%token  T_CONTINUE      "'continue'"
%token  T_GOTO          "'goto'"
%token  T_FUNCTION      "'function'"
%token  T_FN            "'fn'"
%token  T_CONST         "'const'"
%token  T_RETURN        "'return'"
%token  T_TRY           "'try'"
%token  T_CATCH         "'catch'"
%token  T_FINALLY       "'finally'"
%token  T_THROW         "'throw'"
%token  T_USE           "'use'"
%token  T_INSTEADOF     "'insteadof'"
%token  T_GLOBAL        "'global'"
%token  T_STATIC        "'static'"
%token  T_ABSTRACT      "'abstract'"
%token  T_FINAL         "'final'"
%token  T_PRIVATE       "'private'"
%token  T_PROTECTED     "'protected'"
%token  T_PUBLIC        "'public'"
%token  T_READONLY      "'readonly'"
%token  T_VAR           "'var'"
%token  T_UNSET         "'unset'"
%token  T_ISSET         "'isset'"
%token  T_EMPTY         "'empty'"
%token  T_HALT_COMPILER "'__halt_compiler'"
%token  T_CLASS         "'class'"
%token  T_TRAIT         "'trait'"
%token  T_INTERFACE     "'interface'"
%token  T_ENUM          "'enum'"
%token  T_EXTENDS       "'extends'"
%token  T_IMPLEMENTS    "'implements'"
%token  T_NAMESPACE     "'namespace'"
%token  T_LIST            "'list'"
%token  T_ARRAY           "'array'"
%token  T_CALLABLE        "'callable'"
%token  T_LINE            "'__LINE__'"
%token  T_FILE            "'__FILE__'"
%token  T_DIR             "'__DIR__'"
%token  T_CLASS_C         "'__CLASS__'"
%token  T_TRAIT_C         "'__TRAIT__'"
%token  T_METHOD_C        "'__METHOD__'"
%token  T_FUNC_C          "'__FUNCTION__'"
%token  T_NS_C            "'__NAMESPACE__'"
%token END 0 "end of file"
%token T_ATTRIBUTE    "'#['"
%token T_PLUS_EQUAL   "'+='"
%token T_MINUS_EQUAL  "'-='"
%token T_MUL_EQUAL    "'*='"
%token T_DIV_EQUAL    "'/='"
%token T_CONCAT_EQUAL "'.='"
%token T_MOD_EQUAL    "'%='"
%token T_AND_EQUAL    "'&='"
%token T_OR_EQUAL     "'|='"
%token T_XOR_EQUAL    "'^='"
%token T_SL_EQUAL     "'<<='"
%token T_SR_EQUAL     "'>>='"
%token T_COALESCE_EQUAL "'??='"
%token T_BOOLEAN_OR   "'||'"
%token T_BOOLEAN_AND  "'&&'"
%token T_IS_EQUAL     "'=='"
%token T_IS_NOT_EQUAL "'!='"
%token T_IS_IDENTICAL "'==='"
%token T_IS_NOT_IDENTICAL "'!=='"
%token T_IS_SMALLER_OR_EQUAL "'<='"
%token T_IS_GREATER_OR_EQUAL "'>='"
%token T_SPACESHIP "'<=>'"
%token T_SL "'<<'"
%token T_SR "'>>'"
%token T_INC "'++'"
%token T_DEC "'--'"
%token T_INT_CAST    "'(int)'"
%token T_DOUBLE_CAST "'(double)'"
%token T_STRING_CAST "'(string)'"
%token T_ARRAY_CAST  "'(array)'"
%token T_OBJECT_CAST "'(object)'"
%token T_BOOL_CAST   "'(bool)'"
%token T_UNSET_CAST  "'(unset)'"
%token T_OBJECT_OPERATOR "'->'"
%token T_NULLSAFE_OBJECT_OPERATOR "'?->'"
%token T_DOUBLE_ARROW    "'=>'"
%token T_COMMENT         "comment"
%token T_DOC_COMMENT     "doc comment"
%token T_OPEN_TAG        "open tag"
%token T_OPEN_TAG_WITH_ECHO "'<?='"
%token T_CLOSE_TAG       "'?>'"
%token T_WHITESPACE      "whitespace"
%token T_START_HEREDOC   "heredoc start"
%token T_END_HEREDOC     "heredoc end"
%token T_DOLLAR_OPEN_CURLY_BRACES "'${'"
%token T_CURLY_OPEN      "'{$'"
%token T_PAAMAYIM_NEKUDOTAYIM "'::'"
%token T_NS_SEPARATOR    "'\\'"
%token T_ELLIPSIS        "'...'"
%token T_COALESCE        "'??'"
%token T_POW             "'**'"
%token T_POW_EQUAL       "'**='"
%token T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG     "'&'"
%token T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG "amp"
%token T_BAD_CHARACTER   "invalid character"
%token T_ERROR
%expect 0
%%
 

start:
	top_statement_list	
;

reserved_non_modifiers:
	  T_INCLUDE | T_INCLUDE_ONCE | T_EVAL | T_REQUIRE | T_REQUIRE_ONCE | T_LOGICAL_OR | T_LOGICAL_XOR | T_LOGICAL_AND
	| T_INSTANCEOF | T_NEW | T_CLONE | T_EXIT | T_IF | T_ELSEIF | T_ELSE | T_ENDIF | T_ECHO | T_DO | T_WHILE | T_ENDWHILE
	| T_FOR | T_ENDFOR | T_FOREACH | T_ENDFOREACH | T_DECLARE | T_ENDDECLARE | T_AS | T_TRY | T_CATCH | T_FINALLY
	| T_THROW | T_USE | T_INSTEADOF | T_GLOBAL | T_VAR | T_UNSET | T_ISSET | T_EMPTY | T_CONTINUE | T_GOTO
	| T_FUNCTION | T_CONST | T_RETURN | T_PRINT | T_YIELD | T_LIST | T_SWITCH | T_ENDSWITCH | T_CASE | T_DEFAULT | T_BREAK
	| T_ARRAY | T_CALLABLE | T_EXTENDS | T_IMPLEMENTS | T_NAMESPACE | T_TRAIT | T_INTERFACE | T_CLASS
	| T_CLASS_C | T_TRAIT_C | T_FUNC_C | T_METHOD_C | T_LINE | T_FILE | T_DIR | T_NS_C | T_FN | T_MATCH | T_ENUM
;

semi_reserved:
	  reserved_non_modifiers
	| T_STATIC | T_ABSTRACT | T_FINAL | T_PRIVATE | T_PROTECTED | T_PUBLIC | T_READONLY
;

ampersand:
		T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG
	|	T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG
;

identifier:
		T_STRING 
	| 	semi_reserved  
;

top_statement_list:
		top_statement_list top_statement 
	|	%empty 
;


namespace_declaration_name:
		identifier								
	|	T_NAME_QUALIFIED						
;


namespace_name:
		T_STRING								
	|	T_NAME_QUALIFIED						
;


legacy_namespace_name:
		namespace_name							
	|	T_NAME_FULLY_QUALIFIED					
;

name:
		T_STRING									
	|	T_NAME_QUALIFIED							
	|	T_NAME_FULLY_QUALIFIED						
	|	T_NAME_RELATIVE								
;

attribute_decl:
		class_name
			
	|	class_name argument_list
			
;

attribute_group:
		attribute_decl
			
	|	attribute_group ',' attribute_decl
			
;

attribute:
		T_ATTRIBUTE attribute_group possible_comma ']'	
;

attributes:
		attribute				
	|	attributes attribute	
;

attributed_statement:
		function_declaration_statement		
	|	class_declaration_statement			
	|	trait_declaration_statement			
	|	interface_declaration_statement		
	|	enum_declaration_statement			
;

top_statement:
		statement							
	|	attributed_statement					
	|	attributes attributed_statement		
	|	T_HALT_COMPILER '(' ')' ';'
			
	|	T_NAMESPACE namespace_declaration_name ';'
			
	|	T_NAMESPACE namespace_declaration_name 
		'{' top_statement_list '}'
			
	|	T_NAMESPACE 
		'{' top_statement_list '}'
			
	|	T_USE mixed_group_use_declaration ';'		
	|	T_USE use_type group_use_declaration ';'	
	|	T_USE use_declarations ';'					
	|	T_USE use_type use_declarations ';'			
	|	T_CONST const_list ';'						
;

use_type:
	 	T_FUNCTION 		
	| 	T_CONST 		
;

group_use_declaration:
		legacy_namespace_name T_NS_SEPARATOR '{' unprefixed_use_declarations possible_comma '}'
			
;

mixed_group_use_declaration:
		legacy_namespace_name T_NS_SEPARATOR '{' inline_use_declarations possible_comma '}'
			
;

possible_comma:
		%empty
	|	','
;

inline_use_declarations:
		inline_use_declarations ',' inline_use_declaration
			
	|	inline_use_declaration
			
;

unprefixed_use_declarations:
		unprefixed_use_declarations ',' unprefixed_use_declaration
			
	|	unprefixed_use_declaration
			
;

use_declarations:
		use_declarations ',' use_declaration
			
	|	use_declaration
			
;

inline_use_declaration:
		unprefixed_use_declaration 
	|	use_type unprefixed_use_declaration 
;

unprefixed_use_declaration:
		namespace_name
			
	|	namespace_name T_AS T_STRING
			
;

use_declaration:
		legacy_namespace_name
			
	|	legacy_namespace_name T_AS T_STRING
			
;

const_list:
		const_list ',' const_decl 
	|	const_decl 
;

inner_statement_list:
		inner_statement_list inner_statement
			
	|	%empty
			
;


inner_statement:
		statement 
	|	attributed_statement					
	|	attributes attributed_statement		
	|	T_HALT_COMPILER '(' ')' ';'
			
;


statement:
		'{' inner_statement_list '}' 
	|	if_stmt 
	|	alt_if_stmt 
	|	T_WHILE '(' expr ')' while_statement
			
	|	T_DO statement T_WHILE '(' expr ')' ';'
			
	|	T_FOR '(' for_exprs ';' for_exprs ';' for_exprs ')' for_statement
			
	|	T_SWITCH '(' expr ')' switch_case_list
			
	|	T_BREAK optional_expr ';'		
	|	T_CONTINUE optional_expr ';'	
	|	T_RETURN optional_expr ';'		
	|	T_GLOBAL global_var_list ';'	
	|	T_STATIC static_var_list ';'	
	|	T_ECHO echo_expr_list ';'		
	|	T_INLINE_HTML 
	|	expr ';' 
	|	T_UNSET '(' unset_variables possible_comma ')' ';' 
	|	T_FOREACH '(' expr T_AS foreach_variable ')' foreach_statement
			
	|	T_FOREACH '(' expr T_AS foreach_variable T_DOUBLE_ARROW foreach_variable ')'
		foreach_statement
			
	|	T_DECLARE '(' const_list ')'
			
		declare_statement
			
	|	';'	 
	|	T_TRY '{' inner_statement_list '}' catch_list finally_statement
			
	|	T_GOTO T_STRING ';' 
	|	T_STRING ':' 
;

catch_list:
		%empty
			
	|	catch_list T_CATCH '(' catch_name_list optional_variable ')' '{' inner_statement_list '}'
			
;

catch_name_list:
		class_name 
	|	catch_name_list '|' class_name 
;

optional_variable:
		%empty 
	|	T_VARIABLE 
;

finally_statement:
		%empty 
	|	T_FINALLY '{' inner_statement_list '}' 
;

unset_variables:
		unset_variable 
	|	unset_variables ',' unset_variable 
;

unset_variable:
		variable 
;

function_name:
		T_STRING 
	|	T_READONLY 
;

function_declaration_statement:
	function returns_ref function_name backup_doc_comment '(' parameter_list ')' return_type
	backup_fn_flags '{' inner_statement_list '}' backup_fn_flags
		
;

is_reference:
		%empty	
	|	T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG	
;

is_variadic:
		%empty 
	|	T_ELLIPSIS  
;

class_declaration_statement:
		class_modifiers T_CLASS 
		T_STRING extends_from implements_list backup_doc_comment '{' class_statement_list '}'
			
	|	T_CLASS 
		T_STRING extends_from implements_list backup_doc_comment '{' class_statement_list '}'
			
;

class_modifiers:
		class_modifier 					
	|	class_modifiers class_modifier
			
;

anonymous_class_modifiers:
		class_modifier
			
	|	anonymous_class_modifiers class_modifier
			
;

anonymous_class_modifiers_optional:
		%empty				
	|	anonymous_class_modifiers	
;

class_modifier:
		T_ABSTRACT 		
	|	T_FINAL 		
	|	T_READONLY 		
;

trait_declaration_statement:
		T_TRAIT 
		T_STRING backup_doc_comment '{' class_statement_list '}'
			
;

interface_declaration_statement:
		T_INTERFACE 
		T_STRING interface_extends_list backup_doc_comment '{' class_statement_list '}'
			
;

enum_declaration_statement:
		T_ENUM 
		T_STRING enum_backing_type implements_list backup_doc_comment '{' class_statement_list '}'
			
;

enum_backing_type:
		%empty	
	|	':' type_expr 
;

enum_case:
		T_CASE backup_doc_comment identifier enum_case_expr ';'
			
;

enum_case_expr:
		%empty	
	|	'=' expr 
;

extends_from:
		%empty				
	|	T_EXTENDS class_name	
;

interface_extends_list:
		%empty			        
	|	T_EXTENDS class_name_list	
;

implements_list:
		%empty		        		
	|	T_IMPLEMENTS class_name_list	
;

foreach_variable:
		variable			
	|	ampersand variable	
	|	T_LIST '(' array_pair_list ')' 
	|	'[' array_pair_list ']' 
;

for_statement:
		statement 
	|	':' inner_statement_list T_ENDFOR ';' 
;

foreach_statement:
		statement 
	|	':' inner_statement_list T_ENDFOREACH ';' 
;

declare_statement:
		statement 
	|	':' inner_statement_list T_ENDDECLARE ';' 
;

switch_case_list:
		'{' case_list '}'					
	|	'{' ';' case_list '}'				
	|	':' case_list T_ENDSWITCH ';'		
	|	':' ';' case_list T_ENDSWITCH ';'	
;

case_list:
		%empty 
	|	case_list T_CASE expr case_separator inner_statement_list
			
	|	case_list T_DEFAULT case_separator inner_statement_list
			
;

case_separator:
		':'
	|	';'
;


match:
		T_MATCH '(' expr ')' '{' match_arm_list '}'
			;
;

match_arm_list:
		%empty 
	|	non_empty_match_arm_list possible_comma 
;

non_empty_match_arm_list:
		match_arm 
	|	non_empty_match_arm_list ',' match_arm 
;

match_arm:
		match_arm_cond_list possible_comma T_DOUBLE_ARROW expr
			
	|	T_DEFAULT possible_comma T_DOUBLE_ARROW expr
			
;

match_arm_cond_list:
		expr 
	|	match_arm_cond_list ',' expr 
;


while_statement:
		statement 
	|	':' inner_statement_list T_ENDWHILE ';' 
;


if_stmt_without_else:
		T_IF '(' expr ')' statement
			
	|	if_stmt_without_else T_ELSEIF '(' expr ')' statement
			
;

if_stmt:
		if_stmt_without_else %prec T_NOELSE 
	|	if_stmt_without_else T_ELSE statement
			
;

alt_if_stmt_without_else:
		T_IF '(' expr ')' ':' inner_statement_list
			
	|	alt_if_stmt_without_else T_ELSEIF '(' expr ')' ':' inner_statement_list
			
;

alt_if_stmt:
		alt_if_stmt_without_else T_ENDIF ';' 
	|	alt_if_stmt_without_else T_ELSE ':' inner_statement_list T_ENDIF ';'
			
;

parameter_list:
		non_empty_parameter_list possible_comma 
	|	%empty	
;


non_empty_parameter_list:
		attributed_parameter
			
	|	non_empty_parameter_list ',' attributed_parameter
			
;

attributed_parameter:
		attributes parameter	
	|	parameter				
;

optional_cpp_modifiers:
		%empty
			
	|	non_empty_member_modifiers
			
;

parameter:
		optional_cpp_modifiers optional_type_without_static
		is_reference is_variadic T_VARIABLE backup_doc_comment
			
	|	optional_cpp_modifiers optional_type_without_static
		is_reference is_variadic T_VARIABLE backup_doc_comment '=' expr
			
;

optional_type_without_static:
		%empty	
	|	type_expr_without_static	
;

type_expr:
		type				
	|	'?' type			
	|	union_type			
	|	intersection_type	
;

type:
		type_without_static	
	|	T_STATIC			
;

union_type_element:
                type 
        |        '(' intersection_type ')' 
;

union_type:
		union_type_element '|' union_type_element
			
	|	union_type '|' union_type_element
			
;

intersection_type:
		type T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG type       
	|	intersection_type T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG type 
;



type_expr_without_static:
		type_without_static			
	|	'?' type_without_static		
	|	union_type_without_static	
	|	intersection_type_without_static	
;

type_without_static:
		T_ARRAY		
	|	T_CALLABLE	
	|	name		
;

union_type_without_static_element:
                type_without_static 
        |        '(' intersection_type_without_static ')' 
;

union_type_without_static:
		union_type_without_static_element '|' union_type_without_static_element
			
	|	union_type_without_static '|' union_type_without_static_element
			
;

intersection_type_without_static:
		type_without_static T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG type_without_static
			
	|	intersection_type_without_static T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG type_without_static
			
;

return_type:
		%empty	
	|	':' type_expr	
;

argument_list:
		'(' ')'	
	|	'(' non_empty_argument_list possible_comma ')' 
	|	'(' T_ELLIPSIS ')' 
;

non_empty_argument_list:
		argument
			
	|	non_empty_argument_list ',' argument
			
;

argument:
		expr				
	|	identifier ':' expr
			
	|	T_ELLIPSIS expr	
;

global_var_list:
		global_var_list ',' global_var 
	|	global_var 
;

global_var:
	simple_variable
		
;


static_var_list:
		static_var_list ',' static_var 
	|	static_var 
;

static_var:
		T_VARIABLE			
	|	T_VARIABLE '=' expr	
;

class_statement_list:
		class_statement_list class_statement
			
	|	%empty
			
;


attributed_class_statement:
		property_modifiers optional_type_without_static property_list ';'
			
	|	class_const_modifiers T_CONST class_const_list ';'
			
	|	class_const_modifiers T_CONST type_expr class_const_list ';'
			
	|	method_modifiers function returns_ref identifier backup_doc_comment '(' parameter_list ')'
		return_type backup_fn_flags method_body backup_fn_flags
			
	|	enum_case 
;

class_statement:
		attributed_class_statement 
	|	attributes attributed_class_statement 
	|	T_USE class_name_list trait_adaptations
			
;

class_name_list:
		class_name 
	|	class_name_list ',' class_name 
;

trait_adaptations:
		';'								
	|	'{' '}'							
	|	'{' trait_adaptation_list '}'	
;

trait_adaptation_list:
		trait_adaptation
			
	|	trait_adaptation_list trait_adaptation
			
;

trait_adaptation:
		trait_precedence ';'	
	|	trait_alias ';'			
;

trait_precedence:
	absolute_trait_method_reference T_INSTEADOF class_name_list
		
;

trait_alias:
		trait_method_reference T_AS T_STRING
			
	|	trait_method_reference T_AS reserved_non_modifiers
			
	|	trait_method_reference T_AS member_modifier identifier
			
	|	trait_method_reference T_AS member_modifier
			
;

trait_method_reference:
		identifier
			
	|	absolute_trait_method_reference 
;

absolute_trait_method_reference:
	class_name T_PAAMAYIM_NEKUDOTAYIM identifier
		
;

method_body:
		';' 		
	|	'{' inner_statement_list '}'	
;

property_modifiers:
		non_empty_member_modifiers
			
	|	T_VAR
			
;

method_modifiers:
		%empty
			
	|	non_empty_member_modifiers
			
;

class_const_modifiers:
		%empty
			
	|	non_empty_member_modifiers
			
;

non_empty_member_modifiers:
		member_modifier
			
	|	non_empty_member_modifiers member_modifier
			
;

member_modifier:
		T_PUBLIC				
	|	T_PROTECTED				
	|	T_PRIVATE				
	|	T_STATIC				
	|	T_ABSTRACT				
	|	T_FINAL					
	|	T_READONLY				
;

property_list:
		property_list ',' property 
	|	property 
;

property:
		T_VARIABLE backup_doc_comment
			
	|	T_VARIABLE '=' expr backup_doc_comment
			
;

class_const_list:
		class_const_list ',' class_const_decl 
	|	class_const_decl 
;

class_const_decl:
		T_STRING '=' expr backup_doc_comment 
	|	semi_reserved '=' expr backup_doc_comment 
;

const_decl:
	T_STRING '=' expr backup_doc_comment 
;

echo_expr_list:
		echo_expr_list ',' echo_expr 
	|	echo_expr 
;
echo_expr:
	expr 
;

for_exprs:
		%empty			
	|	non_empty_for_exprs	
;

non_empty_for_exprs:
		non_empty_for_exprs ',' expr 
	|	expr 
;

anonymous_class:
		anonymous_class_modifiers_optional T_CLASS  ctor_arguments
		extends_from implements_list backup_doc_comment '{' class_statement_list '}' 
;

new_expr:
		T_NEW class_name_reference ctor_arguments
			
	|	T_NEW anonymous_class
			
	|	T_NEW attributes anonymous_class
			
;

expr:
		variable
			
	|	T_LIST '(' array_pair_list ')' '=' expr
			
	|	'[' array_pair_list ']' '=' expr
			
	|	variable '=' expr
			
	|	variable '=' ampersand variable
			
	|	T_CLONE expr 
	|	variable T_PLUS_EQUAL expr
			
	|	variable T_MINUS_EQUAL expr
			
	|	variable T_MUL_EQUAL expr
			
	|	variable T_POW_EQUAL expr
			
	|	variable T_DIV_EQUAL expr
			
	|	variable T_CONCAT_EQUAL expr
			
	|	variable T_MOD_EQUAL expr
			
	|	variable T_AND_EQUAL expr
			
	|	variable T_OR_EQUAL expr
			
	|	variable T_XOR_EQUAL expr
			
	|	variable T_SL_EQUAL expr
			
	|	variable T_SR_EQUAL expr
			
	|	variable T_COALESCE_EQUAL expr
			
	|	variable T_INC 
	|	T_INC variable 
	|	variable T_DEC 
	|	T_DEC variable 
	|	expr T_BOOLEAN_OR expr
			
	|	expr T_BOOLEAN_AND expr
			
	|	expr T_LOGICAL_OR expr
			
	|	expr T_LOGICAL_AND expr
			
	|	expr T_LOGICAL_XOR expr
			
	|	expr '|' expr	
	|	expr T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG expr	
	|	expr T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG expr	
	|	expr '^' expr	
	|	expr '.' expr 	
	|	expr '+' expr 	
	|	expr '-' expr 	
	|	expr '*' expr	
	|	expr T_POW expr	
	|	expr '/' expr	
	|	expr '%' expr 	
	| 	expr T_SL expr	
	|	expr T_SR expr	
	|	'+' expr %prec '~' 
	|	'-' expr %prec '~' 
	|	'!' expr 
	|	'~' expr 
	|	expr T_IS_IDENTICAL expr
			
	|	expr T_IS_NOT_IDENTICAL expr
			
	|	expr T_IS_EQUAL expr
			
	|	expr T_IS_NOT_EQUAL expr
			
	|	expr '<' expr
			
	|	expr T_IS_SMALLER_OR_EQUAL expr
			
	|	expr '>' expr
			
	|	expr T_IS_GREATER_OR_EQUAL expr
			
	|	expr T_SPACESHIP expr
			
	|	expr T_INSTANCEOF class_name_reference
			
	|	'(' expr ')' 
	|	new_expr 
	|	expr '?' expr ':' expr
			
	|	expr '?' ':' expr
			
	|	expr T_COALESCE expr
			
	|	internal_functions_in_yacc 
	|	T_INT_CAST expr		
	|	T_DOUBLE_CAST expr	
	|	T_STRING_CAST expr	
	|	T_ARRAY_CAST expr	
	|	T_OBJECT_CAST expr	
	|	T_BOOL_CAST expr	
	|	T_UNSET_CAST expr	
	|	T_EXIT exit_expr	
	|	'@' expr			
	|	scalar 
	|	'`' backticks_expr '`' 
	|	T_PRINT expr 
	|	T_YIELD 
	|	T_YIELD expr 
	|	T_YIELD expr T_DOUBLE_ARROW expr 
	|	T_YIELD_FROM expr 
	|	T_THROW expr 
	|	inline_function 
	|	attributes inline_function 
	|	T_STATIC inline_function 
	|	attributes T_STATIC inline_function
			
	|	match 
;


inline_function:
		function returns_ref backup_doc_comment '(' parameter_list ')' lexical_vars return_type
		backup_fn_flags '{' inner_statement_list '}' backup_fn_flags
			
	|	fn returns_ref backup_doc_comment '(' parameter_list ')' return_type
		T_DOUBLE_ARROW backup_fn_flags backup_lex_pos expr backup_fn_flags
			
;

fn:
	T_FN 
;

function:
	T_FUNCTION 
;

backup_doc_comment:
	%empty 
;

backup_fn_flags:
	%prec PREC_ARROW_FUNCTION %empty 
;

backup_lex_pos:
	%empty 
;

returns_ref:
		%empty	
	|	ampersand	
;

lexical_vars:
		%empty 
	|	T_USE '(' lexical_var_list possible_comma ')' 
;

lexical_var_list:
		lexical_var_list ',' lexical_var 
	|	lexical_var 
;

lexical_var:
		T_VARIABLE		
	|	ampersand T_VARIABLE	
;

function_call:
		name argument_list
			
	|	T_READONLY argument_list 
	|	class_name T_PAAMAYIM_NEKUDOTAYIM member_name argument_list
			
	|	variable_class_name T_PAAMAYIM_NEKUDOTAYIM member_name argument_list
			
	|	callable_expr  argument_list 
;

class_name:
		T_STATIC
			
	|	name 
;

class_name_reference:
		class_name		
	|	new_variable	
	|	'(' expr ')'	
;

exit_expr:
		%empty				
	|	'(' optional_expr ')'	
;

backticks_expr:
		%empty
			
	|	T_ENCAPSED_AND_WHITESPACE 
	|	encaps_list 
;


ctor_arguments:
		%empty	
	|	argument_list 
;


dereferenceable_scalar:
		T_ARRAY '(' array_pair_list ')'	
	|	'[' array_pair_list ']'			
	|	T_CONSTANT_ENCAPSED_STRING		
	|	'"' encaps_list '"'			 	
;

scalar:
		T_LNUMBER 	
	|	T_DNUMBER 	
	|	T_START_HEREDOC T_ENCAPSED_AND_WHITESPACE T_END_HEREDOC 
	|	T_START_HEREDOC T_END_HEREDOC
			
	|	T_START_HEREDOC encaps_list T_END_HEREDOC 
	|	dereferenceable_scalar	
	|	constant				
	|	class_constant			
;

constant:
		name		
	|	T_LINE 		
	|	T_FILE 		
	|	T_DIR   	
	|	T_TRAIT_C	
	|	T_METHOD_C	
	|	T_FUNC_C	
	|	T_NS_C		
	|	T_CLASS_C	
;

class_constant:
		class_name T_PAAMAYIM_NEKUDOTAYIM identifier
			
	|	variable_class_name T_PAAMAYIM_NEKUDOTAYIM identifier
			
	|	class_name T_PAAMAYIM_NEKUDOTAYIM '{' expr '}'
			
	|	variable_class_name T_PAAMAYIM_NEKUDOTAYIM '{' expr '}'
			
;

optional_expr:
		%empty	
	|	expr		
;

variable_class_name:
		fully_dereferenceable 
;

fully_dereferenceable:
		variable				
	|	'(' expr ')'			
	|	dereferenceable_scalar	
	|	class_constant			
;

array_object_dereferenceable:
		fully_dereferenceable	
	|	constant				
;

callable_expr:
		callable_variable		
	|	'(' expr ')'			
	|	dereferenceable_scalar	
;

callable_variable:
		simple_variable
			
	|	array_object_dereferenceable '[' optional_expr ']'
			
	|	array_object_dereferenceable '{' expr '}'
			
	|	array_object_dereferenceable T_OBJECT_OPERATOR property_name argument_list
			
	|	array_object_dereferenceable T_NULLSAFE_OBJECT_OPERATOR property_name argument_list
			
	|	function_call 
;

variable:
		callable_variable
			
	|	static_member
			
	|	array_object_dereferenceable T_OBJECT_OPERATOR property_name
			
	|	array_object_dereferenceable T_NULLSAFE_OBJECT_OPERATOR property_name
			
;

simple_variable:
		T_VARIABLE			
	|	'$' '{' expr '}'	
	|	'$' simple_variable	
;

static_member:
		class_name T_PAAMAYIM_NEKUDOTAYIM simple_variable
			
	|	variable_class_name T_PAAMAYIM_NEKUDOTAYIM simple_variable
			
;

new_variable:
		simple_variable
			
	|	new_variable '[' optional_expr ']'
			
	|	new_variable '{' expr '}'
			
	|	new_variable T_OBJECT_OPERATOR property_name
			
	|	new_variable T_NULLSAFE_OBJECT_OPERATOR property_name
			
	|	class_name T_PAAMAYIM_NEKUDOTAYIM simple_variable
			
	|	new_variable T_PAAMAYIM_NEKUDOTAYIM simple_variable
			
;

member_name:
		identifier 
	|	'{' expr '}'	
	|	simple_variable	
;

property_name:
		T_STRING 
	|	'{' expr '}'	
	|	simple_variable	
;

array_pair_list:
		non_empty_array_pair_list
			
;

possible_array_pair:
		%empty 
	|	array_pair  
;

non_empty_array_pair_list:
		non_empty_array_pair_list ',' possible_array_pair
			
	|	possible_array_pair
			
;

array_pair:
		expr T_DOUBLE_ARROW expr
			
	|	expr
			
	|	expr T_DOUBLE_ARROW ampersand variable
			
	|	ampersand variable
			
	|	T_ELLIPSIS expr
			
	|	expr T_DOUBLE_ARROW T_LIST '(' array_pair_list ')'
			
	|	T_LIST '(' array_pair_list ')'
			
;

encaps_list:
		encaps_list encaps_var
			
	|	encaps_list T_ENCAPSED_AND_WHITESPACE
			
	|	encaps_var
			
	|	T_ENCAPSED_AND_WHITESPACE encaps_var
			
;

encaps_var:
		T_VARIABLE
			
	|	T_VARIABLE '[' encaps_var_offset ']'
			
	|	T_VARIABLE T_OBJECT_OPERATOR T_STRING
			
	|	T_VARIABLE T_NULLSAFE_OBJECT_OPERATOR T_STRING
			
	|	T_DOLLAR_OPEN_CURLY_BRACES expr '}'
			
	|	T_DOLLAR_OPEN_CURLY_BRACES T_STRING_VARNAME '}'
			
	|	T_DOLLAR_OPEN_CURLY_BRACES T_STRING_VARNAME '[' expr ']' '}'
			
	|	T_CURLY_OPEN variable '}' 
;

encaps_var_offset:
		T_STRING			
	|	T_NUM_STRING		
	|	'-' T_NUM_STRING 	
	|	T_VARIABLE			
;


internal_functions_in_yacc:
		T_ISSET '(' isset_variables possible_comma ')' 
	|	T_EMPTY '(' expr ')' 
	|	T_INCLUDE expr
			
	|	T_INCLUDE_ONCE expr
			
	|	T_EVAL '(' expr ')'
			
	|	T_REQUIRE expr
			
	|	T_REQUIRE_ONCE expr
			
;

isset_variables:
		isset_variable 
	|	isset_variables ',' isset_variable
			
;

isset_variable:
		expr 
;


%%

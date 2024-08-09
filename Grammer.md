```EBNF
	<program> ::= { <satement> <end> | <comment> }

	<end> ::= "\n"
	<comma> ::= "."
	<letter> ::= "a" | "b" | "c" | ... | "X" | "Y" | "Z"
	<digit> ::= "0" | "1" | ... | "8" | "9"
	<number> ::= <digit> { <digit> }
	<character> ::= <letter> | <digit>
	<text> ::= { <character> }

	<string> ::= """ <text> """
	<char> ::= "'" <character> "'"
	<integer> ::= [ "-" ] { <digit> }
	<boolean> ::= "True" | "False"
	<list> ::= "[" <type_expression> { "," <type_expression> } "]"
	<dictionary> ::= <identifier> ":" <type_expression> { "," <identifier> ":" <type_expression>} <comma>
	<float> ::= <number> "." <number>
	<version> ::= <number>"." <number> "." <number>

	<type_expression> ::= <string>
											| <char>
											| <integer>
											| <boolean>
											| <list>
											| <dictionary>
											| <float>
											| <version>

	<comment> ::= "//" <text> <end>
							| "/*" <text> "*/" <end>

	<satement> ::=

	<variable_declaration> ::= [ "const" ] [ "$" ] <identifier> [ <type_declaration> ] "=" <expression> <end>
	<identifier> ::= <letter> { <letter> | <digit> | "_" }
	<type_declaration> ::= ":" <type_name>

	<type_name> ::= ( "String" | "Str" ) |
									( "Int" | "Integer" ) |
									( "Char" | "Character" ) |
									( "Bool" | "Boolean" ) |
									"List[" <type_name> "]" |
									( "Dictionary" | "Dict" [ "{" <type_name> "}" ] ) |
									"Float" |
									( "Version" | "Ver" ) |
									"Char[" <digit> "]"

	<expression> ::= <type_expression>
								| <arithmetic_expression>
								| <boolean_expression>
								| <identifier>

	<arithmetic_expression> ::= <term> {("+" | "-") <term>}
	<term> ::= <factor> {("*" | "/" | "%" | "//" | "^") <factor>}
	<factor> ::= <digit> | <identifier> | "(" <arithmetic_expression> ")"

	<boolean_expression> ::= <comparison> { ("&&" | "||") <comparison> }
	<comparison> ::= <expression> ("==" | "!=" | "<" | ">" | "<=" | ">=") <expression>

	<function_call> ::= <identifier> { <expression> }
```

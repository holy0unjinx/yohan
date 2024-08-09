```EBNF
	<program> ::= { <satement> }

	<end> ::= "\n"
	<alphabet_lower> ::= "a" | "b" | "c" | ... | "x" | "y" | "z"
	<alphabet_upper> ::= "A" | "B" | "C" | ... | "X" | "Y" | "Z"
	<character> ::= "." | "가" | "a" | "1" | ... | "あ" | "Ā" | "啊"
	<digit> ::= "0" | "1" | "2" | ... | "7" | "8" | "9"
	<number> ::= <digit> { <digit> }
	<float> ::= <number> "." <number>
	<alphabet> ::= <alphabet_lower> | <alphabet_upper>
	<text> ::= <character> { <character> }
	<identifier> ::= <alphabet> { <alphabet> | <number> | "_" }

	<satement> ::= <comment> | <variable_declaration> | <function_declaration> | <control_structure>

	<comment> ::= "//" <text> <end> | "/*" <text> "*/"

	<variable_declaration> ::= [ "const" ] [ "$" ] <identifier> [ <type_declaration> ] "=" <expression> <end>
	<function_declaration> ::= [ "private" ] "func" [ <parameter_list> ] [ <type_declaration> ] <end> <body>

	<parameter_list> ::= <param> { <param> }
	<param> ::= <identifier> [ <type_declaration> ]

	<body> ::= { <satement> <end> } <satement> "." <end>

	<type_declaration> ::= ":" <data_type>

	<control_structure> ::= <if_satement> | <for_satement> | <while_satement>

	<if_satement> ::= "if" <condition> <end> <body> { "elif" <condition> <end> <body> } [ "else" <end> <body> ] | "if" <expression> <end> <case> { <case> } <default_case> <end>

	<case> ::= "is" <expression> <end> <body>
	<default_case> ::= "is not all" <end> <body>

	<for_satement> ::= "for" <identifier> "in" <identifier> <end> <body> | "for" <identifier> "=" <expression> "to" <expression> "step" <expression> <end> <body>

	<while_satement> ::= "while" <condition> <end> <body> | "do" <end> <body> <end> "while" <condition>

	<expression> ::= <arithmetic_expression> | <boolean_expression>

	<arithmetic_expression> ::= <term> { ( "+" | "-" ) <term> }
	<term> ::= <factor> { ( "*" | "/" | "%" | "//" | "^" ) <factor> }
	<factor> ::= <number> | <float> | <identifier> | "(" <arithmetic_expression> ")"

	<boolean_expression> ::= <comparison> { ( "&&" | "||" | "and" | "or") <comparison> }
	<comparison> ::= <expression> ( "==" | "!=" | "<" | ">" | "<=" | ">=" | "is" ) <expression> | ( "!" | "not" ) <expression>

	<function_call> ::= <identifier> <argument_list>
	<argument_list> ::= <expression> { "," <expression> }

	<data_type> ::= "int" | "float" | "char" | "string" | "list" | "boolean" | "dictionary" | "tuple" | "version"
```

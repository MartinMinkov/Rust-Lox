# Rust Lox

The Lox language based on "Crafting Interpreters" in Rust.

Find and support the book here:
[Crafting Interpreters](https://craftinginterpreters.com)

# Grammar

```
expression		-> comma;
comma 			-> expression ( "," expression )*;
ternary 		-> equality ( "?" expression ":" ternary )?
equality		-> comparison ( ( "!=" | "==" ) comparison )*;
comparison		-> term ( ( ">" | ">=" | "<" | ">=" ) term)*;
term			-> factor ( ( "-" | "+" ) factor )*;
factor			-> unary ( ( "/" | "*" ) unary )*;
unary 			-> ( ("!" | "-" ) unary | primary );
primary 		-> ( NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" );
```

# Added Features from Challenges

These are features that are offered as challanges by the book at the end of each chapter.

- Multi line comments
- Ternary operator
- Comma operator

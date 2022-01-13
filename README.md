# Rust Lox

The Lox language based on "Crafting Interpreters" in Rust.

Find and support the book here:
[Crafting Interpreters](https://craftinginterpreters.com)

# Grammar

```
program			-> declaration* EOF ;

declaration		-> varDecl | statement ;

varDecl			-> "var" IDENTIFIER ( "=" expression )? ";" ;

statement 		-> exprStmt | printStmt | block ;

block				-> "{" declaration* "}" ;

exprStmt		-> expression ";" ;
printStmt		-> "print" expression ";" ;

expression		-> comma;
comma 			-> assigment ( "," assigment )* ;
assigment			-> "IDENTIFIER" "=" assigment | equality | ternary ;
ternary 		-> equality ( "?" expression ":" ternary )?
equality		-> comparison ( ( "!=" | "==" ) comparison )* ;
comparison		-> term ( ( ">" | ">=" | "<" | ">=" ) term)* ;
term			-> factor ( ( "-" | "+" ) factor )* ;
factor			-> unary ( ( "/" | "*" ) unary )* ;
unary 			-> ( ("!" | "-" ) unary | primary ) ;
primary 		-> ( NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ) ;
```

# Added Features from Challenges

These are features that are offered as challanges by the book at the end of each chapter.

- Multi line comments
- Ternary operator
- Comma operator

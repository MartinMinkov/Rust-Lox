# Rust Lox

The Lox language based on "Crafting Interpreters" in Rust.

Find and support the book here:
[Crafting Interpreters](https://craftinginterpreters.com)

# Grammar

```
program			-> declaration* EOF ;

declaration		-> funDecl | varDecl | statement ;

varDecl			-> "var" IDENTIFIER ( "=" expression )? ";" ;
funDecl		-> "fun" function ;
function		-> IDENTIFIER "(" parameters? ")" block ;
parameters		-> IDENTIFIER ( "," IDENTIFIER)* ;

statement 		-> exprStmt | funExpr | ifStmt | printStmt | whileStmt | forStmt | returnStmt | block ;

block			-> "{" declaration* "}" ;

exprStmt		-> expression ";" ;
printStmt		-> "print" expression ";" ;
ifStmt			-> "if" "(" expression ")" statement ("else" statement )? ;
whileStmt		-> "while" "(" expression ")" statement ;
forStmt			-> "for" "(" (varDecl | expression | ";" ) expression? ";" expression? ")" statement ;
returnStmt		-> "return" expression? ";" ;

expression		-> comma;
comma 			-> assigment ( "," assigment )* ;
assigment		-> "IDENTIFIER" "=" assigment | ternary | logic_or ;
logic_or		-> logic_and ( "or" logic_and )* ;
logic_and		-> equality ( "and" equality)* ;
ternary 		-> equality ( "?" expression ":" ternary )?
equality		-> comparison ( ( "!=" | "==" ) comparison )* ;
comparison		-> term ( ( ">" | ">=" | "<" | ">=" ) term)* ;
term			-> factor ( ( "-" | "+" ) factor )* ;
factor			-> unary ( ( "/" | "*" ) unary )* ;
unary 			-> ( ("!" | "-" ) unary | call ) ;
call			-> ( "(" arguments? ")" )* ;
arguments		-> expression ( "," expression *) ;

funExpr			-> "fun" "(" parameters ")" block ;
primary 		-> ( NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER | funExpr ) ;
```

# Added Features from Challenges

These are features that are offered as challanges by the book at the end of each chapter.

- Multi line comments
- Ternary operator
- Comma operator

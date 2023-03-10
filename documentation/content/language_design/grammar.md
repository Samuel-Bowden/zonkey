---
title: "Grammar"
weight: 1
---

This is the current grammar that I'm implementing in the parser. It uses the notation defined in Crafting Interpreters.

```grammar
# Entry
program -> (function | start | class)*;

# Current data types
data_type -> "Integer" | "String" | "Float" | "Boolean" | IDENTIFIER;

# Definitions
start -> "start" block;

function -> "function" IDENTIFIER "(" parameters? ")" return_type block;
parameters -> data_type IDENTIFIER ("," data_type IDENTIFIER)*;
return_type -> ("->" data_type)?;

class -> "class" IDENTIFIER "{" property* method*"}";
property -> data_type IDENTIFIER ";";
method -> IDENTIFIER "(" parameters? ")" return_type block;

# Statements
statement -> terminated_statement | block | if_statement | while_statement | loop_statement | for_statement;
terminated_statement -> (expression_statement | return_statement | variable_init | "break" | "continue") ";";
expression_statement -> (IDENTIFIER ("=" | "+=" | "-=" | "/=" | "*="))? expression;
variable_init = "let" IDENTIFIER "=" (expression | new IDENTIFIER); 
if_statement -> "if" "(" expression ")" block ("else" block)?; 
loop_statement -> "loop" block;
while_statement -> "while" "(" expression ")" block;
for_statement -> "for" "(" variable_init "," expression "," expression_statement ")" block;
block -> "{" statement* "}"
return_statement -> "return" expression?;

# Expressions
expression -> cast;
cast -> data_type? and;
or -> and ("|" and)*;
and -> equality ("&" equality)*;
equality -> comparision (("==" | "!=") comparision)*;
comparision -> addsub ((">=" | "<=" | "<" | ">") addsub)*;
addsub -> multdiv (("-" | "+") multdiv)*;
multdiv -> literal (("/" | "*") literal)*;
unary -> ("-" | "!") unary | literal;
literal -> INTEGER | FLOAT | STRING | BOOLEAN | IDENTIFIER (. IDENTIFIER)* | call | "(" expression ")";
call -> IDENTIFIER (. IDENTIFIER)* "(" expression ("," expression)* ")";
```

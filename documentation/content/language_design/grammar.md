---
title: "Grammar"
weight: 1
---

This is the current grammar that I'm implementing in the parser, which is a subset of the language. It uses the notation defined in Crafting Interpreters.

```grammar
program -> global_declaration*;
global_declaration -> function_declaration | start_declaration | structure_declaration;
local_declaration -> terminated_variable_declaration | statement;
statement -> terminated_statement | block | if_statement | while_statement | loop_statement | for_statement;
terminated_statement -> (expression_statement | return_statement | "break" | "continue") ";";
expression_statement -> (IDENTIFIER ("=" | "+=" | "-=" | "/=" | "*="))? equality;
variable_declaration = "let" IDENTIFIER "=" (expression | new IDENTIFIER); 
terminated_variable_declaration = variable_declaration ";";
expression -> cast;
cast -> data_type? and;
or -> and ("|" and)*;
and -> equality ("&" equality)*;
equality -> comparision (("==" | "!=") comparision)*;
comparision -> addsub ((">=" | "<=" | "<" | ">") addsub)*;
addsub -> multdiv (("-" | "+") multdiv)*;
multdiv -> literal (("/" | "*") literal)*;
unary -> ("-" | "!") unary | literal;
literal -> INTEGER | FLOAT | STRING | BOOLEAN | IDENTIFIER (. IDENTIFIER)* | call | "(" equality ")";
call -> (IDENTIFIER ":")? IDENTIFIER "(" equality ("," equality)* ")";
block -> "{" local_declaration* "}"
if_statement -> "if" "(" equality ")" block ("else" block)?; 
while_statement -> "while" "(" equality ")" block;
loop_statement -> "loop" block;
for_statement -> "for" "(" variable_declaration "," equality "," expression_statement ")" block;
function_declaration -> "function" IDENTIFIER "(" parameters? ")" ("->" data_type)? block;
parameters -> data_type IDENTIFIER ("," data_type IDENTIFIER)*;
data_type -> "Integer" | "String" | "Float" | "Boolean";
start_declaration -> "start" block;
return_statement -> "return" expression?;
structure_declaration -> "class" IDENTIFIER class_block;
class_block -> "{" property* "}";
property -> data_type IDENTIFIER ";";
```

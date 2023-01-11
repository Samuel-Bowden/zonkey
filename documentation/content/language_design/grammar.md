---
title: "Grammar"
weight: 16
---

This is the current grammar that I'm implementing in the parser, which is a subset of the language. It uses the notation defined in Crafting Interpreters.

Print will eventually be implemented as a function but for now I will be implementing it as part of the language grammar.

Grammar
```grammar
program -> declaration*;
declaration -> terminated_variable_declaration | statement;
statement -> terminated_statement | block | if_statement | while_statement | loop_statement | for_statement;
terminated_statement -> (expression_statement | print_statement | exit_statement | "break" | "continue") ";";
expression_statement -> (IDENTIFIER ("=" | "+=" | "-=" | "/=" | "*="))? equality;
print_statement -> print "(" equality ")";
exit_statement -> exit "(" ")";
variable_declaration = ("Integer" | "String" | "Float" | "Boolean") IDENTIFIER "=" expression; 
terminated_variable_declaration = variable_declaration ";";
equality -> comparision (("==" | "!=") comparision)*;
comparision -> addsub ((">=" | "<=" | "<" | ">") addsub)*;
addsub -> multdiv (("-" | "+") multdiv)*;
multdiv -> literal (("/" | "*") literal)*;
literal -> INTEGER | FLOAT | STRING | BOOLEAN | IDENTIFIER;
block -> "{" declaration* "}"
if_statement -> "if" "(" equality ")" block ("else" block)?; 
while_statement -> "while" "(" equality ")" block;
loop_statement -> "loop" block;
for_statement -> "for" "(" variable_declaration "," equality "," expression_statement ")" block;
```

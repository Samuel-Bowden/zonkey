---
title: "Grammar"
weight: 16
---

This is the current grammar that I'm implementing in the parser, which is a subset of the language. It uses the notation defined in Crafting Interpreters.

Print will eventually be implemented as a function but for now I will be implementing it as part of the language grammar.

Grammar
```grammar
program -> statement*;
statement -> expression ";";
expression -> equality;
equality -> comparision (("==" | "!=") comparision)*;
comparision -> addsub ((">=" | "<=" | "<" | ">") addsub)*;
addsub -> multdiv (("-" | "+") multdiv)*;
multdiv -> literal (("/" | "*") literal)*;
literal -> INTEGER | FLOAT | STRING | "true" | "false";
```

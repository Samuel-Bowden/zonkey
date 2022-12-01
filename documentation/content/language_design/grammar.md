---
title: "Grammar"
weight: 16
---

This is the current grammar that I'm implementing in the parser, which is a subset of the language. It uses the notation defined in Crafting Interpreters.

Ambigious Grammar
```grammar
expression -> literal | unary | binary | grouping;

literal -> Integer | Float | String | "true" | "false";
grouping -> "(" expression ")";
unary -> "!" expression;
binary -> expression operator expression;
operator -> "==" | "!=" | "<" | ">" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/";
```

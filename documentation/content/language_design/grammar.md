---
title: "Grammar"
weight: 16
---

This is the current grammar that I'm implementing in the parser, which is a subset of the language. It uses the notation defined in Crafting Interpreters.

Grammar
```grammar
expression -> addsub;
addsub -> multdiv (("-" | "+") multdiv)*;
multdiv -> literal (("/" | "*") literal)*;
literal -> Integer | Float | String;
```
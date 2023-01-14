---
title: "Entry Point"
weight: 6
---

A start block is defined as so:
```
start {
  String name = "Sam";
  sayHi(name);
}

function sayHi(String name) {
  print("Hi" + name);
}
```

Functions which are called in the start block can be defined after where the start block is defined.

Having anything other than function and start declarations in the global scope will result in an error:

```start
start {
  String name = "Sam";
  sayHi(name);
}

function sayHi(String name) {
  print("Hi" + name);
}

print("Printing in the global scope!"); # Not allowed
```

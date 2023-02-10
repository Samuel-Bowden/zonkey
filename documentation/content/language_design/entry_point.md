---
title: "Entry Point"
weight: 6
---

A start block is defined as so:
```start
function sayHi(String name) {
  cli:println("Hi" + name);
}

start {
  let name = "Sam";
  sayHi(name);
}
```

Having anything other than function and start declarations in the global scope will result in an error:


```start
function sayHi(String name) {
  cli:println("Hi" + name);
}

start {
  let name = "Sam";
  sayHi(name);
}

cli:print("Printing in the global scope!"); # Not allowed
```

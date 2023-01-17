---
title: "Variables"
weight: 3
---

Variables are initialised with "let" and an identifier. Variables must be initialised by the programmer.

Types are inferred.

```zonkey
let a = 5;
let b = 0;
cli::println("Sum: " + (a+b));
```

```output
$ zonkey correct-variables.zonk
Sum: 5
```

If a programmer does not initialise a variable, the program will refuse to execute.

```zonkey
let a = 5;
let b;
cli::println("Sum: " + (a+b));
```

```output
$ zonkey incorrect-variables.zonk
Error on line 2: b has not been initialised with a value
```

Variables can be assigned with following syntax:

```zonkey
let a = 5;
a = 1;
a += 1;
a -= 1;
a *= 1;
a /= 1;
```

---
title: "Variables"
weight: 2
---

Variables are initialised with "let" and an identifier. Variables must be initialised by the programmer.

Types are inferred. Available types are explained in the next section.

```zonkey
start {
	let a = 5;
	let b = 0;
	cli:println("Sum: " + (String a+b));
}
```

```output
$ zonkey correct-variables.zonk
Sum: 5
```

If a programmer does not initialise a variable, the program will refuse to execute.

```zonkey
start {
	let a = 5;
	let b;
}
```

```output
$ zonkey incorrect-variables.zonk
(ERROR) Expected '=' after 'b' to assign a value to the declared variable.
	3 | 	let b;
        But the next token was ';'.
	3 | 	let b;
        Tip: All variables must be assigned a value when they are declared.
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

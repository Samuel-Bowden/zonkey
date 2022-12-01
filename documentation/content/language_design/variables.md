---
title: "Variables"
weight: 3
---

Variables are initialised with their type and identifier. Variables must be initialised by the programmer unless they are optional (e.g "Integer?").

```zonkey
start {
	Integer a = 5;
	Integer b = 0;
	print("Sum: " + (a+b));
}
```

```output
$ zonkey correct-variables.zonk
Sum: 5
```

If a programmer does not initialise a variable, the program will refuse to execute.

```zonkey
start {
	Integer a = 5;
	Integer b;
	print("Sum: " + (a+b));
}
```

```output
$ zonkey incorrect-variables.zonk
Error on line 2: Integer b has not been initialised with a value
```

## Casting the type of variables

The type of variables can be cast with the following syntax.

```zonkey
String test = "1";

# Converts the type of test to an Integer
test.toInteger().toValue();

test += 1;

print(str(test));
```

```zonkey
String name = "Bob";
Integer age = 32;

print(name + str(age);
```

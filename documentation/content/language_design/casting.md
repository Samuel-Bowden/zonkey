---
title: "Casting"
weight: 4
---

The type of an expression can be cast by placing the data type before an expression.

```zonkey
...
let a = Float 16 + 1;
# a = 17.0
...
```

Grouping can be used to apply the cast to the correct sub expression.
```zonkey
...
cli:println("5 * 2 = " + (String 5 * 2));
...
```

Most casts are safe and will always work. However, the following casts can have errors:
- String to Integer
- String to Float
- String to Boolean

If an error happens when trying to cast, an exception will be thrown. The program can then either catch this exception or ignore it. Ignoring it will cause the execution of the program to halt once it reaches the top of the call stack.

The syntax to catch exceptions has not been implemented yet, so it is only possible to ignore them currently.

```zonkey
start {
	# Prompt returns the string the user entered
	let input = cli:prompt("Enter an integer:");

	# Casts the string input into an integer and adds 1
	let integer = (Integer input) + 1;

	cli:println("Your integer plus one is equal to " + (String integer));
}
```

If the user inputs a valid number, the program executes fine:
```output
$ zonkey input_integer_plus_one.zonk
Enter a number: 5
Your number plus one is equal to 6
```

However, any invalid input will cause an exception:
```output
$ zonkey input_integer_plus_one.zonk
Enter a number: five
(UNCAUGHT EXCEPTION) FailedStringToIntegerCast
```

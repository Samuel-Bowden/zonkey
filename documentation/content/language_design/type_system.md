---
title: "Type System"
weight: 2
---

Typically, the languages that are taught to children in schools use dynamic typing systems, such as Python and Lua.

Dynamic typing systems allow programs to be written more quickly, however, bugs involving types are often easier for the developer to write and harder for them to diagnose.

Although Zonkey is interpreted, it will be ***statically typed*** with type inference and programs will be passed through a type checker ahead of program execution. This will make the language easier to use for developers.

The code examples below show the benefits of forcing static types in Zonkey. You may need to read sections beyond this page to understand the code that has been written.

As you can see, the error is caught much later in a dynamically typed system and the error message is a lot less helpful. Even though the developer used the function incorrectly, the error is reported inside the function, making it appear that the function is wrong. In the statically typed system that Zonkey uses, the error reported lets the developer know that they used the function incorrectly. The signature of the function also lets the developer understand the inputs and the output of the function, helping to prevent them from using the function incorrectly like this in the first place.

## Correct Program (Statically typed)

```zonkey
function add_one(Integer val) -> Integer {
	return val + 1;
}

start {
	let count = 5;
	cli::println("Initial count: " + count);
	count = add_one(count);
	cli::println("Final count: " + count);
}
```

Example Output:
```output
$ zonkey correct-static.zonk
Initial count: 5
Final count: 6
```

## Incorrect Program (Statically typed)

```zonkey
function add_one(Integer val) -> Integer {
	return val + 1;
}

start {
	let name = "Bob";
	cli::println("Initial count: " + count);
	name = add_one(name);
	cli::println("Final count: " + count);
}
```

Example Output:
```output
$ zonkey incorrect-static.zonk
Error on line 5: First parameter of function call to "add_one" must be an Integer, not a String
```

## Incorrect Program (Dynamically typed)
```zonkey
function add_one(val) {
	return val + 1;
}

start {
	name = "Bob";
	cli::println("Initial count: " + count);
	name = add_one(name);
	cli::println("Final count: " + count);
}
```

```output
$ zonkey incorrect-dynamic.zonk

Initial count: Bob
--- Execution Terminated ---
Error on line 9: Cannot add Integer to variable "val" with type String
```

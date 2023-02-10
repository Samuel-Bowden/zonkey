---
title: "Type System"
weight: 3
---

The type system of Zonkey is aimed to be easily learnable by an inexperienced developer:

- Types are simplified. For example, instead of implementing separate types for different Integer widths, only one type is provided with a long width of 64 bits.
- Type names are descriptive and kept simple.

Types:
- `Integer` (internally a 64 bit integer)
- `Float` (internally a 64 bit floating point number)
- `String`
- `Boolean`
	- Either:
		- `true`
		- `false`
- `Array<Type>` (A dynamic array which can only contain elements of Type) (To be done)
- `Iterator<Type>` (To be done)

There is not a `null` value as Zonkey is going to be a statically typed language, which will allow this to be avoided. Using null values would go against Zonkey's goal of being easy as null pointer errors are a very infamous source of developer pain.

Typically, the languages that are taught to children in schools use dynamic typing systems, such as Python and Lua.

Dynamic typing systems allow programs to be written more quickly, however, bugs involving types are often easier for the developer to write and harder for them to diagnose.

Although Zonkey is interpreted, it will be ***statically typed*** with type inference and programs will be passed through a type checker ahead of program execution. This will make the language easier to use for developers.

The code examples below show the benefits of forcing static types in Zonkey with a type checker ahead of execution. You may need to read sections beyond this page to understand the code that has been written.

As you can see, the error is found whilst executing the program in a dynamically typed system and the error message is a lot less helpful. Even though the developer used the function incorrectly, the error is reported inside the function, making it appear that the function is wrong. In the statically typed system that Zonkey uses, the error reported lets the developer know that they used the function incorrectly. The signature of the function also lets the developer understand the inputs and the output of the function, helping to prevent them from using the function incorrectly like this in the first place. As execution is prevented, it allows Zonkey programs to be more stable.

## Correct Program (Statically typed)

```zonkey
function add_one(Integer val) -> Integer {
	return val + 1;
}

start {
	let count = 5;
	cli:println("Initial count: " + (String count));
	count = add_one(count);
	cli:println("Final count: " + (String count));
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
	let count = "Bob";
	cli:println("Initial count: " + count);
	count = add_one(count);
	cli:println("Final count: " + count);
}
```

Example Output:
```output
$ zonkey incorrect-static.zonk
(ERROR) Function call add_one does not accept a value of type String for the parameter at position 0.
	8 | 	count = add_one(count);

(ERROR) Expression to assign to variable with operator '=' evaluated to the type Integer, but the variable is of type String.
	8 | 	count = add_one(count);

(ABORTING) Cannot start execution of script due to 2 error(s).
```

## Incorrect Program (Dynamically typed)
```zonkey
function add_one(val) {
	return val + 1;
}

start {
	let count = "Bob";
	cli:println("Initial count: " + count);
	name = add_one(name);
	cli:println("Final count: " + count);
}
```

```output
$ zonkey incorrect-dynamic.zonk

Initial count: Bob
--- Execution Terminated ---
Error on line 3: Cannot add Integer to variable "val" with type String
```

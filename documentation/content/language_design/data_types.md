---
title: "Data Types"
weight: 1
---

The type system of Zonkey is aimed to be easily learnable by an inexperienced developer:

- Types are simplified. For example, instead of implementing separate types for different Integer widths, only one type is provided with a long width of 64 bits.
- Type names are descriptive and kept simple.
- Data Types are all internally objects

Types:
- `Integer` (internally a 64 bit integer)
- `Float` (internally a 64 bit floating point number)
- `String`
- `Boolean`
	- Either:
		- `true`
		- `false`
- `Array<Type>` (A dynamic array which can only contain elements of Type)
- `Iterator<Type>`

There is not a `null` value as Zonkey is going to be a statically typed langauge, which will allow this to be avoided. Using null values would go against Zonkey's goal of being easy as null pointer errors are a very infamous source of developer pain.

Instead, Zonkey programs will use the `?` symbol after a data type to say that the data may not exist, a system inspired by Kotlin.

Programs then need to check whether the data exists with the `.exists()` or `.absent()` methods of the optional data type. They can then retrieve the data with `.value()` method. If this method is called and the data does not exist, the program will crash.

```zonkey
String? name;

if (name.exists())
	print(name.value());

else
	print("You must enter a name");
```

In addition, the type of the variable can be converted to its non-optional type as well with `.toValue()`. Again, this will crash the program if the data does not exist.

```zonkey
Integer? count = 10;

count.toValue();

count += 1;

print(count);
```

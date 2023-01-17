---
title: "Functions"
weight: 5
---

```zonkey
function energy(Float power, Float time) -> Float {
	return power * time;
}

start() {
	let power = 30.0;
	let time = 60.0;

	cli::println(energy(power, time));
}
```

```output
$ zonkey energy.zonk
180
```

The return type in the function signature is inspired by Rust, using its ‘->’ symbol to signify the return type. It intends to make the return type more visible to the reader of code. 

The start of the function is labeled with ‘function’ as this language is aimed towards inexperienced developers who may not be familiar with the concept of a function at first. By labelling a function clearly with its full name, they will be easily reminded of what they are reading.

The syntax for returning values follows the C-style, with the return label before the value to be returned.

Functions are also called in a C-style, with arguments to the function separated by commas and surrounded by mandatory round brackets.

Functions cannot be nested in Zonkey and must only be declared in the global scope.

---
title: "Loops"
weight: 9
---

## Infinite Loop

```zonkey
...
Boolean quit = false;

infinite loop {
	# Some operations
	...

	if (quit) {
		break;	
	}
}
...
```

In most programming languages, you would need to set up something like `while(true)` to create an infinite loop. Infinite loops are often used in programming, however they are dangerous if used incorrectly, therefore Zonkey denotes these with `infinite loop` in order for the user to understand what they are using and the dangers that come along from using them.

## For Loop

### Normal

```zonkey
start {
	for (Integer i, i < 10, i+=1) {
		print(i);
	}
}
```

### Iterator based

A common need when programming is to iterate over elements of a set or a range. Zonkey will support iterators, and this is a special type of for loop to support iterating over elements of an iterator.

```zonkey
start {
	Array<Integer> arr = [1, 2, 3];

	for (i in arr) {
		print(i);
	}
}
```

```output
$ zonkey for-range.zonk
0
1
2
3
4
5
```

The value for each iteration that is used in the for loop, "i" in this case, will have its data type inferred from the data type used for the iterator.

## While Loop

```zonkey
start {
	Integer i = 10;	

	while (i < 20) {
		print(i);
	}
}
```

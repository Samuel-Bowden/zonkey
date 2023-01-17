---
title: "Loops"
weight: 9
---

## Infinite Loop

```zonkey
...
let quit = false;

loop {
	# Some operations
	...

	if (quit) {
		break;	
	}
}
...
```

## For Loop

### Normal

```zonkey
for (let i = 0, i < 10, i+=1) {
	cli::println(i);
}
```

### Iterator based

A common need when programming is to iterate over elements of a set or a range. Zonkey will support iterators, and this is a special type of for loop to support iterating over elements of an iterator.

```zonkey
let arr = [1, 2, 3];

for (i in arr) {
	cli::println(i);
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
let i = 10;	

while (i < 20) {
	cli::println(i);
}
```

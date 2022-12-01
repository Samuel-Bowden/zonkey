---
title: "Execution"
weight: 11
---

Zonkey programs will run with the Zonkey interpreter, which will be installed on the users system.

Source code files written in the Zonkey language can optionally end with the `.zonk` file format to be recognised as a Zonkey script.

They will either be able to be executed with the Zonkey interpreter executable directly:

```output
$ zonkey count.zonk
I'm going to count up from 0 to 10:
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
```

Or be placing a unix shebang at the top of the file.

```zonkey
#!/bin/zonkey

function count(Integer start, Integer finish) -> String {
	String output = "";

	for (i in start to finish-1) {
		output += i + ", ";
	}

	output += finish;

	return output;
}

start {
	Integer start = 0;
	Integer finish = 10;

	print("I'm going to count up from 0 to 10:");
	print(count(start, finish));
}
```

```output
$ ./count
I'm going to count up from 0 to 10:
0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
```

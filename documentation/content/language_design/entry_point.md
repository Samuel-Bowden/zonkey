---
title: "Entry Point"
weight: 6
---

```zonkey
start {
    print("Hello");
}
```

Program entry is its own separate structure named `start` unlike other programming languages which use a function or method commonly labeled main. This will allow new developers to easily understand where program execution starts.

```zonkey
start(String? name) {
	if name.absent()
		print("Error: You must provide a name for me to greet you!");

	else
		print("Hello " + name.value());
}
```

```output
$ zonkey greet.zonk Sam
Hello Sam
$ zonkey greet.zonk
Error: You must provide a name for me to greet you!
```

Zonkey programs accept command line arguments and these can be accessed in `start` by declaring `String?` variables to accept them.

---
title: "Conditional Constructs"
weight: 9
---

## If-else statements

The if-else syntax is very similar to what is found in other languages. Conditional tests for each `if` statement must be surrounded by rounded brackets.

```zonkey
...
let choice = 1;

if (choice == 1) {
	cli:println("First choice");
}
else if (choice == 2) {
	cli:println("Second choice");
}
else {
	cli:println("Invalid choice");
}
...
```

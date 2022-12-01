---
title: "Arrays"
weight: 14
---

Arrays objects will be of the iterator type, therefore the methods of iterators will function on them as well.

In addition, Array objects will have the .get() method, which will return the optional value of the generic type the array is. For example, if the generic type of the array is `Integer`, then the return type of get will be `Integer?`. The user then has the option to check this value before using it, or they can use it immediately if they want to risk crashing the program.

```zonkey
...
Array<Integer> arr = [1, 2, 3];

# Prints 1
print(arr.next().value());

# Prints 2
print(arr.next().value());

# Prints 1
print(arr.get(0).value());

# Crashes program because 4 outside of bounds (no data)
print(arr.get(4).value());
...
```

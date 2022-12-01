---
title: "Classes"
weight: 10
---

```zonkey
class Dog {
    private String name;
    private Integer age;

    constructor(String name, Integer age) {
        self.name = name;
        self.age = age;
        print("I've just been created");
    }

    destructor() {
        print("I've just been destroyed");
    }

    get_age() -> Integer {
        return self.age;
    }

    get_name() -> String {
        return self.name;
    }

    set_age(Integer age) {
        self.age = age;
    }

    set_name(String name) {
        self.name = name;
    }
}

start {
    Dog jack = Dog("Jack", 7);
    print(jack.get_age());
    jack.set_age(10);
    print(jack.get_age());
}
```

```output
7
10
```

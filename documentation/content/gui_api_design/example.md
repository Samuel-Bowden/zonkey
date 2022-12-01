---
title: "Example"
weight: 1
---

Creating an GUI application that has a button which originally says "Click me", but when clicked, starts counting up every time it is clicked until it reaches 10, which then causes it to say "I've reached 10".

```zonkey
class ClickMeBtn : Button {
	private Integer count;
	
	constructor() {
		self.label = "Click Me";
		self.count = 0;
	}
	on_click {
		if (self.count + 1 = 10)
			self.label = "I've reached 10";
		else
			self.label = str(self.count);
	}
}

start {
	App app = new App();

	app.gui.set_box_layout();

	app.gui.add(ClickMeBtn());

	app.run();
}
```

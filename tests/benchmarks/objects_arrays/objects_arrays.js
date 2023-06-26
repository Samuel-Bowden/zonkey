let people = [];

for (let i = 1; i <= 10000; i++) people.push({
    id: i,
    age: 100 - i,
    weight: 80000 - i,

    age_hundred() {
        while (this.age < 100)
            this.age += 1;
        return this;
    },

    inc_weight(increment) {
        this.weight += increment;
        return this;
    },

    print() {
        return `Person ${this.id} - Age: ${this.age}, Weight: ${this.weight}`;
    },
});

for (let p of people) p.age_hundred().inc_weight(p.id);
for (let p of people) console.log(p.print());

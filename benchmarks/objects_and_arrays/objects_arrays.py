class Person:
    def __init__(self, id, age, weight):
        self.id = id
        self.age = age
        self.weight = weight
    def age_hundred(self):
        while self.age < 100: self.age += 1
        return self
    def inc_weight(self, increment):
        self.weight += increment; return self
    def print(self):
        return f"Person {self.id} - Age {self.age}: Weight {self.weight}"

people = [Person(i, 100 - i, 80000 - i) for i in range(1, 10001)]
for p in people: p.age_hundred().inc_weight(p.id)
for p in people: print(p.print())

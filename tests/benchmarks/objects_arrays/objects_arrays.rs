struct Person {
    id: i32,
    age: i32,
    weight: f64,
}

impl Person {
    fn new(id: i32, age: i32, weight: f64) -> Self {
        Self { id, age, weight }
    }

    fn age_hundred(&mut self) -> &mut Self {
        while self.age < 100 {
            self.age += 1;
        }
        self
    }

    fn inc_weight(&mut self, increment: f64) -> &mut Self {
        self.weight += increment;
        self
    }

    fn print(&self) -> String {
        format!(
            "Person {} - Age: {}, Weight: {}",
            self.id, self.age, self.weight
        )
    }
}

fn main() {
    let mut people = Vec::new();
    for i in 1..=10000 {
        let person = Person::new(i, 100 - i, 80000.0 - i as f64);
        people.push(person);
    }
    for p in &mut people {
        let id = p.id;
        p.age_hundred().inc_weight(id as f64);
    }
    for p in people {
        println!("{}", p.print());
    }
}

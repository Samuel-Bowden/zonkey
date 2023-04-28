#include <iostream>
#include <vector>
#include <sstream>
using namespace std;

class Person {
public:
    int id, age; float weight;
    Person(int id, int age, float weight) {
        this->id = id; this->age = age; this->weight = weight;
    }
    Person* age_hundred() { while (this->age < 100) this->age += 1; return this; }
    Person* inc_weight(int increment) { this->weight += increment; return this; }
    string print() {
        stringstream result;
        result << "Person " << id << " - Age: " << age << ", Weight: " << weight;
        return result.str();
    }
};
int main() {
    vector<Person> people;
    for (int i = 1; i <= 10000; i++) people.push_back(Person(i, 100 - i, 80000 - i));
    for (Person p: people) p.age_hundred()->inc_weight(p.id);
    for (Person p: people) cout << p.print() << endl;
}

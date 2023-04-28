#include <iostream>
using namespace std;

void fib(int n, int a = 0, int b = 1) {
    cout << a << endl;
    if (n > 1) fib(n-1, b, a + b);
}

int main() {
    for (int i = 0; i < 100000; i++) fib(40);
}

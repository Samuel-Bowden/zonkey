#include <iostream>
#include <sstream>
using namespace std;

void fib(stringstream* result, int n, int a = 0, int b = 1) {
    *result << a << endl;
    if (n > 1) fib(result, n-1, b, a + b);
}

int main() {
    stringstream result;

    for (int i = 0; i < 1000; i++) 
        fib(&result, 40);

    cout << result.str();
}

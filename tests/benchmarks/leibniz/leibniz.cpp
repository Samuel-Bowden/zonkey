#include <iostream>
#include <cmath>
#include <iomanip>
using namespace std;

double calc_pi(int n) {
    double result = 0.0;
    for (int i = 0; i < n; i++) {
        result += pow(-1, i) / (2 * i + 1);
    }
    return result * 4;
}

int main() {
    cout << std::setprecision(17) << calc_pi(50000000) << endl;
}

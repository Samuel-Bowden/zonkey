import math

def calc_pi(n):
    result = 0.0
    for i in range(n):
        result += math.pow(-1, i) / (2 * i + 1)
    return result * 4

print(calc_pi(50_000_000));

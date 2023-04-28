def fib(n, a=0, b = 1):
    print(a)
    if n > 1: fib(n - 1, b, a + b)

for i in range(100000):
    fib(40)

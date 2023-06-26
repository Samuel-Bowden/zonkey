function fib(n, a = 0, b = 1) {
  console.log(a);
  if (n > 1) fib(n - 1, b, a + b);
}

for (let i = 0; i < 1000; i++) fib(40);

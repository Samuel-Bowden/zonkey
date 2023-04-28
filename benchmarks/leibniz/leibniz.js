function calc_pi(n) {
    let result = 0;
    for (let i = 0; i < n; i += 1) {
        result += Math.pow(-1, i) / (2 * i + 1);
    }
    return result * 4;
}

console.log(calc_pi(50_000_000));

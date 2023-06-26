fn calc_pi(n: u32) -> f64 {
    let mut result = 0.0;
    for i in 0..n {
        result += i32::pow(-1, i) as f64 / (2 * i + 1) as f64;
    }
    result * 4.0
}

fn main() {
    println!("{}", calc_pi(50_000_000));
}

use std::io::{stdout, BufWriter, Write};

fn fib(n: u32, a: u32, b: u32, out: &mut BufWriter<impl Write>) {
    writeln!(out, "{}", a).unwrap();
    if n > 1 {
        fib(n - 1, b, a + b, out);
    }
}

fn main() {
    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    for _ in 0..1000 {
        fib(40, 0, 1, &mut out);
    }

    out.flush().unwrap();
}

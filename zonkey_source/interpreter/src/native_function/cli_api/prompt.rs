use std::io::Write;

pub fn prompt(prompt: String) -> String {
    print!("{prompt} ");

    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn prompt_int(prompt: String) -> i64 {
    print!("{prompt} ");

    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}

use std::io;

fn main() {
    let mut input=String::new();
    io::stdin().read_line(&mut input).ok();

    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|n| n.parse().ok().unwrap()).collect();
}

fn range() -> String {
    "Yes".to_string()
}

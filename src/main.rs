use std::io;

fn main() {
    let mut input=String::new();
    io::stdin().read_line(&mut input).ok();

    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|n| n.parse().ok().unwrap()).collect();
    let answer = range(nums[0], nums[1], nums[2]);
    println!("{}", answer)
}

fn range(a: i32, b: i32, c: i32) -> String {
    if a < b && b < c {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

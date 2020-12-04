use std::io;

fn main() {
    let mut input=String::new();
    io::stdin().read_line(&mut input).ok();

    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|n| n.parse().ok().unwrap()).collect();
    let answer = small_large_or_equal(nums[0], nums[1]);
    println!("{}", answer)
}


fn small_large_or_equal(a: i32, b: i32) -> String {
    if a > b {
        "a > b".to_string()
    } else if a < b {
        "a < b".to_string()
    } else {
        "a == b".to_string()
    }
}

use std::io;

fn main() {
    let mut input=String::new();
    io::stdin().read_line(&mut input).ok();

    /*
       let nums: Vec<u32> = input.trim().split_whitespace()
       .map(|n| n.parse().ok().unwrap()).collect();
       */
    let num: u32 = input.trim().parse().ok().unwrap();
    let answer = watch(num);
    println!("{}", answer);
}

fn watch(seconds: u32) -> String {
    let mut tmp_seconds = seconds;
    let hour = tmp_seconds / (60 * 60);
    tmp_seconds = tmp_seconds - (hour * (60 * 60));
    let minute = tmp_seconds / 60;
    tmp_seconds = tmp_seconds - (minute * 60);
    let seconds = tmp_seconds;
    return format!("{}:{}:{}", hour, minute, seconds);
}

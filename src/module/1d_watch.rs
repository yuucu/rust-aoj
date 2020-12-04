
fn watch(seconds: u32) -> String {
    let mut tmp_seconds = seconds;
    let hour = tmp_seconds / (60 * 60);
    tmp_seconds = tmp_seconds - (hour * (60 * 60));
    let minute = tmp_seconds / 60;
    tmp_seconds = tmp_seconds - (minute * 60);
    let seconds = tmp_seconds;
    return format!("{}:{}:{}", hour, minute, seconds);
}

fn make_readable(x: u32) -> String {
    let h = x / 3600;
    let m = (x % 3600) / 60;
    let s = x % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

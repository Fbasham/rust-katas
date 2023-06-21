fn leg_room(a: u32, b: &str) -> &'static str {
    if b.contains("00") {
        return "Jackpot!";
    }
    let n = (a as f64 * 0.55).floor();
    let x = b.chars().fold(0.0, |a, c| {
        a + (if "aeiou".contains(c) { 0.0 } else { 2.0 })
    });
    if x > 0.25 * n {
        "super comfy"
    } else if x > 0.15 * n {
        "comfortable"
    } else {
        "ouch"
    }
}

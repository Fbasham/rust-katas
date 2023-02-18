mod preloaded;
use preloaded::MORSE_CODE;

fn decode_morse(s: &str) -> String {
    let mut r = String::new();
    for e in s.split("   ") {
        for k in e.split(" ") {
            r += MORSE_CODE.get(k).unwrap_or(&"".to_string())
        }
        r += " "
    }
    r.trim().to_string()
}

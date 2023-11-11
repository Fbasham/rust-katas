mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

use regex::Regex;

pub fn decode_bits(inp: &str) -> String {
    let s = inp.trim_start_matches("0").trim_end_matches("0");
    let m = Regex::new("0+|1+")
        .unwrap()
        .find_iter(s)
        .map(|e| e.as_str().len())
        .min()
        .unwrap();
    let t = Regex::new("0+|1+")
        .unwrap()
        .find_iter(s)
        .map(|e| {
            let x = e.as_str().chars().nth(0).unwrap();
            let n = e.as_str().len() / m;
            match x {
                '0' if n == 3 => " ",
                '0' if n == 7 => "   ",
                '1' if n == 1 => ".",
                '1' if n == 3 => "-",
                _ => "",
            }
        })
        .collect::<String>();
    t
}

pub fn decode_morse(s: &str) -> String {
    let t = Regex::new("   |[.-]+| ")
        .unwrap()
        .find_iter(s)
        .map(|e| {
            let x = e.as_str();
            match x {
                " " => "",
                "   " => " ",
                _ => MORSE_CODE.get(x).unwrap(),
            }
        })
        .collect::<String>();
    t
}

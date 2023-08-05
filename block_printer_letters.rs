use itertools::Itertools;
use std::collections::*;

fn block_print(s: &str) -> String {
    let A = vec![
        vec![
            " AAA ", "A   A", "A   A", "AAAAA", "A   A", "A   A", "A   A",
        ],
        vec![
            "BBBB ", "B   B", "B   B", "BBBB ", "B   B", "B   B", "BBBB ",
        ],
        vec![
            " CCC ", "C   C", "C    ", "C    ", "C    ", "C   C", " CCC ",
        ],
        vec![
            "DDDD ", "D   D", "D   D", "D   D", "D   D", "D   D", "DDDD ",
        ],
        vec![
            "EEEEE", "E    ", "E    ", "EEEEE", "E    ", "E    ", "EEEEE",
        ],
        vec![
            "FFFFF", "F    ", "F    ", "FFFFF", "F    ", "F    ", "F    ",
        ],
        vec![
            " GGG ", "G   G", "G    ", "G GGG", "G   G", "G   G", " GGG ",
        ],
        vec![
            "H   H", "H   H", "H   H", "HHHHH", "H   H", "H   H", "H   H",
        ],
        vec![
            "IIIII", "  I  ", "  I  ", "  I  ", "  I  ", "  I  ", "IIIII",
        ],
        vec![
            "JJJJJ", "    J", "    J", "    J", "    J", "    J", "JJJJ ",
        ],
        vec![
            "K   K", "K  K ", "K K  ", "KK   ", "K K  ", "K  K ", "K   K",
        ],
        vec![
            "L    ", "L    ", "L    ", "L    ", "L    ", "L    ", "LLLLL",
        ],
        vec![
            "M   M", "MM MM", "M M M", "M   M", "M   M", "M   M", "M   M",
        ],
        vec![
            "N   N", "NN  N", "N   N", "N N N", "N   N", "N  NN", "N   N",
        ],
        vec![
            " OOO ", "O   O", "O   O", "O   O", "O   O", "O   O", " OOO ",
        ],
        vec![
            "PPPP ", "P   P", "P   P", "PPPP ", "P    ", "P    ", "P    ",
        ],
        vec![
            " QQQ ", "Q   Q", "Q   Q", "Q   Q", "Q Q Q", "Q  QQ", " QQQQ",
        ],
        vec![
            "RRRR ", "R   R", "R   R", "RRRR ", "R R  ", "R  R ", "R   R",
        ],
        vec![
            " SSS ", "S   S", "S    ", " SSS ", "    S", "S   S", " SSS ",
        ],
        vec![
            "TTTTT", "  T  ", "  T  ", "  T  ", "  T  ", "  T  ", "  T  ",
        ],
        vec![
            "U   U", "U   U", "U   U", "U   U", "U   U", "U   U", " UUU ",
        ],
        vec![
            "V   V", "V   V", "V   V", "V   V", "V   V", " V V ", "  V  ",
        ],
        vec![
            "W   W", "W   W", "W   W", "W W W", "W W W", "W W W", " W W ",
        ],
        vec![
            "X   X", "X   X", " X X ", "  X  ", " X X ", "X   X", "X   X",
        ],
        vec![
            "Y   Y", "Y   Y", " Y Y ", "  Y  ", "  Y  ", "  Y  ", "  Y  ",
        ],
        vec![
            "ZZZZZ", "    Z", "   Z ", "  Z  ", " Z   ", "Z    ", "ZZZZZ",
        ],
        vec![
            "     ", "     ", "     ", "     ", "     ", "     ", "     ",
        ],
    ];
    let mut v = (0..7).map(|_| vec![]).collect::<Vec<_>>();
    for x in s.trim().to_uppercase().chars() {
        for (i, e) in A[if x == ' ' { 26 } else { x as usize - 65 }]
            .iter()
            .cloned()
            .enumerate()
        {
            v[i].push(e);
        }
    }
    v.iter()
        .map(|e| e.join(" ").trim_end().to_string())
        .join("\n")
        .trim_end()
        .to_string()
}

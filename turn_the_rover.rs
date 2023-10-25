fn turn(c: char, t: char) -> String {
    (match (c, t) {
        ('N', 'E') => "right",
        ('N', 'W') => "left",
        ('S', 'E') => "left",
        ('S', 'W') => "right",
        ('E', 'N') => "left",
        ('E', 'S') => "right",
        ('W', 'N') => "right",
        ('W', 'S') => "left",
        _ => " ",
    })
    .to_string()
}

fn dot(n: u32, m: u32) -> String {
    let x = "+---".repeat(n as usize) + "+";
    let y = "| o ".repeat(n as usize) + "|";
    let mut v = vec![];
    for i in 0..2 * m + 1 {
        v.push(if i % 2 == 0 {
            x.to_string()
        } else {
            y.to_string()
        });
    }
    v.join("\n")
}

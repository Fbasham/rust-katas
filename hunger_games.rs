fn who_eats_who(zoo: &str) -> Vec<String> {
    let d = vec![
        ("antelope", "grass"),
        ("big-fish", "little-fish"),
        ("bug", "leaves"),
        ("bear", "big-fish"),
        ("bear", "bug"),
        ("bear", "chicken"),
        ("bear", "cow"),
        ("bear", "leaves"),
        ("bear", "sheep"),
        ("chicken", "bug"),
        ("cow", "grass"),
        ("fox", "chicken"),
        ("fox", "sheep"),
        ("giraffe", "leaves"),
        ("lion", "antelope"),
        ("lion", "cow"),
        ("panda", "leaves"),
        ("sheep", "grass"),
    ];
    let mut a = zoo.split(",").collect::<Vec<_>>();
    let mut r = vec![zoo.to_string()];
    let mut i = 0;
    while i < a.len() {
        if i > 0 && d.iter().any(|(x, y)| x == &a[i] && y == &a[i - 1]) {
            r.push(format!("{} eats {}", a[i], a[i - 1]));
            a.remove(i - 1);
            i = 0;
            continue;
        }
        if i < a.len() - 1 && d.iter().any(|(x, y)| x == &a[i] && y == &a[i + 1]) {
            r.push(format!("{} eats {}", a[i], a[i + 1]));
            a.remove(i + 1);
            i = 0;
            continue;
        }
        i += 1;
    }
    r.push(a.join(","));
    r
}
